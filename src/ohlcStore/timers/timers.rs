use std::time::Duration;

use candid::CandidType;
use defi_oracle_shared::shared_types::{OverviewV1, StableCurrency};
use serde::{Deserialize, Serialize};

use crate::core::{
        runtime::RUNTIME_STATE, 
        utils::{canister_call, critical_err, log}
    };
use super::{state::TIMER_STATE, processing::update_prices_in_processing_bucket};

pub fn start_processing_timer(secs: u64) {
    let secs = Duration::from_secs(secs);
    let timer_id = ic_cdk_timers::set_timer_interval(secs, ||
        ic_cdk::spawn(schedule_data_processing_temp()) // schedule_data_processing *** TEMP FIX!
    );
    TIMER_STATE.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
struct QuoteArgs(StableCurrency);

// Fetch Main quotes
pub async fn schedule_data_processing(){
    let ocid = RUNTIME_STATE.with(|s|{s.borrow().data.get_oracle_id()});

    let args = QuoteArgs(StableCurrency::ICP);

    let oracle_call:  Result<(Vec<OverviewV1>,), (ic_cdk::api::call::RejectionCode, String)> 
    = canister_call(&ocid, "get_all_quotes_v1", args).await;
    match oracle_call {
        Ok(v) => {
           log("GOT Data from CPC!");

           // chunk  processing
             update_prices_in_processing_bucket(v.0.clone());

             // handle errors !! 
            // let time = ic_cdk::api::time();
            // RUNTIME_STATE.with(|s|{
            //     s.borrow_mut().stats.last_update_time = time;
            // });
        },
        Err(e) => {
            critical_err(format!("Error (schedule_data_processing) - {:?} - {}", e.0, e.1)).await;
        }
    }
}


// TEMP FIX FOR INSTRUCTION LIMIT ISSUE
pub async fn schedule_data_processing_temp(){
    let ocid = RUNTIME_STATE.with(|s|{s.borrow().data.get_oracle_id()});

    let args = QuoteArgs(StableCurrency::ICP);

    let oracle_call:  Result<(Vec<OverviewV1>,), (ic_cdk::api::call::RejectionCode, String)> 
    = canister_call(&ocid, "get_all_quotes_v1", args).await;
    match oracle_call {
        Ok(v) => {
           log("Temp Call - GOT Data from CPC!");
            // trim to only the desired token
            let ret_len = v.0.len();
            let mut new_vec: Vec<OverviewV1> = Vec::new();
            let chunk_size = 6_usize;
            let mut chunk_point = chunk_size; 
            for (i, ov) in v.0.iter().enumerate() {
                new_vec.push(ov.clone());
                if i == chunk_point || i == ret_len-1 {
                    // process chunks
                    let self_id = RUNTIME_STATE.with(|s|{s.borrow().data.get_self_id()});
                    let chunk_call:  Result<((),), (ic_cdk::api::call::RejectionCode, String)> 
                    = canister_call(&self_id, "self_call", new_vec.clone()).await;
                    match chunk_call {
                        Ok(_v) => {
                            chunk_point += chunk_size;
                            let nvlen = new_vec.len();
                            log(format!("Chunk Processed - len: {}", nvlen));
                            new_vec.clear();
                        },
                        Err(e) => {
                            log(format!("Error - (schedule_data_processing_temp) {:?} {}", e.0, e.1))
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

// Fetch Main quotes
pub async fn test_schedule_data_processing(from: u64, to: u64){
    let ocid = RUNTIME_STATE.with(|s|{s.borrow().data.get_oracle_id()});

    let args = QuoteArgs(StableCurrency::ICP);

    let oracle_call:  Result<(Vec<OverviewV1>,), (ic_cdk::api::call::RejectionCode, String)> 
    = canister_call(&ocid, "get_all_quotes_v1", args).await;
    match oracle_call {
        Ok(mut v) => {
           log("TEST CALL - GOT Data from CPC!");
            // trim to only the desired token
            let mut newVec: Vec<OverviewV1> = Vec::new();
            for (i, ov) in v.0.iter().enumerate() {
                if i >= from as usize && i<= to as usize {
                    newVec.push(ov.clone());
                }
            }
             update_prices_in_processing_bucket(newVec);
            // let time = ic_cdk::api::time();
            // RUNTIME_STATE.with(|s|{
            //     s.borrow_mut().stats.last_update_time = time;
            // });
        },
        Err(e) => {
            critical_err(format!("Error (schedule_data_processing) - {:?} - {}", e.0, e.1)).await;
        }
    }
}



