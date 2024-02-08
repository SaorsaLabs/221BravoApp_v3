use std::time::Duration;
use defi_oracle_shared::{utils::{process_mib_quotes}, shared_types::{OverviewV1, SwapPair, InternalRateEntry}};

use crate::{
    core::{runtime::RUNTIME_STATE, utils::{log, critical_err}, working_stats::count_error},
    cpc::{
        fetch_data::{fetch_quotes_from_all_mibs, fetch_icp_xdr_rate, fetch_icp_usd_rate}, 
        constants::MAX_SPREAD_FOR_UPDATE
    }
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
      // get internal rates
      let i_rates = RUNTIME_STATE.with(|s|{
        s.borrow().data.internal_rates.get_all_rates_vec()
    });

    let trade_size = RUNTIME_STATE.with(|s|{
        s.borrow().stats.get_trade_size()
    });

    // get latest quotes
    let quotes = fetch_quotes_from_all_mibs(i_rates, trade_size).await;

    // get latest icp to usd quote
    let i2u = RUNTIME_STATE.with(|s|{
        // set timer gets the init USD price. 
        match s.borrow().data.internal_rates.get_single_rate(&SwapPair::ICP_USD) {
            Some(v) => {
                return v;
            },
            None => {
                return 0.0_f64;
            }
        }
    });

    //combine/ update quotes and internal rates.
    let sorted = process_mib_quotes(quotes, MAX_SPREAD_FOR_UPDATE, i2u);

    // update internal quotes
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().data.internal_rates.update_all_quotes(sorted.1);
    });

    // update external quotes
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().data.update_external_quotes(sorted.0.clone());
    });

    // update last update time 
    let time_now = ic_cdk::api::time();
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.last_update_time.0 = time_now;
    });
    
}

pub fn start_stable_quotes_timer(secs: u64) {
    let secs = Duration::from_secs(secs);
    let timer_id = ic_cdk_timers::set_timer_interval(secs, ||
        ic_cdk::spawn(schedule_stable_quote_fetch())
    );
    TIMER_STATE.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
}

pub async fn schedule_stable_quote_fetch(){
    let mut all_ok = true;

    // ICP/XDR
    let xdr_rate = fetch_icp_xdr_rate().await; // (price, time)
    match xdr_rate {
        Ok(v) => {
            let rate = v.0 as f64 / 10_000_f64; // /10k to get actual cross
               // check if usd exists in internal rates
            let exists = RUNTIME_STATE.with(|s| 
                s.borrow().data.internal_rates.does_swap_exist(SwapPair::ICP_XDR)
            );
            // update existing
            if exists == true {
                RUNTIME_STATE.with(|s|
                    s.borrow_mut().data.internal_rates.update_single_quote(&SwapPair::ICP_XDR, rate)
                );
             // else add new
             } else { 
                RUNTIME_STATE.with(|s|
                    s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(
                        InternalRateEntry{
                            swap_pair: SwapPair::ICP_XDR,
                            quote: rate
                        }
                    )
                );
             }
        },
        Err(e) => {
            all_ok = false;
            RUNTIME_STATE.with(|s|{
                s.borrow_mut().stats.metrics.increment_total_errors()
            });
            count_error();
            log(format!("ERROR: Could not update ICP/XDR Cross. {}", e));
            critical_err(format!("ERROR: Could not update ICP/XDR Cross. {}", e)).await;
        }
    }

    // ICP/USD
    let usd_rate = fetch_icp_usd_rate().await;
    match usd_rate {
        Ok(v) => {
            // check if usd exists in internal rates
            let exists = RUNTIME_STATE.with(|s| 
                s.borrow().data.internal_rates.does_swap_exist(SwapPair::ICP_USD)
            );
            // update existing
            if exists == true {
                RUNTIME_STATE.with(|s|
                    s.borrow_mut().data.internal_rates.update_single_quote(&SwapPair::ICP_USD, v.0)
                );
             // else add new
             } else { 
                RUNTIME_STATE.with(|s|
                    s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(
                        InternalRateEntry{
                            swap_pair: SwapPair::ICP_USD,
                            quote: v.0
                        }
                    )
                );
             }
             let time_now = ic_cdk::api::time();
             RUNTIME_STATE.with(|s|{
                 s.borrow_mut().stats.last_update_time.1 = time_now;
             });
        },
        Err(e) => {
            all_ok = false;
            RUNTIME_STATE.with(|s|{
                s.borrow_mut().stats.metrics.increment_total_errors()
            });
            count_error();
            log(format!("ERROR: Could not update ICP/USD Cross. {}", e));
            critical_err(format!("ERROR: Could not update ICP/USD Cross. {}", e)).await;
        },
    }

    // last update time
    if all_ok == true {
        let time = ic_cdk::api::time();
        RUNTIME_STATE.with(|s|{
            s.borrow_mut().stats.last_update_time.1 = time; 
        });
    }
}


