use ic_cdk::api::call::RejectionCode;
use ic_cdk_macros::{update};
use ic_cdk_timers::TimerId;

use crate::{core::{runtime::RUNTIME_STATE, utils::log}, 
    stats::api::init_target_ledger
};
use super::{timers::{process_self_call, process_self_call2, start_processing_time_impl, schedule_data_processing}, state::TIMER_STATE};

// #[update]
// async fn test_init() -> String {
//     let args = SetTargetArgs{ 
//         target_ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
//         tx_store: "qzgyj-6iaaa-aaaak-qcqza-cai".to_string() 
//     };
//     let route = IndexerType::DfinityIcp;
//     init_target_ledger(args, route).await
// }

// #[update]
// async fn test_run(){
//     schedule_data_processing().await;
// }

// [][] -- TIMER METHODS -- [][]
#[update]
fn stop_all_timers() -> String {
    // check admin
    RUNTIME_STATE.with(|s|{s.borrow().data.check_admin(ic_cdk::caller().to_text())});

    // clear timers
    TIMER_STATE.with(|timer_ids| {
        let vec1: &mut std::cell::RefMut<Vec<TimerId>> = &mut timer_ids.borrow_mut();
        for i in vec1.iter() {
            ic_cdk_timers::clear_timer(*i);
        }
        vec1.clear();
    });

    //update working stats
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.update_timer(false)
    });   

    log("[][] ---- All timers stopped ---- [][]");
    return String::from("All timers stopped");
}

#[update]
fn start_processing_timer(secs: u64) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});
    let ret;

    // check target canister set
    let init_done = RUNTIME_STATE.with(|s|{s.borrow().data.target_ledger_locked});
    if init_done == false {
        return String::from("Cannot start timer - No target ledger set. Use init method before starting timer")
    }

    // check if running already
    let is_running = RUNTIME_STATE.with(|s|{
        s.borrow().stats.get_timer_state()
    });
     if is_running == true {
        ret = String::from("Processing timer is already running");
    } else { 
        start_processing_time_impl(secs);
        RUNTIME_STATE.with(|s|{
            s.borrow_mut().stats.update_timer(true)
        });
        ret = String::from("Processing timer has been started");
        log("[][] ---- Starting Processing Timer ---- [][]");
    }
    return ret;
}


#[update]
async fn self_call(){
    let self_id = RUNTIME_STATE.with(|s|{
        s.borrow().data.get_self_id()
    });
    let caller_string = ic_cdk::caller().to_text();
    if caller_string != self_id {
        ic_cdk::trap("This method can only be called by this canister (self-call)");
    }
    process_self_call().await
}

#[update]
async fn self_call2(){
    let self_id = RUNTIME_STATE.with(|s|{
        s.borrow().data.get_self_id()
    });
    let caller_string = ic_cdk::caller().to_text();
    if caller_string != self_id {
        ic_cdk::trap("This method can only be called by this canister (self-call)");
    }
    process_self_call2().await
}