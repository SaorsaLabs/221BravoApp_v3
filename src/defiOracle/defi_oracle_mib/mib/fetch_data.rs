#[allow(non_snake_case)]
extern crate defi_oracle_shared;
use candid::{CandidType, Principal, Error, Nat, error};
use ic_cdk::api::call::RejectionCode;
use num_traits::SaturatingMul;
use serde::Deserialize;
use num_traits::cast::ToPrimitive;
use defi_oracle_shared::utils::{nat_to_u128, nat_to_u64, round_number_to_u64};
use defi_oracle_shared::shared_types::*;
use futures::future::join_all;
use crate::core::runtime::RUNTIME_STATE;

use super::utils::{get_trade_size_exs, log};

pub async fn fetch_quotes_v1(assigned_market: Marketplace, trade_size: f64, stable_currency: StableCurrency) -> Option<Vec<ExchangeSnapshot>> {
    // for each token
    let token_swaps: Vec<SwapPairDetails> = RUNTIME_STATE.with(|s|{
        s.borrow().data.assigned_crosses.available_swaps.clone()
    });

    // one for each exchange to avoid mismatch futures warning
    let mut future_vec0 = Vec::new();
    let mut future_vec1 = Vec::new();
    let mut future_vec2 = Vec::new();

    // iterate over swap pairs
    for swap in token_swaps {
        // iterate over markets for the swap pair, only fetch data for assigned_market
        if swap.active == true {
            for mkt in swap.marketplaces {
                if mkt.marketplace == Marketplace::ICDEX && &assigned_market == &Marketplace::ICDEX {
                    future_vec0.push(fetch_icdex_quote(
                            mkt.canister_id,
                            mkt.reverse_cross,
                            swap.token0.decimals,
                            swap.token1.decimals, 
                            mkt.unit_size as u128, 
                            trade_size,
                            swap.swap_id.clone(),
                            stable_currency.clone()
                        ));
                } 
                else if mkt.marketplace == Marketplace::ICPSWAP && &assigned_market == &Marketplace::ICPSWAP {
                    
                    future_vec1.push(fetch_icpswap_quote(
                        mkt.canister_id, 
                        mkt.reverse_cross,
                        swap.token0.decimals,
                        swap.token1.decimals,
                        swap.swap_id.clone(),
                        trade_size,
                        stable_currency.clone()
                    ));
                }
                else if mkt.marketplace == Marketplace::SONIC && &assigned_market == &Marketplace::SONIC {
                     future_vec2.push(fetch_sonic_quote(
                        swap.token0.ledger.clone(), 
                        swap.token1.ledger.clone(), 
                        mkt.reverse_cross,
                        swap.token0.decimals,
                        swap.token1.decimals,
                        swap.swap_id.clone(),
                        trade_size,
                        stable_currency.clone()
                    ))
                } else {
                    // doesn't match
                }
            }
        }
    }

    // futures return
    match &assigned_market {
        &Marketplace::ICDEX => {
            let mut return_array = Vec::new();
            let future_complete = join_all(future_vec0).await;
            for res in future_complete {
                if let Some(v) = res.clone().ok() {
                    return_array.push(v);
                }
            }
            if return_array.len() > 0 {
                return Some(return_array);
            } else {
                return None
            }
        },
        &Marketplace::ICPSWAP => {
            let mut return_array = Vec::new();
            let future_complete = join_all(future_vec1).await;
            for res in future_complete {
                if let Some(v) = res.clone().ok() {
                    return_array.push(v);
                }
            }
            if return_array.len() > 0 {
                return Some(return_array);
            } else {
                return None
            }
        },
        &Marketplace::SONIC => {
            let mut return_array = Vec::new();
            let future_complete = join_all(future_vec2).await;
            for res in future_complete {
                if let Some(v) = res.clone().ok() {
                    return_array.push(v);
                }
            }
            if return_array.len() > 0 {
                return Some(return_array);
            } else {
                return None
            }
        },
        _ => {
            return None;
        },
    }
}

