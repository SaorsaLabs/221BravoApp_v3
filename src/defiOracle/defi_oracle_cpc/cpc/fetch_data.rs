use candid::{CandidType, Principal};
use defi_oracle_shared::shared_types::{ExchangeSnapshot, InternalRates, InternalRateEntry};
use num_traits::Pow;
use serde::Deserialize;
use futures::future::join_all;
use crate::core::runtime::RUNTIME_STATE;
use super::{
    constants::{CYCLES_MANAGEMENT_CANISTER, EXCHANGE_RATE_CANISTER}, 
    utils::log, 
    misc_types::{GetExchangeRateRequest, Asset, AssetClass, GetExchangeRateResult}
};

#[derive(CandidType, Deserialize)]
pub struct IcpXdrConversionRate {
  xdr_permyriad_per_icp: u64,
  timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize)]
pub struct IcpXdrConversionRateResponse {
  certificate: serde_bytes::ByteBuf,
  data: IcpXdrConversionRate,
  hash_tree: serde_bytes::ByteBuf,
}

pub async fn fetch_icp_xdr_rate() -> Result<(u64, u64), String> {
    // note /10000 to get the actual ICP/ XDR rate. 

    let c1 = Principal::from_text(CYCLES_MANAGEMENT_CANISTER);
    match c1 {
        Ok(call_canister) => {
            let res: Result<(IcpXdrConversionRateResponse,), (ic_cdk::api::call::RejectionCode, String)> 
            = ic_cdk::call(call_canister, "get_icp_xdr_conversion_rate", ((), )).await;
            match res {
                Ok(res_value) => {
                    let price = res_value.0.data.xdr_permyriad_per_icp;
                    let time = ic_cdk::api::time();
                    return Ok((price, time));
                },
                Err(error) => {
                    RUNTIME_STATE.with(|s|{
                        s.borrow_mut().stats.metrics.increment_total_errors()
                    });
                    return Err(format!("Could not get XDR/ICP Rate - {:?} - {}", error.0, error.1));
                },
            }
        },
        Err(_e) => {
            RUNTIME_STATE.with(|s|{
                s.borrow_mut().stats.metrics.increment_total_errors()
            });
            return Err("Could not parse principal for cylces canister".to_string());
        },
    }
}

pub async fn fetch_icp_usd_rate() -> Result<(f64, u64), String> {
    let c1 = Principal::from_text(EXCHANGE_RATE_CANISTER);
    match c1 {
        Ok(call_canister) => {
            let cycles_amt = 1_000_000_000; // 1b cycles
            let args = GetExchangeRateRequest {
                timestamp: None,
                quote_asset:Asset { class: AssetClass::FiatCurrency, symbol: String::from("USD") }, 
                base_asset: Asset { class: AssetClass::Cryptocurrency, symbol: String::from("ICP") },
              };
            
            let res: Result<(GetExchangeRateResult,), (ic_cdk::api::call::RejectionCode, String)> 
            = ic_cdk::api::call::call_with_payment128(call_canister, "get_exchange_rate", (args, ), cycles_amt).await;
            match res {
                Ok(res_value) => {
                    match res_value.0 {
                        GetExchangeRateResult::Ok(v) => {

                            let quote = v.rate;
                            let pow = 10.pow(v.metadata.decimals);
                            let res = quote as f64 / pow as f64;
                            let time = ic_cdk::api::time();
                            return Ok((res,time));
                            //return Ok((price, time));
                        },
                        GetExchangeRateResult::Err(e) => {
                            return Err(format!("ERROR :: {:?}", e));
                        }
                    }
                },
                Err(error) => {
                    RUNTIME_STATE.with(|s|{
                        s.borrow_mut().stats.metrics.increment_total_errors()
                    });
                    return Err(format!("Could not get USD/ICP Rate - {:?} - {}", error.0, error.1));
                },
            }
        },
        Err(_e) => {
            RUNTIME_STATE.with(|s|{
                s.borrow_mut().stats.metrics.increment_total_errors()
            });
            return Err("Could not parse principal for cylces canister".to_string());
        },
    }
}

pub async fn fetch_quotes_from_all_mibs(latest_rates: Vec<InternalRateEntry>, trade_size: f64) -> Vec<ExchangeSnapshot> {
    let mib_list = RUNTIME_STATE.with(|s|{
        s.borrow().data.get_mib_all_canisters()
    });

    let mut future_vec0 = Vec::new();
    let mut data_returned: Vec<Vec<ExchangeSnapshot>> = Vec::new();
    for mib in mib_list {
        future_vec0.push(call_mib_for_quotes(mib.0, latest_rates.clone(), trade_size));
    }

    // process futures and take 'Some' values
    let future_complete = join_all(future_vec0).await;
    for res in future_complete {
        match res {
            Some(v) => {
                data_returned.push(v);
            },
            None => {}
        }
    }

    // update metrics
    RUNTIME_STATE.with(|s|{s.borrow_mut().stats.metrics.increment_snapshots_taken(data_returned.len() as u64)});

    let merged: Vec<ExchangeSnapshot> = data_returned.into_iter().flatten().collect();
    return merged;
}

async fn call_mib_for_quotes(mib_canister: String, latest_rates: Vec<InternalRateEntry>, trade_size: f64) -> Option<Vec<ExchangeSnapshot>> {
    let pr = Principal::from_text(mib_canister.clone());
    match pr {
        Ok(call_canister) => {
            let res: Result<(Option<Vec<ExchangeSnapshot>>,), (ic_cdk::api::call::RejectionCode, String)> 
            = ic_cdk::call(call_canister, "fetch_price_data", (latest_rates, trade_size, )).await;
            match res {
                Ok(v) => {
                    return v.0;
                },
                Err(e) => {
                    log(format!("Error - could not fetch quotes!, canister {} . Error {:?} - {}", mib_canister, e.0, e.1 ));
                     return None;
                },
            }
        },
        Err(_e) => {
            RUNTIME_STATE.with(|s|{
                s.borrow_mut().stats.metrics.increment_total_errors()
            });
            log("Could not parse principal for fetch_quotes_from_all_mibs");
            return None
        },
    }
}