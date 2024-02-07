use ic_cdk_macros::update;
use ic_cdk_timers::TimerId;
use crate::core::{runtime::RUNTIME_STATE, utils::log};
use super::{state::TIMER_STATE, timers::{start_processing_timer, start_icrc_stable_timer, start_top_holder_timer}};

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
        s.borrow_mut().stats.timer_active = false;
    });   

    log("[][] ---- All timers stopped ---- [][]");
    return String::from("All timers stopped");
}

#[update] // 5min, 24hours, 12 hours.
async fn start_quotes_timer(secs: u64, supply_secs: u64, top_holder_secs: u64) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});

    let ret;
    // check if running already
    let is_running = RUNTIME_STATE.with(|s|{
        s.borrow().stats.timer_active
    });
     if is_running == true {
        ret = String::from("Timers are already running");
    } else {
        
        start_processing_timer(secs);
        start_icrc_stable_timer(supply_secs);
        start_top_holder_timer(top_holder_secs);

        RUNTIME_STATE.with(|s|{
            s.borrow_mut().stats.timer_active = true;
        });
        ret = String::from("Processing and supply timers have been started");
        log("[][] ---- Starting All Timers ---- [][]");
    }
     return ret;
}

