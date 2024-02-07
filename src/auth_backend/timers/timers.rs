use std::time::Duration;


use crate::{
    core::{runtime::RUNTIME_STATE, utils::critical_err, working_stats::count_error}, services::top_tokens::{update_icrc1_total_supply, update_price_data, update_top_holders},
};
use super::state::TIMER_STATE;

pub fn start_processing_timer(secs: u64) { 
    let secs = Duration::from_secs(secs);
    let timer_id = ic_cdk_timers::set_timer_interval(secs, ||
        ic_cdk::spawn(schedule_data_processing())
    );
    TIMER_STATE.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
}

// Fetch Main quotes
async fn schedule_data_processing(){

    match update_price_data().await {
        Ok(_) => {
            // update last update time 
            let time_now = ic_cdk::api::time();
            RUNTIME_STATE.with(|s|{
                s.borrow_mut().stats.last_update_time.0 = time_now;
            });
        },
        Err(_) => {
            count_error();
            critical_err(String::from("Update price data error - check logs!")).await;
        }
    }
}

pub fn start_icrc_stable_timer(secs: u64) {
    let secs = Duration::from_secs(secs);
    let timer_id = ic_cdk_timers::set_timer_interval(secs, ||
        ic_cdk::spawn(schedule_icrc_supply_fetch())
    );
    TIMER_STATE.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
}

async fn schedule_icrc_supply_fetch(){

    match update_icrc1_total_supply().await {
        Ok(_) => {
            let time = ic_cdk::api::time();
            RUNTIME_STATE.with(|s|{
                s.borrow_mut().stats.last_update_time.1 = time; 
            });
        },
        Err(_) => {
            count_error();
            critical_err(String::from("Update icrc supply error - check logs!")).await;
        }
    }
}

pub fn start_top_holder_timer(secs: u64) {
    let secs = Duration::from_secs(secs);
    let timer_id = ic_cdk_timers::set_timer_interval(secs, ||
        ic_cdk::spawn(schedule_top_holder_fetch())
    );
    TIMER_STATE.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
}

async fn schedule_top_holder_fetch(){
    match update_top_holders().await {
        Ok(_) => {
            let time = ic_cdk::api::time();
            RUNTIME_STATE.with(|s|{
                s.borrow_mut().stats.last_update_time.2 = time; 
            });
        },
        Err(_) => {
            count_error();
            critical_err(String::from("Update top holders error - check logs!")).await;
        }
    }
}


