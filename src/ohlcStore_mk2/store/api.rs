use ic_cdk_macros::{update, query};
use crate::core::{runtime::RUNTIME_STATE, stable_memory::STABLE_STATE};

use super::{price_data::fetch_token_history, types::{OHLCBucket, PriceData, OHLC}, utils::{add_cross_impl, calculate_all_price_changes, TokenPriceChange}};

// [][] --- ADMIN GATED --- [][]
#[update]
fn add_cross(cross: String) -> Option<u64> {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_admin(ic_cdk::caller().to_text())});
    add_cross_impl(cross)
}

#[update]
fn remove_cross(cross: String) -> String {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_admin(ic_cdk::caller().to_text())}); 
    STABLE_STATE.with(|s|{
        s.borrow_mut().as_mut().unwrap().remove_cross_from_store(cross)
    })
}

// [][] -- ADMIN GATED -- TEST METHODS -- [][]
// Transfer state from old store to new store
#[update]
async fn import_token_history(){
    RUNTIME_STATE.with(|s| { s.borrow().data.check_admin(ic_cdk::caller().to_text())}); 
    fetch_token_history().await;
}


// [][] --- AUTHORISED USER GATED --- [][]
#[update]
fn get_all_crosses() -> Vec<String> {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())}); 
    STABLE_STATE.with(|s|{
        let binding = s.borrow();
        binding.as_ref().unwrap().get_all_crosses()
    })
}

// #[query] // (use PriceDataResult type here)!
// fn get_all_data(cross: String) -> Option<PriceData>{
//     RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())});

// }

#[query]
fn get_m5_data(cross: String, length: u64) -> Option<Vec<OHLC>> {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
    STABLE_STATE.with(|s|{
        let binding = s.borrow();
        binding.as_ref().unwrap().get_m5_price_data(cross, length as usize)
    })
}

#[query]
fn get_m15_data(cross: String, length: u64) -> Option<Vec<OHLC>> {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
    STABLE_STATE.with(|s|{
        let binding = s.borrow();
        binding.as_ref().unwrap().get_m15_price_data(cross, length as usize)
    })
}

#[query]
fn get_h1_data(cross: String, length: u64) -> Option<Vec<OHLC>> {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
    STABLE_STATE.with(|s|{
        let binding = s.borrow();
        binding.as_ref().unwrap().get_h1_price_data(cross, length as usize)
    })
}

#[query]
fn get_d1_data(cross: String, length: u64) -> Option<Vec<OHLC>> {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
    STABLE_STATE.with(|s|{
        let binding = s.borrow();
        binding.as_ref().unwrap().get_d1_price_data(cross, length as usize)
    })
}

#[query]
fn get_w1_data(cross: String, length: u64) -> Option<Vec<OHLC>> {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
    STABLE_STATE.with(|s|{
        let binding = s.borrow();
        binding.as_ref().unwrap().get_w1_price_data(cross, length as usize)
    })
}

#[update]
fn get_all_change_data() -> Vec<TokenPriceChange> {
   RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
   calculate_all_price_changes()
}

