use std::time::Duration;

use ic_cdk::query;
use ic_cdk_macros::update;
use ic_cdk_timers::TimerId;
use crate::core::runtime::RUNTIME_STATE;
use crate::core::stable_memory::STABLE_STATE;
use crate::core::types::IDKey;
use crate::core::utils::log;
use crate::price_alerts::btree::InputAlert;
use crate::price_alerts::fetch::fetch_icp_usd_rate;
use super::state::TIMER_STATE;
use super::timers::{fetch_and_update_prices, schedule_data_processing, send_alerts};

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
fn start_alert_timer(secs: u64) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});

    // check if running already
    let is_running = RUNTIME_STATE.with(|s|{
        s.borrow().stats.get_timer_state()
    });
     if is_running == true {
        return String::from("Main quotes timer is already running");
    }

    // check if oracle ID is set
    let oracle = RUNTIME_STATE.with(|s|{
        s.borrow().data.get_oracle_id()
    });
    if oracle == None {
        ic_cdk::trap("Oracle ID has not been set!");
    }

    // add to timer to state
    let secs = Duration::from_secs(secs);
    let timer_id = ic_cdk_timers::set_timer_interval(secs, ||
        ic_cdk::spawn(schedule_data_processing())
    );
    TIMER_STATE.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.update_timer(true)
    });

    log("[][] ---- Starting Timer ---- [][]");
    String::from("Processing timer has been started")
}


#[update]
async fn test_call(){
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());}); 
   fetch_and_update_prices().await;

   let pending = RUNTIME_STATE.with(|s|{
    s.borrow().data.alerts_pending.clone()
   });
   log(format!("PENDING :: {:?}", pending));
   send_alerts().await;
}

#[update]
async fn test_call2(){
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());}); 

    // add alert
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().data.alerts_pending.push(InputAlert{
            id: 0,
            user: String::from("XX-XX-XX"),
            cross: String::from("TEST/TEST"),
            oc_id: String::from("zx4ws-zqaaa-aaaar-adsuq-cai"),
            price: 123.1,
            direction: 0
        });
    });
    send_alerts().await;
}

#[query]
async fn test_call3() -> Option<Vec<InputAlert>> {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());}); 

    let alerts = STABLE_STATE.with(|s|{
        let cross = IDKey::from_str("CHAT/ICP").unwrap();
        let ptr = s.borrow();
        let state = ptr.as_ref().unwrap();
        state.alert_data.get_all_alerts_by_cross(cross)
    });
    alerts
}

#[update]
async fn manual_trigger_update(){
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());}); 
    schedule_data_processing().await;
}

#[update]
async fn test_icp_price_fetch() -> String {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());}); 
    match fetch_icp_usd_rate().await {
        Ok(v) => {
            let op = format!("{:?}", v);
            return op;
        },
        Err(e) => {
            return e;
        }
    }
}
