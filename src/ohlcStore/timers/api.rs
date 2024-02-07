use defi_oracle_shared::shared_types::OverviewV1;
use ic_cdk_macros::update;
use ic_cdk_timers::TimerId;
use crate::{
    core::{runtime::RUNTIME_STATE, utils::log}, store::btree::test_get_values
    };
use super::{processing::update_prices_in_processing_bucket, state::TIMER_STATE, timers::{schedule_data_processing, schedule_data_processing_temp, test_schedule_data_processing}};
use super::timers::start_processing_timer;

// [][] -- TIMER METHODS -- [][]
#[update]
fn stop_all_timers() -> String {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});

    // clear timers
    TIMER_STATE.with(|timer_ids| {
        let vec1: &mut std::cell::RefMut<Vec<TimerId>> = &mut timer_ids.borrow_mut();
        for i in vec1.iter() {
            ic_cdk_timers::clear_timer(*i);
        }
        vec1.clear();
    });

    // update working stats
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.update_timer(false)
    });   

    log("[][] ---- All timers stopped ---- [][]");
    return String::from("All timers stopped");
}

#[update]
fn start_quotes_timer(secs: u64, oracle_canister: String) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});

    let ret;
    // check if running already
    let is_running = RUNTIME_STATE.with(|s|{
        s.borrow().stats.get_timer_state()
    });
     if is_running == true {
        ret = String::from("Main quotes timer is already running");
    } else {
        RUNTIME_STATE.with(|s|{
            s.borrow_mut().data.set_oracle_id(oracle_canister)
        });

        start_processing_timer(secs);

        RUNTIME_STATE.with(|s|{
            s.borrow_mut().stats.update_timer(true)
        });
        ret = String::from("Main quotes has been started");
        log("[][] ---- Starting Main quotes Timer ---- [][]");
    }
    return ret;
}

#[update]
fn self_call(quote_vec: Vec<OverviewV1>) {
    let self_id = RUNTIME_STATE.with(|s|{
        s.borrow().data.get_self_id()
    });
    let caller_string = ic_cdk::caller().to_text();
    if caller_string != self_id {
        ic_cdk::trap("This method can only be called by this canister (self-call)");
    }
    update_prices_in_processing_bucket(quote_vec);
}