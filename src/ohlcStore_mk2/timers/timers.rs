use ic_cdk_timers::TimerId;
use crate::core::{runtime::RUNTIME_STATE, utils::log};
use super::{constants::PROCESSING_CHUNK_SIZE, state::TIMER_STATE};
use std::time::Duration;

use candid::CandidType;
use oracle_shared_mk2::shared_types::{OverviewV1, StableCurrency};
use serde::{Deserialize, Serialize};
use crate::core::utils::{canister_call, critical_err};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
struct QuoteArgs(StableCurrency);

pub fn stop_all_timers_impl() -> String {
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

pub fn start_quotes_timer_impl(secs: u64, oracle_canister: String) -> String {
    // check if running already
    let is_running = RUNTIME_STATE.with(|s|{
        s.borrow().stats.get_timer_state()
    });
    if is_running == true {
        return String::from("Main quotes timer is already running");
    }
    // store oracle id
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().data.set_oracle_id(oracle_canister)
    });

    // set timer
    let secs = Duration::from_secs(secs);
    let timer_id = ic_cdk_timers::set_timer_interval(secs, ||
        ic_cdk::spawn(schedule_data_processing()) // schedule_data_processing_temp *** TEMP FIX!
    );
    TIMER_STATE.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));

    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.update_timer(true)
    });

    log("[][] ---- Starting Main quotes Timer ---- [][]");
    return String::from("Main quotes has been started");
   
}

pub async fn schedule_data_processing(){
    let ocid = RUNTIME_STATE.with(|s|{s.borrow().data.get_oracle_id()});
    let args = QuoteArgs(StableCurrency::ICP);

    // fetch prices from oracle
    let oracle_call:  Result<(Vec<OverviewV1>,), (ic_cdk::api::call::RejectionCode, String)> 
    = canister_call(&ocid, "get_all_quotes_v1", args, None).await;
    match oracle_call {
        Ok(v) => {
           log("Got latest prices from oracle");
            // process prices in batches
            let ret_len = v.0.len();
            let mut new_vec: Vec<OverviewV1> = Vec::new();
            let mut chunk_point = PROCESSING_CHUNK_SIZE; 
            for (i, ov) in v.0.iter().enumerate() {
                new_vec.push(ov.clone());
                if i == chunk_point || i == ret_len-1 {
                    // process chunk
                    let self_id = RUNTIME_STATE.with(|s|{s.borrow().data.get_self_id()});
                    let chunk_call:  Result<((),), (ic_cdk::api::call::RejectionCode, String)> 
                    = canister_call(&self_id, "process_chunk", new_vec.clone(), None).await;
                    match chunk_call {
                        Ok(_v) => {
                            chunk_point += PROCESSING_CHUNK_SIZE;
                            let nvlen = new_vec.len();
                            log(format!("Chunk Processed - len: {}", nvlen));
                            // clear vec for next chunk
                            new_vec.clear();
                        },
                        Err(e) => {
                            log(format!("Error - (schedule_data_processing) {:?} {}", e.0, e.1))
                        }
                    }
                }
            }
        },
        Err(e) => {
            critical_err(format!("Error (schedule_data_processing) - {:?} - {}", e.0, e.1)).await;
        }
    }
}


