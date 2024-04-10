use oracle_shared_mk2::shared_types::{InternalRateEntry, InternalRates, Marketplace, MarketplaceDetails, OverviewV1, SwapPairDetails, TokenOverview};
use ic_cdk_macros::{update, query};

use crate::{core::{runtime::RUNTIME_STATE, working_stats::{count_api, Metrics}}, timers::timers::{schedule_data_processing, schedule_stable_quote_fetch}};
use super::{
    misc_types::{AddSwapInput, MIBVersion, MIBV1}, 
    send_data::{
        add_swap_pair_to_mib_canister_impl, 
        remove_swap_pair_from_mib_canister_impl, 
        request_authorisation_on_mib, 
        set_mib_marketplace_impl, 
        update_pair_status_impl}, 
        utils::{convert_quotes, log, AltQuotes}
};
// [][] --- ADMIN APIs --- [][]

// MIB CANISTER ADMIN
#[update]
async fn add_mib_canister(canister: String, name: String, marketplace: Marketplace) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    let v1 = MIBV1{ 
        name, 
        canister: canister.clone(), 
        crosses: Vec::new(), 
        assigned_marketplace: marketplace.clone(),
    };
    let mib: MIBVersion = MIBVersion::V1(v1);

    RUNTIME_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.data.mib_manager.add_mib(mib)
    });
    let req = request_authorisation_on_mib(canister.clone()).await;
    let set = set_mib_marketplace(canister.clone(), marketplace).await;
    let ret = format!("{}. {}", req, set);
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
        s.data.mib_manager.remove_mib(canister)
    })
}

#[query]
fn get_all_mib_canisters() -> Vec<MIBV1> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.mib_manager.get_all_v1_mib_canisters_raw()
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
async fn add_pair_to_mib_canister(mib_canister: String, swap_pair: String) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    let res = add_swap_pair_to_mib_canister_impl(mib_canister, swap_pair).await;
    return res;
}

#[update]
async fn remove_pair_from_mib_canister(mib_canister: String, swap_pair: String) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    let res = remove_swap_pair_from_mib_canister_impl(mib_canister, swap_pair).await;
    return res;
}

#[update]
async fn update_pair_status(swap_pair: String, marketplace: Option<Marketplace>, status: bool) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    update_pair_status_impl(swap_pair, marketplace, status).await
}


// SWAP PAIR ADMIN      
#[update]               // swap_id as XXX/YYY
fn add_swap_to_oracle(swap: AddSwapInput) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    let new_swap: SwapPairDetails = SwapPairDetails{
        swap_id: swap.swap_id.clone(),
        token0: swap.token0,
        token1: swap.token1,
        marketplaces: Vec::new(),
        swap_type: swap.swap_type,
        active: false,
    };
    // init internal quote
    let time = ic_cdk::api::time();
    RUNTIME_STATE.with(|state|{
        state.borrow_mut().data.internal_rates.add_swap_pair_to_vec(
            InternalRateEntry { swap_pair: swap.swap_id, quote: swap.init_quote, timestamp: time }
        );
    });
    // add swap pair
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.all_swaps.add_swap_pair(new_swap)
    })
}

#[update]                 // swap_id as XXX/YYY
fn remove_swap_from_oracle(swap_id: String) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    // remove from internal rates
    RUNTIME_STATE.with(|state|{
        state.borrow_mut().data.internal_rates.remove_swap_pair_from_vec(swap_id.clone());
    });
    // remove from all swaps
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.all_swaps.remove_swap_pair(swap_id)
    })
}

// #[update]
// fn set_swap_status(swap_id: String, status: bool) -> String {
//     // check admin
//     RUNTIME_STATE.with(|state| {
//         state.borrow().data.check_admin(ic_cdk::caller().to_text());
//     });
//     RUNTIME_STATE.with(|state| {
//         state.borrow_mut().data.all_swaps.set_swap_status(swap_id, status)
//     })
// }

#[update]
fn add_marketplace_to_swap(swap_id: String, marketplace: MarketplaceDetails) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.all_swaps.add_marketplace(swap_id, marketplace)
    })
}

#[update]
fn remove_marketplace_from_swap(swap_id: String, marketplace: Marketplace) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.all_swaps.remove_marketplace(swap_id, marketplace)
    })
}

// #[update]
// fn set_marketplace_status(swap_id: String, marketplace: Marketplace, status: bool) -> String {
//     // check admin
//     RUNTIME_STATE.with(|state| {
//         state.borrow().data.check_admin(ic_cdk::caller().to_text());
//     });
//     RUNTIME_STATE.with(|state| {
//         state.borrow_mut().data.all_swaps.set_marketplace_status(swap_id, marketplace, status)
//     })
// }

#[query]
fn get_all_swap_marketplaces(swap_id: String) -> Option<Vec<MarketplaceDetails>> {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        state.borrow().data.all_swaps.get_all_swap_marketplaces(swap_id)
    })
}

#[query]
fn get_swap_details(swap_id: String) -> Option<SwapPairDetails> {
     // check admin
     RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        state.borrow().data.all_swaps.get_single_swap_pair(swap_id)
    })
}

// QUOTE FETCH ADMIN 
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
#[query]
fn get_all_swap_pairs() -> Vec<String> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.all_swaps.get_all_swap_pairs()
    })
}

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
    //log(format!("All quotes :: {:?}", quotes.clone()));
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


// [][] --- TESTING ONLY !! --- [][]

#[update]
async fn fetch_stable_quotes(){
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    schedule_stable_quote_fetch().await;
}

#[update]
async fn fetch_mib_quotes(){
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    schedule_data_processing().await;
}

#[query]
fn get_internal_rates() -> InternalRates {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
     RUNTIME_STATE.with(|s|{
        s.borrow().data.internal_rates.clone()
    })
}

#[update]
fn update_internal_rate(swap_pair: String, quote: f64, timestamp: u64){
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().data.internal_rates.update_single_quote(&swap_pair, quote, timestamp)
    });
}

#[query]
fn does_internal_rate_exist(swap_pair: String) -> bool {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().data.internal_rates.does_swap_exist(swap_pair)
    })
}



