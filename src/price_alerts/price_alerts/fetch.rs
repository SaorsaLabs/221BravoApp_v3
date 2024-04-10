use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::core::runtime::RUNTIME_STATE;
const EXCHANGE_RATE_CANISTER: &str = "uf6dk-hyaaa-aaaaq-qaaaq-cai";

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
                            let pow = 10usize.pow(v.metadata.decimals);
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

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GetExchangeRateRequest {
    pub timestamp: Option<u64>,
    pub quote_asset: Asset,
    pub base_asset: Asset,
  }

  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub struct Asset { pub class: AssetClass, pub symbol: String }

  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub enum AssetClass { Cryptocurrency, FiatCurrency }

  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub enum GetExchangeRateResult { Ok(ExchangeRate), Err(ExchangeRateError) }
  
  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub struct ExchangeRateMetadata {
    pub decimals: u32,
    pub forex_timestamp: Option<u64>,
    pub quote_asset_num_received_rates: u64,
    pub base_asset_num_received_rates: u64,
    pub base_asset_num_queried_sources: u64,
    pub standard_deviation: u64,
    pub quote_asset_num_queried_sources: u64,
  }
  
  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub struct ExchangeRate {
    pub metadata: ExchangeRateMetadata,
    pub rate: u64,
    pub timestamp: u64,
    pub quote_asset: Asset,
    pub base_asset: Asset,
  }
  
  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub enum ExchangeRateError {
    AnonymousPrincipalNotAllowed,
    CryptoQuoteAssetNotFound,
    FailedToAcceptCycles,
    ForexBaseAssetNotFound,
    CryptoBaseAssetNotFound,
    StablecoinRateTooFewRates,
    ForexAssetsNotFound,
    InconsistentRatesReceived,
    RateLimited,
    StablecoinRateZeroRate,
    Other{ code: u32, description: String },
    ForexInvalidTimestamp,
    NotEnoughCycles,
    ForexQuoteAssetNotFound,
    StablecoinRateNotFound,
    Pending,
  }