// IC DEX TYPES
#[derive(CandidType, Deserialize, Debug)]
pub struct icdex_PriceResponse { quantity: candid::Nat, price: candid::Nat }

#[derive(CandidType, Deserialize, Debug)]
pub struct icdex_level100_ret1 { ask: Vec<icdex_PriceResponse>, bid: Vec<icdex_PriceResponse> }

async fn fetch_icdex_quote(
    market_canister_id: String,
    reverse: bool,
    base_decimals: u32,
    quote_decimals: u32,
    unit_size: u128, 
    trade_size: f64, 
    swap_pair: SwapPair,
    stable_currency: StableCurrency) 
    -> Result< ExchangeSnapshot ,String>{
    let pr_id: Principal;
    match Principal::from_text(market_canister_id) {
        Ok(v) => { pr_id = v; },
        Err(error) => {
            return Err(String::from(format!("Error - Could not parse market_canister_id into principal. {}", error)));
        }
    }
    let res: Result<(candid::Nat, icdex_level100_ret1,), (ic_cdk::api::call::RejectionCode, String)> 
    = ic_cdk::call(pr_id, "level100", ((),)).await;
    match res {
        Ok(value) => {
            // Get Base Size
            let base_size = get_trade_size_exs(
                &swap_pair, 
                &false, 
                &base_decimals, 
                &quote_decimals, 
                &trade_size, 
                &stable_currency
            );
            match base_size {
                Some(size) => {
                    let bid = calc_price_for_quantity(value.1.bid, size.0, base_decimals, unit_size.clone());
                    let ask = calc_price_for_quantity(value.1.ask, size.0, base_decimals, unit_size.clone());
                    let bid_ask_u64: (u128, u128); // e8s
                    match (bid, ask) {
                        (Some(bid_value), Some(ask_value)) => { bid_ask_u64 = (bid_value, ask_value)},
                        _ => { return Err(String::from("Error - Cannot fill order with available liquidity"))},
                    }
                    let p = ((bid_ask_u64.0 as f64 + bid_ask_u64.1 as f64) / 2_f64) / 10f64.powi(quote_decimals as i32); 
                    let time_now = ic_cdk::api::time();
                    let spread = bid_ask_u64.1 - bid_ask_u64.0;
                    if reverse == false {
                        let ret_data = ExchangeSnapshot{
                            snapshot_time: time_now,
                            swap_pair,
                            exchange: Marketplace::ICDEX,
                            price: p,
                            bid: bid_ask_u64.0,
                            ask: bid_ask_u64.1,
                            spread_pct: (spread as f64 / bid_ask_u64.0 as f64) * 100.0,
                            liquidity: (0_u64, 0_u64),
                        };
                        return Ok(ret_data);
                    } else {
                        return Err("REVERSE NOT SUPPORTED YET".to_string());
                    }
                },
                None => {
                    log("Error - Cannot calculate base_size");
                    return Err(String::from("Error - Cannot calculate base_size"))
                }
            }
        },
        Err(error) => {
            log(format!("Canister error {:?}", error.1));
            return Err(format!("Error - {:?}, {}", error.0, error.1));
        }
    }
}

