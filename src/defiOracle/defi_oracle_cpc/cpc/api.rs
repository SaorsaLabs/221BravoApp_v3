use defi_oracle_shared::{utils::process_mib_quotes, shared_types::{SwapPair, Marketplace, OverviewV1, TokenOverview}};
use ic_cdk_macros::{update, query};
use ic_stable_structures::MAX_PAGES;
use crate::{core::{runtime::RUNTIME_STATE, working_stats::{Metrics, count_api}}, timers::timers::schedule_stable_quote_fetch};
use super::{
    send_data::{request_authorisation_on_mib, set_mib_marketplace_impl, add_swap_pair_to_mib_canister_impl, remove_swap_pair_from_mib_canister_impl}, 
    fetch_data::{fetch_icp_usd_rate, fetch_icp_xdr_rate, fetch_quotes_from_all_mibs}, 
    constants::MAX_SPREAD_FOR_UPDATE, utils::{log, convert_quotes, AltQuotes}
};
// [][] --- ADMIN APIs --- [][]
#[update]
async fn add_mib_canister(canister: String, name: String) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.data.add_mib_canister(canister.clone(), name);
    });
    let ret = request_authorisation_on_mib(canister.clone()).await;
    return ret;
}

#[update]
fn remove_mib_canister(canister: String) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.data.remove_mib_canister(canister)
    })
}

#[query]
fn get_all_mib_canisters() -> Vec<(String, String)> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.get_mib_all_canisters()
    })
}

#[query]
fn get_all_swap_pairs() -> Vec<String> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.all_swaps.get_all_swap_pairs()
    })
}

#[update]
async fn set_mib_marketplace(mib_canister: String, marketplace: Marketplace) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    let res = set_mib_marketplace_impl(mib_canister, marketplace).await;
    return res; 
}

#[update]
async fn add_pair_to_mib_canister(mib_canister: String, swap_pair: SwapPair) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    let res = add_swap_pair_to_mib_canister_impl(mib_canister, swap_pair).await;
    return res;
}

#[update]
async fn remove_pair_from_mib_canister(mib_canister: String, swap_pair: SwapPair) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    let res = remove_swap_pair_from_mib_canister_impl(mib_canister, swap_pair).await;
    return res;
}

#[update] // usually 10.0 + USD
fn set_quote_trade_size(new_size: f64, quote_currency: AltQuotes) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().stats.set_trade_size(new_size, quote_currency)
    })
}

#[query] 
fn get_quote_trade_size() -> f64 {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().stats.get_trade_size()
    })
}

#[query]
fn get_trade_quote_currency() -> AltQuotes {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().stats.get_trade_quote_currency()
    })
}

#[query]
fn get_metrics() -> Metrics {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().stats.get_metrics()
    })
}

// [][] -- AUTHORISED APIs -- [][]
#[update] 
// update call to ensure consensus on quotes and also allow metric updates.
fn get_quote_v1(swap_pair: String, quote: AltQuotes) -> Option<OverviewV1> {
    count_api();
    // check authorised
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    // update stats
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.metrics.increment_total_api_v1()
    });
    // fetch requested data
    let quotes = RUNTIME_STATE.with(|state|{
        state.borrow().data.get_quote_v1(swap_pair)
    });
    match quotes {
        Some(v) => {
            let mut temp_vec: Vec<TokenOverview> = Vec::new();
            temp_vec.push(TokenOverview::V1(v));
            let ret = convert_quotes(temp_vec, quote);
            match ret {
                Ok(rv) => {
                    match &rv[0] {
                        TokenOverview::V1(data) => {
                            return Some(data.clone());
                        },
                        _ => { return None }
                    }
                },
                Err(e) => {
                    log(format!("Error converting quote (fn get_quote_v1) : {}", e));
                    RUNTIME_STATE.with(|s|{
                        s.borrow_mut().stats.metrics.increment_total_errors()
                    });
                    return None;
                }
            }
        },
        None => {
            return None;
        }
    }
}

#[update]
// update call to ensure consensus on quotes and also allow metric updates.
fn get_all_quotes_v1(quote: AltQuotes) -> Vec<OverviewV1> {
    count_api();
    // check authorised
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    // update stats
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.metrics.increment_total_api_v1()
    });
    // get data
    let quotes = RUNTIME_STATE.with(|state|{
        state.borrow().data.get_all_quotes()
    });
    
    let converted = convert_quotes(quotes, quote);
    match converted {
        Ok(v) => {
            let mut ret: Vec<OverviewV1> = Vec::new();
            for qte in v {
                match qte {
                    TokenOverview::V1(data) => {
                        ret.push(data);
                    },
                    _ => {}
                }
            }
            return ret;
         },
        Err(e) => { 
            RUNTIME_STATE.with(|s|{
                s.borrow_mut().stats.metrics.increment_total_errors()
            });
            let er = format!("Could not convert rates into selected quote currency - {} ", e);
            ic_cdk::trap(er.as_str())
        }
    }
}

#[query] // (main quotes time, stable quotes time (USD/XDR))
fn get_last_update_time() -> (u64, u64) {
    count_api();
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        state.borrow().stats.last_update_time  
    })
}