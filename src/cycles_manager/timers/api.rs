use ic_cdk_macros::update;
use ic_cdk_timers::TimerId;
use crate::{
    core::{runtime::RUNTIME_STATE, utils::log, constants::CHECK_DAYS}, manager::check_cycles::check_cycles_impl
    };
use super::{state::TIMER_STATE, utils::next_midnight_time, timers::schedule_first_call};
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
fn start_cycles_timer() -> String {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});

    let ret;
    // check if running already
    let is_running = RUNTIME_STATE.with(|s|{
        s.borrow().stats.get_timer_state()
    });
     if is_running == true {
        ret = String::from("Timer is already running");
    } else {
        
        // get seconds till midnight 
        let time_now = ic_cdk::api::time();
        let next_midnight = next_midnight_time();
        let secs_till_midnight = (next_midnight - time_now)/1000000000; // milli to seconds
        
        schedule_first_call(secs_till_midnight);

        RUNTIME_STATE.with(|s|{
            s.borrow_mut().stats.update_timer(true)
        });
        ret = String::from("Timer has been started");
        log("[][] ---- Starting Main quotes Timer ---- [][]");
    }
    return ret;
}

// 24 hr timer
#[update]
async fn self_call0(){
    // only this canister can call this method
    let caller  = ic_cdk::caller().to_text();
    let self_id = RUNTIME_STATE.with(|state| {state.borrow().data.get_self_id()});
    if caller != self_id { ic_cdk::trap("NOT AUTHORISED") }
    
    // check cycles 
    let check_list = RUNTIME_STATE.with(|s|s.borrow().data.get_canister_checklist());
    check_cycles_impl(check_list).await;

    // clear timers (self_call0 no longer needed)
    TIMER_STATE.with(|timer_ids| {
        let vec1: &mut std::cell::RefMut<Vec<TimerId>> = &mut timer_ids.borrow_mut();
        for i in vec1.iter() {
            ic_cdk_timers::clear_timer(*i);
        }
        vec1.clear();
    });

    // set X day timer
    let check_days = RUNTIME_STATE.with(|s| s.borrow().data.get_check_days());
    let time_secs = check_days as u64 * 86400; 
    start_processing_timer(time_secs);
      
    // update last update time 
    let time_now = ic_cdk::api::time();
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.last_update_time = time_now;
    });
}

#[update]
async fn manual_check_all(){
        // check cycles 
        let check_list = RUNTIME_STATE.with(|s|s.borrow().data.get_canister_checklist());
        check_cycles_impl(check_list).await;
    
        // update last update time 
        let time_now = ic_cdk::api::time();
        RUNTIME_STATE.with(|s|{
            s.borrow_mut().stats.last_update_time = time_now;
        });
}