fn calc_price_for_quantity( orderbook: Vec<icdex_PriceResponse> , base_size: u128, base_decimals: u32, unit_size: u128 ) -> Option<u128> {
    let mut total_quantity_filled: u128 = 0;
    let mut total_price: u128 = 0;
    let mut price_in_lots: u128 = 0;
    let mut u128_price:u128;
    let u128_price2:u128;
    let mut q = 0_u128;
    let mut p = 0_u128;
    let base_u128 = base_size;
    for spot in orderbook {
        match nat_to_u128(spot.quantity){
            Ok(v) => {q = v} // quantity is e8s. 
            Err(e) => {log(format!("Error converting Nat to u128 (1) (calc_price_for_quantity) - {}",e))}
        }
        match nat_to_u128(spot.price){
            Ok(v) => {p = v} // is in lots (*unit size to get e8s) 
            Err(e) => {log(format!("Error converting Nat to u128 (2) (calc_price_for_quantity) - {}",e))}
        } 
        // bucket can fill entire order/ or fill remaining amount
        if q > base_u128.saturating_sub(total_quantity_filled) {
            total_quantity_filled = total_quantity_filled.saturating_add(base_u128.saturating_sub(total_quantity_filled));
            price_in_lots = total_quantity_filled.saturating_mul(p);
            total_price = total_price.saturating_add(price_in_lots); // in lots/ units
            u128_price = total_price.saturating_div(unit_size); // div by lot size
            u128_price = u128_price.saturating_mul(10_u128.pow(base_decimals));
            u128_price2 = u128_price.saturating_div(total_quantity_filled);
            
            return Some(u128_price2); // 
        } else {
            // need to build order over several price buckets. 
            total_quantity_filled = total_quantity_filled.saturating_add(q);
            price_in_lots = price_in_lots.saturating_add(q.saturating_mul(p)); // in lots/ units
        }
    }
    return None;
}

