use defi_oracle_shared::shared_types::SwapPair;
use ic_cdk_macros::update;
use ic_cdk_timers::TimerId;
use crate::{
    core::{runtime::RUNTIME_STATE, utils::log}, cpc::fetch_data::fetch_icp_usd_rate
    };
use super::{state::TIMER_STATE, timers::start_stable_quotes_timer};
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
        s.borrow_mut().stats.timer_active = false;
    });   
    log("[][] ---- All timers stopped ---- [][]");
    return String::from("All timers stopped");
}

#[update]                   // 60, 900
async fn start_quotes_timer(secs: u64, stable_secs: u64) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});
    
    let ret;
    // check if running already
    let is_running = RUNTIME_STATE.with(|s|{
        s.borrow().stats.timer_active
    });
     if is_running == true {
        ret = String::from("Main quotes timer is already running");
    } else {
        
        // get initial USD Swap price
        let i2u = fetch_icp_usd_rate().await;
        match i2u {
            Ok(v) => {
                log(format!("Updated interal ICP/USD Rate - {}",v.0.clone()));
                RUNTIME_STATE.with(|s|{
                    s.borrow_mut().data.internal_rates.update_single_quote(&SwapPair::ICP_USD, v.0)
                });
            },
            Err(e) => {
                // return error.. couldn't fetch init ICP/ USD Price.
                log(format!("Could not fetch ICP/USD Rate - {}",e));
                ic_cdk::trap("Could not fetch ICP/USD Rate. Check Logs for more info.");
            }
        }

        start_processing_timer(secs);
        start_stable_quotes_timer(stable_secs);

        RUNTIME_STATE.with(|s|{
            s.borrow_mut().stats.timer_active = true;
        });
        ret = String::from("Main quotes has been started");
        log("[][] ---- Starting Main quotes Timer ---- [][]");
    }
    return ret;
}

