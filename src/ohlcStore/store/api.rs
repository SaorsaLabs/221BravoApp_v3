use ic_cdk_macros::{update, query};
use crate::{core::runtime::RUNTIME_STATE, timers::processing::OHLCBucket};

use super::{
    btree::{ 
        add_cross_to_store, get_all_keys, remove_cross_from_store, count_num_crosses, 
        test_populate_all, get_m5_price_data, get_m15_price_data, get_h1_price_data, get_d1_price_data, 
        get_w1_price_data, get_price_data, calculate_all_price_changes, TokenPriceChange 
    }, 
    price_data::{OHLC, PriceData}
};

// [][] --- ADMIN GATED --- [][]
#[update]
fn add_cross(cross: String) {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_admin(ic_cdk::caller().to_text())});  
    add_cross_to_store(cross);
}

#[update]
fn remove_cross(cross: String) -> String {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_admin(ic_cdk::caller().to_text())}); 
    remove_cross_from_store(cross)
}

#[query]
fn get_all_crosses() -> Vec<String> {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_admin(ic_cdk::caller().to_text())}); 
    get_all_keys()
}

#[query]
fn get_cross_count() -> u64 {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_admin(ic_cdk::caller().to_text())}); 
    count_num_crosses()
}

#[query]
fn get_processing_buckets() -> Vec<OHLCBucket> {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_admin(ic_cdk::caller().to_text())});
    RUNTIME_STATE.with(|s|{
        s.borrow().processing.clone()
    })
}

// [][] -- ADMIN GATED -- TEST METHODS -- [][]
#[update]
fn populate_m5_test(cross: String) -> String {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_admin(ic_cdk::caller().to_text())}); 
    test_populate_all(cross)
}

// [][] --- AUTHORISED USER GATED --- [][]
#[query]
fn get_all_data(cross: String) -> Option<PriceData>{
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
    get_price_data(cross)
}

#[query]
fn get_m5_data(cross: String, length: u64) -> Option<Vec<OHLC>> {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
    get_m5_price_data(cross, length as usize)
}

#[query]
fn get_m15_data(cross: String, length: u64) -> Option<Vec<OHLC>> {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
    get_m15_price_data(cross, length as usize)
}

#[query]
fn get_h1_data(cross: String, length: u64) -> Option<Vec<OHLC>> {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
    get_h1_price_data(cross, length as usize)
}

#[query]
fn get_d1_data(cross: String, length: u64) -> Option<Vec<OHLC>> {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
    get_d1_price_data(cross, length as usize)
}

#[query]
fn get_w1_data(cross: String, length: u64) -> Option<Vec<OHLC>> {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
    get_w1_price_data(cross, length as usize)
}

#[update]
fn get_all_change_data() -> Vec<TokenPriceChange> {
   RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
   calculate_all_price_changes()
}


