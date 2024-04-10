use candid::Principal;
use futures::future::join_all;
use oracle_shared_mk2::shared_types::{OverviewV1, StableCurrency};

use crate::{core::{constants::OC_ALERT_BOT, runtime::RUNTIME_STATE, stable_memory::{PriceTuple, STABLE_STATE}, types::IDKey, utils::{canister_call, critical_err, log}}, price_alerts::{btree::InputAlert, fetch::fetch_icp_usd_rate}, test_data::test_oracle_data};

pub async fn schedule_data_processing(){
    // fetch and update latest prices
    fetch_and_update_prices().await;
    let inst_count = ic_cdk::api::instruction_counter();
    let num_sent = send_alerts().await;
    
    // update last update time 
    let time_now = ic_cdk::api::time();  // this gets the current time in nano seconds. 
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.last_update_time = time_now;
    });

    log(format!("Processing Complete. Alerts Sent :: {}, fetch+update instructions :: {}", num_sent, inst_count));
}

pub async fn fetch_and_update_prices(){
    // fetch
    let oracle_id = RUNTIME_STATE.with(|s|{
        s.borrow().data.get_oracle_id()
    });
    let mut new_prices: Vec<PriceTuple> = Vec::new();
    match oracle_id {
        Some(v) => {
            let call_result:Result<(Vec<OverviewV1>,), (ic_cdk::api::call::RejectionCode, String)> = 
            canister_call(v.as_str(), "get_all_quotes_v1", StableCurrency::USD, None).await;
            let mut idk: IDKey;
            match call_result {
                Ok(res) => {
                    for pr in res.0.iter() {
                        idk = IDKey::from_string(pr.token_cross.clone()).unwrap();
                        new_prices.push( PriceTuple { cross: idk.clone(), price: pr.average_price.clone() })
                    }
                },
                Err(e) => {
                    let error = String::from(format!("Error fetching Quotes V1 - {:?}. {}", e.0, e.1));
                    critical_err(error).await;
                    return;
                },
            }
        },
        None => {  
            let error = String::from("Error - Oracle ID has not been set!");
            critical_err(error).await; 
            return;
        }
    }

    // Fetch ICP Price (Once every 5 calls of the timer)
    let icp_count = RUNTIME_STATE.with(|s|{s.borrow().data.icp_fetch_count});
    match icp_count {
        Some(v) => {
            if v >= 5 {
                match fetch_icp_usd_rate().await {
                    Ok(v) => {
                        let cross = IDKey::from_str("ICP/USD").unwrap();
                        new_prices.push( PriceTuple { cross, price: v.0.clone()});
                        RUNTIME_STATE.with(|s|{
                            s.borrow_mut().data.icp_last_price = Some(v.0);
                        });
                    },
                    Err(e) => { log(e) }
                }
            } else {
                // persist ICP/USD between updates
                let stored_price = RUNTIME_STATE.with(|s|{s.borrow().data.icp_last_price});
                if let Some(sp) = stored_price {
                    let cross = IDKey::from_str("ICP/USD").unwrap();
                    new_prices.push( PriceTuple { cross, price: sp});
                }
            }
            
            if v >=5 {
                // reset count
                RUNTIME_STATE.with(|s|{
                    s.borrow_mut().data.icp_fetch_count = Some(1);
                });
            } else {
                //+1 count
                RUNTIME_STATE.with(|s|{
                    let new = v + 1;
                    s.borrow_mut().data.icp_fetch_count = Some(new);
                });
            }
        },
        None => {
            match fetch_icp_usd_rate().await {
                Ok(v) => {
                    let cross = IDKey::from_str("ICP/USD").unwrap();
                    new_prices.push( PriceTuple { cross, price: v.0.clone()});
                    RUNTIME_STATE.with(|s|{
                        s.borrow_mut().data.icp_last_price = Some(v.0);
                    });
                },
                Err(e) => { log(e) }
            }
            RUNTIME_STATE.with(|s|{
                s.borrow_mut().data.icp_fetch_count = Some(1);
            });
        }
    }

    // check previous prices != 0 
    let previous_prices = STABLE_STATE.with(|s|{
        let mut ptr = s.borrow_mut();
        let state = ptr.as_mut().unwrap();
        let vec_len = state.previous_prices.len();
        if vec_len > 0 { state.get_all_previous_prices() } else { 
            state.overwrite_stored_prices(new_prices.clone());
            let empty_vec: Vec<PriceTuple> = Vec::new();
            empty_vec
        }
    });
    if previous_prices.len() == 0 {return}

    // process  
    let keys = STABLE_STATE.with(|s|{
        let ptr = s.borrow();
        let state = ptr.as_ref().unwrap();
        state.alert_data.get_all_cross_keys()
    });

    // clear waiting alerts
    RUNTIME_STATE.with(|s|{s.borrow_mut().data.alerts_pending.clear()});

    // update prev prices
    STABLE_STATE.with(|s|{
        let mut ptr = s.borrow_mut();
        let state = ptr.as_mut().unwrap();
        state.overwrite_stored_prices(new_prices.clone());
    });

    // chunk this? 
    for cc in keys {
        if let Some(cca) = STABLE_STATE.with(|s|{
            let ptr = s.borrow();
            let state = ptr.as_ref().unwrap();
            state.alert_data.get_all_alerts_by_cross(cc.clone())
        }) {
            let latest_price = lookup_price(&new_prices, &cc);
            let previous_price = lookup_price(&previous_prices, &cc);
            match (latest_price, previous_price){
                (Some(l), Some(p)) => {
                    // loop over alerts for X cross
                    for alert in cca {
                        check_alert(&alert, &l, &p);
                    }
                },
                _ => { return }
            }
        }
    }
}