// ICP SWAP TYPES
#[derive(CandidType, Deserialize, Debug)]
pub struct ICPSwapArgs {
  operator: Principal,
  amountIn: String,
  zeroForOne: bool,
  amountOutMinimum: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum ICPSError {
  CommonError,
  InternalError(String),
  UnsupportedToken(String),
  InsufficientFunds,
}

#[derive(CandidType, Deserialize)]
pub enum ICPSResult3 { ok(candid::Nat), err(ICPSError) }

async fn fetch_icpswap_quote(
    market_canister_id: String, 
    reverse: bool, 
    base_decimals: u32,  
    quote_decimals: u32, 
    swap_pair: SwapPair, 
    trade_size: f64,
    stable_currency: StableCurrency) 
    -> Result< ExchangeSnapshot ,String> {

    let mut future_vec = Vec::new();
    
    match Principal::from_text(market_canister_id) {
        Ok(pr_id) => { 
            // reverse quote/base decimals if needed
            let rev_rev = if reverse == false { true } else { false };

            // BID
            let bid_base:u128;
            let ask_base:u128;
            let base_size = get_trade_size_exs(
                &swap_pair, 
                &reverse, 
                &base_decimals, 
                &quote_decimals, 
                &trade_size, 
                &stable_currency
            );
            // if base is going in.. this is the ASK

            match base_size {
                Some(base) => {
                    let q_args_1 = ICPSwapArgs{
                        operator: pr_id.clone(),
                        amountIn: base.0.to_string(), 
                        zeroForOne: true,  // true = (input base -> output quote)
                        amountOutMinimum: "0".to_string(),
                    };
                    bid_base = base.0;
                    // BID CALL
                    future_vec.push(icp_swap_call(q_args_1, pr_id.clone()));
                },
                None => {
                    log(format!("Base_size returned NONE - ICP Swap. Swap-Pair :: {:?}", swap_pair.clone()));
                    return Err("Could not get base size".to_string());
                }
            }

            // ASK
            let base_size2 = get_trade_size_exs(
                &swap_pair, 
                &rev_rev, 
                &base_decimals, 
                &quote_decimals, 
                &trade_size, 
                &stable_currency
            );
            match base_size2 {
                Some(base) => {
                    let q_args_1 = ICPSwapArgs {
                        operator: pr_id.clone(),
                        amountIn: base.0.to_string(), 
                        zeroForOne: false,
                        amountOutMinimum: "0".to_string(),
                    };
                    ask_base = base.0;
                    // ASK CALL
                    future_vec.push(icp_swap_call(q_args_1, pr_id));
                },
                None => {
                    log(format!("Base_size returned NONE - ICP Swap. Swap-Pair :: {:?}", swap_pair.clone()));
                    return Err("Could not get base size".to_string());
                }
            }

            // HANDLE FUTURES
            let future_complete = join_all(future_vec).await;
            let bid_price: u128;//f64;
            let ask_price: u128;//f64;
                   // BID SIDE (BUY)
                   match &future_complete[0] {
                    Ok(output) => {

                        let mut bid_exs;
                        if reverse == false {
                            //((out/in)*POW(t0_dec - t1_dec))*t1_eXs
                            bid_exs = ((*output as f64 / bid_base as f64) * 
                            10f64.powi(base_decimals as i32 - quote_decimals as i32)) *
                            10f64.powi(quote_decimals as i32); // X / ICP
                        } else {
                            //((out/in)*POW(t1_dec - t0_dec))*t0_eXs
                            bid_exs = ((*output as f64 / bid_base as f64) * 
                            10f64.powi(quote_decimals as i32 - base_decimals as i32)) *
                            10f64.powi(base_decimals as i32); // = (number of X for 1 icp in X decimals .. icp/ X )
                            // reverse to = 1 of X / ICP 
                            bid_exs = (10f64.powi(quote_decimals as i32) / bid_exs) * 10f64.powi(base_decimals as i32);
                        }

                        // catch underflows
                        if bid_exs > 0.0 {
                            bid_price = bid_exs as u128;
                        } else {
                            bid_price = 0_u128;
                        }
                    },
                    Err(e) => {
                        return Err(e.clone())
                    }
                }
    
                // ASK SIDE (SELL)
                match &future_complete[1] {
                    Ok(output) => {// QUOTE / BASE 
 
                    // ((1 / (out/in)*POW(t0_dec - t1_dec)) * t1_eXs
                    let mut ask_exs; 
                    if reverse == false {
                        ask_exs = ((1.0 / (*output as f64 / ask_base as f64)) *
                        10f64.powi(base_decimals as i32 - quote_decimals as i32)) *
                        10f64.powi(quote_decimals as i32);
                    } else {
                        ask_exs = ((1.0 / (*output as f64 / ask_base as f64)) *
                        10f64.powi(quote_decimals as i32 - base_decimals as i32)) *
                        10f64.powi(base_decimals as i32); // = (number of X for 1 icp in X decimals .. icp/ X )
                        // reverse to = 1 of X / ICP 
                        ask_exs = (10f64.powi(quote_decimals as i32) / ask_exs) * 10f64.powi(base_decimals as i32);
                    }

                    // catch underflows
                    if ask_exs > 0.0 {
                        ask_price = ask_exs as u128;
                    } else {
                        ask_price = 0_u128;
                    }

                    },
                    Err(e) => {
                        return Err(e.clone())
                    }
                }
    
                // COMBINE AND RETURN 
                let time_now = ic_cdk::api::time();
                    let data = calc_price_bid_ask(
                        base_decimals, 
                        quote_decimals, 
                        bid_price,
                        ask_price,
                        reverse
                        );

                    
                return Ok(ExchangeSnapshot{
                    snapshot_time: time_now,
                    swap_pair,
                    exchange: Marketplace::ICPSWAP,
                    price: data.0,
                    bid: data.1,
                    ask: data.2,
                    spread_pct: data.3,
                    liquidity: (0_u64, 0_u64),
                });
            },

        Err(error) => {
            return Err(String::from(format!("Error - Could not parse market_canister_id into principal. {}", error)));
        }
    }
}

async fn icp_swap_call(args: ICPSwapArgs, canister: Principal) -> Result<u128, String> {
    let res: Result<(ICPSResult3,), (ic_cdk::api::call::RejectionCode, String)> 
    = ic_cdk::call(canister, "quote", (args,)).await;

    match res {
        Ok(value) => {
            match value.0 {
                ICPSResult3::ok(price) => {
                    match nat_to_u128(price){
                        Ok(v) => {
                            return Ok(v);
                        } 
                        Err(e) => {
                            let err = format!("Error converting Nat to u128 (1) (icp_swap_call) - {}",e);
                            log(err.clone());
                            return Err(err);
                        }
                    }
                    
                },
                ICPSResult3::err(error2) => {
                      return Err(String::from(format!("Call complete but ICP Swap canister returned an error. {:?}", error2)));
                }
            }
        },
        Err(error) => {
              return Err(String::from(format!("Error - Could not fetch data from ICP Swap. {:?}", error)));
        }
    }
}

//[][] --- TYPES FOR SONIC --- [][]
#[derive(CandidType, Deserialize)]
pub struct PairInfoExt {
  id: Option<String>,
  price0CumulativeLast: candid::Nat,
  creator: Principal,
  reserve0: candid::Nat,
  reserve1: candid::Nat,
  lptoken: String,
  totalSupply: candid::Nat,
  token0: String,
  token1: String,
  price1CumulativeLast: candid::Nat,
  kLast: candid::Nat,
  blockTimestampLast: candid::Int,
}

async fn fetch_sonic_quote(
    token0_principal: String, 
    token1_principal: String, 
    reverse: bool, 
    base_decimals: u32,
    quote_decimals: u32,
    swap_pair: SwapPair,
    trade_size: f64,
    stable_currency: StableCurrency
    ) -> Result< ExchangeSnapshot ,String> {
    // (principal, principal) -> (opt PairInfoExt) query;
    let can = Principal::from_text("3xwpq-ziaaa-aaaah-qcn4a-cai");
    let pr1 = Principal::from_text(token0_principal);
    let pr2 = Principal::from_text(token1_principal);
    match (pr1, pr2, can) {
        (Ok(v1), Ok(v2), Ok(c1)) => {
            let res: Result<(Option<PairInfoExt>,), (ic_cdk::api::call::RejectionCode, String)> 
                 = ic_cdk::call(c1, "getPair", (v1, v2, )).await;

            match res {
                Ok(rv) => {
                    match rv.0 {
                        Some(ret_value) => {
                            // to calc ask side

                            // data returned from sonic canister
                            let reserve0;
                            match nat_to_u128(ret_value.reserve0) {
                                Ok(v) => { reserve0 = v},
                                Err(e) => {
                                    let error = format!("Cannot convert Nat to u128 (1) (fetch_sonic_quote) - {}", e);
                                    log(error.clone());
                                    return Err(error)
                                }
                            }
                
                            let reserve1;
                            match nat_to_u128(ret_value.reserve1) {
                                Ok(v) => { reserve1 = v},
                                Err(e) => {
                                    let error = format!("Cannot convert Nat to u128 (1) (fetch_sonic_quote) - {}", e);
                                    log(error.clone());
                                    return Err(error)
                                }
                            }
                            let quote_tuple;

                            // TX Size
                            let base_size = get_trade_size_exs(
                                &swap_pair, 
                                &reverse, 
                                &base_decimals, 
                                &quote_decimals, 
                                &trade_size, 
                                &stable_currency
                            );
                            match base_size {
                                Some(bid_size) => {
                                    quote_tuple = calc_sonic_price(
                                        bid_size.0 as u128, 
                                        bid_size.1 as u128, 
                                        base_decimals, 
                                        quote_decimals, 
                                        reserve0, 
                                        reserve1,
                                        reverse
                                    );
                                },
                                None => {
                                    log(format!("Base_size returned NONE - Sonic. Swap-Pair :: {:?}", swap_pair.clone()));
                                    return Err("Could not get base size".to_string());
                                }
                            }
                            
                            let data = calc_price_bid_ask(
                                base_decimals,
                                quote_decimals,
                                quote_tuple.0,
                                quote_tuple.1,
                                false
                            );

                            let time_now = ic_cdk::api::time();
                            return Ok(ExchangeSnapshot{
                                snapshot_time: time_now,
                                swap_pair,
                                exchange: Marketplace::SONIC,
                                price: data.0,
                                bid: data.1,
                                ask: data.2,
                                spread_pct: data.3,
                                liquidity: (0_u64, 0_u64),
                            });
                        },
                        None => {
                            return Err(format!("No data for that pair"));
                        }
                    }
                },
                Err(error) => {
                    return Err(format!("Error getting data from Sonic - {:?} : {}", error.0, error.1));
                }
            }
        },
        _  => {
            return Err("Could not parse input principals from text".to_string());
        }
    }
}

fn calc_sonic_price(base_size: u128, quote_size: u128, base_decimals: u32, quote_decimals: u32, reserve0: u128, reserve1: u128, reverse: bool) -> (u128, u128){

    let r0: u128 = reserve0; 
    let r1: u128 = reserve1; 
    let amount_in: u128 = base_size.clone(); 
    let amount_in_qte = quote_size.clone();
    let mut numerator;
    let mut denominator;
    let mut result;
    let mut cross;

    if reverse == false {
        // ASK 
        numerator = (amount_in_qte*997) * r0;
        denominator = (r1*1000) + (amount_in_qte*997);
        result = numerator as f64 / denominator as f64;
        cross = (result as f64 / amount_in_qte as f64) * 10f64.powi(quote_decimals as i32 - base_decimals as i32);
        let ask_hr = 1.0/ cross;
        let ask_exs = (ask_hr * 10f64.powi(quote_decimals as i32)) as u128;

        // BID
        numerator = (amount_in*997) * r1;
        denominator = (r0*1000) + (amount_in*997);
        result = numerator as f64 / denominator as f64;
        cross = (result as f64 / amount_in as f64) * 10f64.powi(base_decimals as i32 - quote_decimals as i32);
        let bid_hr = cross;
        let bid_exs = (bid_hr * 10f64.powi(quote_decimals as i32)) as u128;

        return (bid_exs, ask_exs); // regular and reverse
    } else {
        // ASK (Is actually the bid!) 
        numerator = (amount_in_qte*997) * r0;
        denominator = (r1*1000) + (amount_in_qte*997);
        result = numerator as f64 / denominator as f64;
        cross = (result as f64 / amount_in_qte as f64) * 10f64.powi(base_decimals as i32 - quote_decimals as i32);
        let ask_exs = (cross * 10f64.powi(quote_decimals as i32)) as u128;

        // BID (Is actually the Ask!) 
        numerator = (amount_in*997) * r1;
        denominator = (r0*1000) + (amount_in*997);
        result = numerator as f64 / denominator as f64;
        cross = (result as f64 / amount_in as f64) * 10f64.powi(quote_decimals as i32 - base_decimals as i32);
        let bid_hr = 1.0/ cross;
        let bid_exs = (bid_hr * 10f64.powi(quote_decimals as i32)) as u128;

        return (ask_exs, bid_exs); // regular and reverse (reversed on purpose!)
    }
}

fn calc_price_bid_ask(_token0_decimals: u32, token1_decimals: u32, bid_exs: u128, ask_exs: u128, reverse: bool) -> (f64, u128, u128, f64) {
    //let t0_power = 10u128.pow(token0_decimals);
    let t1_power = 10u128.pow(token1_decimals);
    let med_price_exs = (bid_exs + ask_exs) / 2;
    let spread: f64 = ((bid_exs as f64 - ask_exs as f64).abs() / bid_exs as f64) * 100.0;

    // used by sonic and ICP Swap
    let human_readable_price = med_price_exs as f64 / t1_power as f64;
    if reverse == false {
        return (human_readable_price, bid_exs, ask_exs, spread);
    } else {
        return (human_readable_price, ask_exs, bid_exs, spread);
    }
}