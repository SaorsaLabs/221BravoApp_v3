use oracle_shared_mk2::shared_types::{ChangeStatusArgs, ExchangeSnapshot, InternalRateEntry, Marketplace, StableCurrency, SwapPairDetails};
use ic_cdk_macros::{query, update};

use crate::core::runtime::RUNTIME_STATE;

use super::{fetch_data::fetch_quotes_v1, utils::log};


// [][] --- AUTHORISED APIs --- [][]
#[query]
fn get_assigned_marketplace() -> String {
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        state.borrow().data.get_assigned_marketplace().to_string()
    })
}


//[][] ---  ADMIN APIs  --- [][]
#[update]
fn set_assigned_marketplace(mkt: Marketplace) -> String {
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.update_working_stats(None, Some(mkt), None, None)
    })
}

#[update]
fn add_token_cross(details: SwapPairDetails) -> String {
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });

    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.assigned_crosses.add_swap_pair(details)
    })
}

#[update]
fn remove_token_cross(token: String) -> String {
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.assigned_crosses.remove_swap_pair(token)
    })
}

#[update]
fn set_token_cross_status(args: ChangeStatusArgs) -> String {
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    }); 
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.assigned_crosses.set_swap_status(args.token, args.status)
    })
}

#[query]
fn get_all_token_crosses() -> Vec<String> {
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        state.borrow().data.assigned_crosses.get_all_swap_pairs()
    })
}

#[query]
fn get_swap_details(swap_id: String) -> Option<SwapPairDetails> {
     // check admin
     RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        state.borrow().data.assigned_crosses.get_single_swap_pair(swap_id)
    })
}

#[update]
async fn fetch_price_data (latest_rates: Vec<InternalRateEntry>, trade_size: f64) -> Option<Vec<ExchangeSnapshot>> {
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });

    // update local rates
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.internal_rates.update_all_quotes(latest_rates.clone())
    });

    log("Local Rates updated");

    let target_exchange = RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.get_assigned_marketplace()
    });
    let quotes = fetch_quotes_v1(target_exchange, trade_size, StableCurrency::ICP).await; 
    return quotes;
}

#[update] // not on DID. 
async fn authorise_cpc() -> bool {
    let active = RUNTIME_STATE.with(|state| {
       state.borrow().data.cpc_link_active
    });
    if active == false {
        RUNTIME_STATE.with(|state| {
            let mut st = state.borrow_mut();
            st.data.add_admin(ic_cdk::caller().to_text());
            st.data.add_authorised(ic_cdk::caller().to_text());
            st.data.cpc_link_active = true;
        });
    }
    return true;
}

#[update] // not on DID. 
async fn authorise_cpc_manually(cpc: String) -> bool {
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });

    let active = RUNTIME_STATE.with(|state| {
       state.borrow().data.cpc_link_active
    });
    if active == false {
        RUNTIME_STATE.with(|state| {
            let mut st = state.borrow_mut();
            st.data.add_admin(cpc.clone());
            st.data.add_authorised(cpc.clone());
            st.data.cpc_link_active = true;
        });
    }
    return true;
}



// #[update]
// async fn test_call() -> Option<Vec<ExchangeSnapshot>> {

//     RUNTIME_STATE.with(|s|{
//         s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::ICL_ICP, quote: 0.0054f64, timestamp: 0 })
//     });

//     let quotes = fetch_quotes_v1(Marketplace::ICPSWAP, 1f64, StableCurrency::ICP).await; 
//     return quotes;
// }
