use ic_cdk_macros::update;
use ic_cdk_timers::TimerId;
use oracle_shared_mk2::shared_types::InternalRateEntry;
use crate::{
    core::{runtime::RUNTIME_STATE, utils::log}, cpc::fetch_data::{fetch_icp_usd_rate, fetch_icp_xdr_rate}
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
                // check if ICP/USD exists 
                let exists:bool = RUNTIME_STATE.with(|s|{
                    s.borrow_mut().data.internal_rates.does_swap_exist(String::from("ICP/USD"))
                });
                let time = ic_cdk::api::time();
                if exists == true {
                    RUNTIME_STATE.with(|s|{
                        s.borrow_mut().data.internal_rates.update_single_quote(&String::from("ICP/USD"), v.0, time)
                    });
                } else {
                    let entry: InternalRateEntry = InternalRateEntry{ 
                        swap_pair: String::from("ICP/USD"), 
                        quote: v.0, 
                        timestamp: time
                    };
                    RUNTIME_STATE.with(|s|{
                        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(entry)
                    });
                }
                log(format!("Updated interal ICP/USD Rate - {}",v.0.clone()));
            },
            Err(e) => {
                // return error.. couldn't fetch init ICP/ USD Price.
                log(format!("Could not fetch ICP/USD Rate - {}",e));
                ic_cdk::trap("Could not fetch ICP/USD Rate. Check Logs for more info.");
            }
        }

        // get initial XDR Swap price
        let i2x = fetch_icp_xdr_rate().await;
        match i2x {
            Ok(v) => {
                // check if ICP/XDR exists 
                let rate = v.0 as f64 / 10_000_f64; // /10k to get actual cross
                let exists:bool = RUNTIME_STATE.with(|s|{
                    s.borrow_mut().data.internal_rates.does_swap_exist(String::from("ICP/XDR"))
                });
                let time = ic_cdk::api::time();
                if exists == true {
                    RUNTIME_STATE.with(|s|{
                        s.borrow_mut().data.internal_rates.update_single_quote(&String::from("ICP/XDR"), rate, time)
                    });
                } else {
                    let entry: InternalRateEntry = InternalRateEntry{ 
                        swap_pair: String::from("ICP/XDR"), 
                        quote: rate, 
                        timestamp: time
                    };
                    RUNTIME_STATE.with(|s|{
                        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(entry)
                    });
                }
                log(format!("Updated interal ICP/XDR Rate - {}",v.0.clone()));
            },
            Err(e) => {
                // return error.. couldn't fetch init ICP/ USD Price.
                log(format!("Could not fetch ICP/XDR Rate - {}",e));
                ic_cdk::trap("Could not fetch ICP/XDR Rate. Check Logs for more info.");
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