fn check_alert(alert: &InputAlert, current_price: &f64, previous_price: &f64) {
    // check cross over
    if alert.direction == 1 && 
       current_price > &alert.price && 
       previous_price <= &alert.price {
            RUNTIME_STATE.with(|s|{s.borrow_mut().data.alerts_pending.push(alert.clone())});
    }
    // check cross under
    if alert.direction == 0 && 
    current_price < &alert.price && 
    previous_price >= &alert.price {
         RUNTIME_STATE.with(|s|{s.borrow_mut().data.alerts_pending.push(alert.clone())});
    }
}

fn lookup_price(prices: &Vec<PriceTuple>, target: &IDKey) -> Option<f64> {
    for pr in prices {
        if pr.cross.0 == target.0 {
            return Some(pr.price)
        }
    }
    None
}

pub async fn send_alerts() -> u32 {

    let pending_alerts = RUNTIME_STATE.with(|s|{
        s.borrow().data.alerts_pending.clone()
    });
    let mut messages_sent:u32 = 0;

    log(format!("Send Alerts - Pending :: {}", pending_alerts.len()));
    //let mut future_vec = Vec::new();
    let mut message: String;
    let mut direction_message; 
    for alert in pending_alerts {
        if alert.direction == 0 {
            direction_message = "crossed BELOW ⬇️";
        } else {
            direction_message = "crossed ABOVE ⬆️";
        };
        message = format!(
            "[][]-- Price Alert --[][] <p>{} has {} your price alert set at {}</p> <p><a href='https://221bravo.app/' target='_blank'>Powered by 221Bravo.App</a></p>",
            alert.cross,
            direction_message,
            alert.price
        );
        //future_vec.push( send_alert_to_oc_bot(alert.oc_id, message) );
        let res = send_alert_to_oc_bot(alert.oc_id, message).await;
        match res {
            Ok(_v) => { messages_sent += 1 },
            Err(e) => { log(e) }
        }
    }
    
    // Clear Pending Alerts
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().data.alerts_pending.clear()
    });

    // Update Stats
    RUNTIME_STATE.with(|s| {
        s.borrow_mut().stats.metrics.increment_alerts_sent(messages_sent.clone() as u64)
    });

    messages_sent
}

async fn send_alert_to_oc_bot(oc_id: String, message: String) -> Result<String, String> {
    let pr = Principal::from_text(OC_ALERT_BOT);
    match pr {
        Ok(principal) => {
            let res: Result<(String,), (ic_cdk::api::call::RejectionCode, String)> 
            = ic_cdk::call(principal, "send_message", (oc_id, message,)).await;
                match res {
                Ok(v)=> { 
                    log("Alert Sent");
                    Ok(v.0) 
                },
                Err(e) => { Err(format!("ERROR :: {:?} - {}", e.0, e.1))}
            }
        },
        Err(e) => {
            ic_cdk::trap("Could not convert text to principal");
        },
    }
}

// clone of fetch_and_update_prices with quote data coming from test_data.rs
pub fn fetch_and_update_prices_test(test_input: u8){
    let mut new_prices: Vec<PriceTuple> = Vec::new();
   
    // testing only
    let test_data = test_oracle_data(test_input);
    let mut idk: IDKey;
    for pr in test_data.iter() {
        idk = IDKey::from_string(pr.token_cross.clone()).unwrap();
        new_prices.push( PriceTuple { cross: idk.clone(), price: pr.average_price.clone() })
    }

    // check previous prices != 0 
    let previous_prices = STABLE_STATE.with(|s|{
        let mut ptr = s.borrow_mut();
        let state = ptr.as_mut().unwrap();
        let vec_len = state.previous_prices.len();
        if vec_len > 0 { state.get_all_previous_prices() } else { 
            state.overwrite_stored_prices(new_prices.clone());
            let empty_vec: Vec<PriceTuple> = Vec::new();
            empty_vec
        }
    });
    if previous_prices.len() == 0 {return}

    // process  
    let keys = STABLE_STATE.with(|s|{
        let ptr = s.borrow();
        let state = ptr.as_ref().unwrap();
        state.alert_data.get_all_cross_keys()
    });

    // clear waiting alerts
    RUNTIME_STATE.with(|s|{s.borrow_mut().data.alerts_pending.clear()});

    // chunk this? 
    for cc in keys {
        if let Some(cca) = STABLE_STATE.with(|s|{
            let ptr = s.borrow();
            let state = ptr.as_ref().unwrap();
            state.alert_data.get_all_alerts_by_cross(cc.clone())
        }) {

            let latest_price = lookup_price(&new_prices, &cc);
            let previous_price = lookup_price(&previous_prices, &cc);
            match (latest_price, previous_price){
                (Some(l), Some(p)) => {
                    // loop over alerts for X cross
                    for alert in cca {
                        check_alert(&alert, &l, &p)
                    }
                },
                _ => { return }
            }
        }
    }

    // update prev prices
    STABLE_STATE.with(|s|{
        let mut ptr = s.borrow_mut();
        let state = ptr.as_mut().unwrap();
        state.overwrite_stored_prices(new_prices);
    });

}