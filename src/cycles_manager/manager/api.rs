// add canister

use ic_cdk_macros::{update, query};
use crate::core::{runtime::RUNTIME_STATE, utils::canister_call};

#[update]
async fn add_canister(canister: String) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});
    // check possible to deposit
    let deposit_call: Result<((),), (ic_cdk::api::call::RejectionCode, String)> 
    = canister_call(canister.as_str(), "deposit_cycles", (), Some(1000)).await;
    match deposit_call {
        Ok(_) => { 
            RUNTIME_STATE.with(|s| s.borrow_mut().data.add_canister(canister))
        },
        Err(e) => { 
            let error = format!("Test cycle deposit failed :: {:?}, {}", e.0, e.1);
            ic_cdk::trap(error.as_str());    
        }
    }    
}

#[update]
fn remove_canister(canister: String) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});
    RUNTIME_STATE.with(|s| s.borrow_mut().data.remove_canister(canister))
}

// cycles topup level
#[query]
fn get_cycles_topup_level() -> u64 {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});
    RUNTIME_STATE.with(|s| s.borrow().data.get_topup_level())    
}

#[update]
fn set_cycles_topup_level(level: u64){
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});
    RUNTIME_STATE.with(|s| s.borrow_mut().data.set_topup_level(level))    
}

// cycles topup amount
#[query]
fn get_cycles_topup_amount() -> u64 {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});
    RUNTIME_STATE.with(|s| s.borrow().data.get_topup_amount())    
}

#[update]
fn set_cycles_topup_amount(level: u64){
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});
    RUNTIME_STATE.with(|s| s.borrow_mut().data.set_topup_amount(level))    
}

// check days
#[query]
fn get_check_days() -> u8 {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});
    RUNTIME_STATE.with(|s| s.borrow().data.get_check_days())    
}

#[update]
fn set_check_days(days: u8){
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});
    RUNTIME_STATE.with(|s| s.borrow_mut().data.set_check_days(days))    
}

#[query]
fn get_checking_list() -> Vec<String> {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});
    RUNTIME_STATE.with(|s| s.borrow().data.get_canister_checklist())
}
