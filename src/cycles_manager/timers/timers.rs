use std::time::Duration;

use crate::{
    core::{runtime::RUNTIME_STATE, utils::{canister_call, log}}, manager::check_cycles::check_cycles_impl,
};
use super::state::TIMER_STATE;

pub fn start_processing_timer(secs: u64) {
    let secs = Duration::from_secs(secs);
    let timer_id = ic_cdk_timers::set_timer_interval(secs, ||
        ic_cdk::spawn(schedule_cycles_check())
    );
    TIMER_STATE.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
}

// Fetch Main quotes
async fn schedule_cycles_check(){
    // check cycles 
    let check_list = RUNTIME_STATE.with(|s|s.borrow().data.get_canister_checklist());
    check_cycles_impl(check_list).await;

    // update last update time 
    let time_now = ic_cdk::api::time();
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.last_update_time = time_now;
    });
}

pub fn schedule_first_call(secs: u64) {
    let secs = Duration::from_secs(secs);
    let timer_id = ic_cdk_timers::set_timer_interval(secs, ||
        ic_cdk::spawn(do_self_call())
    );
    TIMER_STATE.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
}

pub async fn do_self_call(){
    let cnstr : String= RUNTIME_STATE.with(|s|{s.borrow().data.get_self_id()});
    let self_call: Result<((),), (ic_cdk::api::call::RejectionCode, String)> 
    = canister_call(cnstr.as_str(), "self_call0", (), None).await;
    match self_call {
        Ok(_) => { log("Self-call complete"); },
        Err(e) => { log(format!("Error (do_self_call) - {:?}, {}",e.0, e.1));} 
    }
}


