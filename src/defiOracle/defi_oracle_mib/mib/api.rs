use candid::de;
use defi_oracle_shared::shared_types::{Marketplace, SwapPairDetails, SwapPair, InternalRateEntry, ExchangeSnapshot, StableCurrency};
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
fn remove_token_cross(token: SwapPair) -> String {
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.assigned_crosses.remove_swap_pair(token)
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

#[update]
async fn fetch_price_data (latest_rates: Vec<InternalRateEntry>, trade_size: f64) -> Option<Vec<ExchangeSnapshot>> {
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });

    // update local rates
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.internal_rates.update_all_quotes(latest_rates)
    });

    log("Local Rates updated");

    let target_exchange = RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.get_assigned_marketplace()
    });

    let quotes = fetch_quotes_v1(target_exchange, trade_size, StableCurrency::ICP).await; 

    return quotes;
}

#[update] // can only be called once 
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
