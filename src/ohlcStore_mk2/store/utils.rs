use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{core::stable_memory::STABLE_STATE, timers::utils::next_midnight_time};


#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct TokenPriceChange{
    pub cross: String, 
    pub latest_price: f64,
    pub change_24: f64,
    pub change_7d: f64,
    pub sparkline: Vec<f64>
}

pub fn calculate_all_price_changes() -> Vec<TokenPriceChange> {
    let mut ret_data: Vec<TokenPriceChange> = Vec::new();
    let crosses = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().get_all_crosses()
    });
    for crs in crosses {
        STABLE_STATE.with(|s|{
            let binding = s.borrow();
            let state = binding.as_ref().unwrap();

            let mut change_24 = 0.0;
            let mut change_7d = 0.0;
            let mut latest_price = 0.0;
            let mut sparkline: Vec<f64> = Vec::new();
            // 24hr hist price
            let data24 = state.get_m15_price_data(crs.clone(), 96);
            // 7d hist price
            let data7d = state.get_d1_price_data(crs.clone(), 7);
            match (data24, data7d) {
                (Some(v24), Some(v7)) => {
                    let last24 = v24.last();
                    let last7d = v7.last();
                    match (last24, last7d){
                        (Some(l24), Some(l7)) => {
                            //changes
                            change_24 = ((v24[0].close.usd_price - l24.close.usd_price) / l24.close.usd_price) * 100.00;
                            change_7d = ((v7[0].close.usd_price - l7.close.usd_price) / l7.close.usd_price) * 100.00;
                            latest_price = v24[0].close.usd_price;
                            // sparkline 
                            for ohlc in v24 {
                                sparkline.push(ohlc.close.usd_price);
                            }
                        },
                        (_,_) => {}
                    }
                }
                (_, _) => {}
            }
            // push OP
            let change_data = TokenPriceChange{
                cross: crs,
                latest_price,
                change_24,
                change_7d,
                sparkline,
            };
            ret_data.push(change_data);
        });
    }
    return ret_data;
}

pub fn add_cross_impl(cross: String) -> Option<u64> {
    let res = STABLE_STATE.with(|s|{
        s.borrow_mut().as_mut().unwrap()
        .directory_data.add_id(cross.clone())
    });
    match res {
        Some(v) => {
            let process_from = next_midnight_time();
            STABLE_STATE.with(|s|{
                let mut binding = s.borrow_mut();
                let state = binding.as_mut().unwrap();
                state.add_cross_to_store(cross, v.clone(), process_from)
            });
            return Some(v);
        },
        None => {
            return None;
        }
    }
}