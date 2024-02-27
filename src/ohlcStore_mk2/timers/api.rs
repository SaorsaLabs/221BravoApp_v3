use std::fmt::format;

use defi_oracle_shared::shared_types::OverviewV1;
use ic_cdk::update;
use ic_cdk_timers::TimerId;
use crate::core::{runtime::RUNTIME_STATE, stable_memory::STABLE_STATE, utils::log};
use super::{state::TIMER_STATE, timers::{schedule_data_processing, start_quotes_timer_impl, stop_all_timers_impl}};

// [][] -- ADMIN GATED -- [][]
#[update]
fn stop_all_timers() -> String {
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});
    stop_all_timers_impl()
}

#[update]
fn start_quotes_timer(secs: u64, oracle_canister: String) -> String {
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});
    start_quotes_timer_impl(secs, oracle_canister)
}

#[update]
fn process_chunk(quote_vec: Vec<OverviewV1>) {
    // gate - only possible to call by self
    let self_id = RUNTIME_STATE.with(|s|{
        s.borrow().data.get_self_id()
    });
    let caller_string = ic_cdk::caller().to_text();
    if caller_string != self_id {
        ic_cdk::trap("This method can only be called by this canister (self-call)");
    }
    STABLE_STATE.with(|s|{
        s.borrow_mut().as_mut().unwrap()
        .update_prices_in_processing_bucket(quote_vec)
    });
    let inst = ic_cdk::api::instruction_counter();
    log(format!("Process Chunk - Instructions :: {}", inst));
}

// #[update]
// fn self_call(quote_vec: Vec<OverviewV1>) {
//     let self_id = RUNTIME_STATE.with(|s|{
//         s.borrow().data.get_self_id()
//     });
//     let caller_string = ic_cdk::caller().to_text();
//     if caller_string != self_id {
//         ic_cdk::trap("This method can only be called by this canister (self-call)");
//     }
//     update_prices_in_processing_bucket(quote_vec);
// }


// [][] --- TESTING ONLY --- [][]
#[update]
async fn test_call_timer(){ 
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});
    
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().data.set_oracle_id(String::from("vq2tb-uiaaa-aaaak-qcqfa-cai"));
    });
    schedule_data_processing().await;
}

// //#[update]
// pub fn test_get_value_len(cross: String){
//     test_get_values(cross);
// }