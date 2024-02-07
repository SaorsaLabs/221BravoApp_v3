use candid::CandidType;
use defi_oracle_shared::shared_types::OverviewV1;
use serde::{Deserialize, Serialize};

use crate::{
    store::{
        price_data::{OHLC, PriceTuple}, 
        btree::{push_m5_to_store, push_m15_to_store, push_h1_to_store, push_w1_to_store, push_d1_to_store}
    }, 
    core::{runtime::RUNTIME_STATE, utils::log}
};
use super::constants::{M5_AS_NANOS, M15_AS_NANOS, H1_AS_NANOS, D1_AS_NANOS, W1_AS_NANOS};

#[derive(CandidType, Deserialize, Serialize, Default, Clone, Debug)]
pub struct OHLCBucket {
    cross: String,
    pub m5: OHLC,
    pub m15: OHLC,
    pub h1: OHLC,
    pub d1: OHLC,
    pub w1: OHLC
}
impl OHLCBucket {
    pub fn init(cross: String, start_time: u64) -> OHLCBucket {
        // get next midnight for start time
        let ret = OHLCBucket {
            cross,
            m5: OHLC::new(
                start_time, 
                start_time + M5_AS_NANOS, 
                PriceTuple{ cross_price: 0.0, usd_price: 0.0 }, 
                PriceTuple{ cross_price: 0.0, usd_price: 0.0 },  
                PriceTuple{ cross_price: f64::MAX, usd_price: f64::MAX},  
                PriceTuple{ cross_price: 0.0, usd_price: 0.0 },  
                0
            ),
            m15: OHLC::new(
                start_time, 
                start_time + M15_AS_NANOS, 
                PriceTuple{ cross_price: 0.0, usd_price: 0.0 }, 
                PriceTuple{ cross_price: 0.0, usd_price: 0.0 },  
                PriceTuple{ cross_price: f64::MAX, usd_price: f64::MAX},  
                PriceTuple{ cross_price: 0.0, usd_price: 0.0 }, 
                0
            ),
            h1: OHLC::new(
                start_time, 
                start_time + H1_AS_NANOS, 
                PriceTuple{ cross_price: 0.0, usd_price: 0.0 }, 
                PriceTuple{ cross_price: 0.0, usd_price: 0.0 },  
                PriceTuple{ cross_price: f64::MAX, usd_price: f64::MAX},  
                PriceTuple{ cross_price: 0.0, usd_price: 0.0 }, 
                0
            ),
            d1: OHLC::new(
                start_time, 
                start_time + D1_AS_NANOS, 
                PriceTuple{ cross_price: 0.0, usd_price: 0.0 }, 
                PriceTuple{ cross_price: 0.0, usd_price: 0.0 },  
                PriceTuple{ cross_price: f64::MAX, usd_price: f64::MAX},  
                PriceTuple{ cross_price: 0.0, usd_price: 0.0 }, 
                0
            ),
            w1: OHLC::new(
                start_time, 
                start_time + W1_AS_NANOS, 
                PriceTuple{ cross_price: 0.0, usd_price: 0.0 }, 
                PriceTuple{ cross_price: 0.0, usd_price: 0.0 },  
                PriceTuple{ cross_price: f64::MAX, usd_price: f64::MAX},  
                PriceTuple{ cross_price: 0.0, usd_price: 0.0 }, 
                0
            ),
        };
        return ret;
    }

    pub fn m5_start_next_bar(&mut self, open_time: u64, close_time: u64, open_price: PriceTuple){
        self.m5.open_time = open_time;
        self.m5.close_time = close_time;
        self.m5.open = open_price;
        self.m5.high = open_price;
        self.m5.low = open_price;
        self.m5.close = open_price;
        self.m5.volume = 0;
    }

    pub fn m15_start_next_bar(&mut self, open_time: u64, close_time: u64, open_price: PriceTuple){
        self.m15.open_time = open_time;
        self.m15.close_time = close_time;
        self.m15.open = open_price;
        self.m15.high = open_price;
        self.m15.low = open_price;
        self.m15.close = open_price;
        self.m15.volume = 0;
    }

    pub fn h1_start_next_bar(&mut self, open_time: u64, close_time: u64, open_price: PriceTuple){
        self.h1.open_time = open_time;
        self.h1.close_time = close_time;
        self.h1.open = open_price;
        self.h1.high = open_price;
        self.h1.low = open_price;
        self.h1.close = open_price;
        self.h1.volume = 0;
    }

    pub fn d1_start_next_bar(&mut self, open_time: u64, close_time: u64, open_price: PriceTuple){
        self.d1.open_time = open_time;
        self.d1.close_time = close_time;
        self.d1.open = open_price;
        self.d1.high = open_price;
        self.d1.low = open_price;
        self.d1.close = open_price;
        self.d1.volume = 0;
    }

    pub fn w1_start_next_bar(&mut self, open_time: u64, close_time: u64, open_price: PriceTuple){
        self.w1.open_time = open_time;
        self.w1.close_time = close_time;
        self.w1.open = open_price;
        self.w1.high = open_price;
        self.w1.low = open_price;
        self.w1.close = open_price;
        self.w1.volume = 0;
    }  
}

pub fn add_cross_to_processing_bucket(cross: String, start_time: u64) -> String {
    let bucket = OHLCBucket::init(cross.clone(), start_time);
    // check if already added
    let added = RUNTIME_STATE.with(|s|{
        let existing = &s.borrow().processing;
        let mut exists = false;
        for bkt in existing {
            if &bkt.cross == &cross { exists = true }
        }
        return exists;
    });
    // already exists, return early
    if added == true { return "Cross already exists".to_string()} 
    // doesn't exist, add. 
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().processing.push(bucket);
    });
    return "Cross added".to_string();
}

pub fn remove_cross_from_processing_bucket(cross: String) -> String {
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().processing.retain(|x: &OHLCBucket | x.cross != cross)
    });
    let rtn:String = String::from("Cross has been removed");
    return rtn;
}

pub fn update_prices_in_processing_bucket(latest_quotes: Vec<OverviewV1> ){
    let mut updated: Vec<OHLCBucket> = Vec::new();
    // current prices
    let hist_bkt = RUNTIME_STATE.with(|s|{
        let mut current_bucket : Vec<OHLCBucket> = Vec::new();
        for bkt in s.borrow().processing.clone() {
            current_bucket.push(bkt);
        }
        return current_bucket;
    });

    for mut bkt in hist_bkt {
        let mut av_price: f64 = 0.0;
        let mut update_time = 0_u64;
        let mut cross_to_usd_ratio: f64 = 0.0;

        // match bkt to latest quotes
        for rte in &latest_quotes {
            if &rte.token_cross == &bkt.cross {
                av_price = rte.average_price;
                update_time = rte.snapshot_time;
                cross_to_usd_ratio = rte.cross_to_usd;
            }
        }
    
        if av_price > 0.0 {
            // m5
            if &update_time > &bkt.m5.close_time {
                // push to btree store
                push_m5_to_store(bkt.cross.clone(), bkt.m5.clone());
                // start next empty OHLC
                let new_close = bkt.m5.close_time.clone() + M5_AS_NANOS;
                let new_open_price = bkt.m5.close.clone();
                bkt.m5_start_next_bar(bkt.m5.close_time, new_close, new_open_price);
            }
            // is within bar window
            if &update_time >= &bkt.m5.open_time && &update_time <= &bkt.m5.close_time {
                // check
                if &bkt.m5.open.cross_price == &0.0 { 
                    bkt.m5.open = PriceTuple{ cross_price: av_price.clone(), usd_price: cross_to_usd_ratio.clone() };
                } // catch first quote since init
                if &av_price > &bkt.m5.high.cross_price { bkt.m5.high = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio }}
                if &av_price < &bkt.m5.low.cross_price { bkt.m5.low = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio }}
                // update close
                bkt.m5.close = PriceTuple{ cross_price: av_price, usd_price: av_price*cross_to_usd_ratio };
            }
    
            // m15
            if &update_time > &bkt.m15.close_time {
                // push to btree store
                push_m15_to_store(bkt.cross.clone(), bkt.m15.clone());
                // start next empty OHLC
                let new_close = bkt.m15.close_time.clone() + M15_AS_NANOS;
                let new_open_price = bkt.m15.close.clone();
                bkt.m15_start_next_bar(bkt.m15.close_time, new_close, new_open_price);
            }
            // is within bar window
            if &update_time >= &bkt.m15.open_time && &update_time <= &bkt.m15.close_time {
                // check
                if &bkt.m15.open.cross_price == &0.0 { bkt.m15.open = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio } } // catch first quote since init
                if &av_price > &bkt.m15.high.cross_price { bkt.m15.high = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio } }
                if &av_price < &bkt.m15.low.cross_price { bkt.m15.low = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio } }
                // update close
                bkt.m15.close = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio };
            }                
    
            // h1
            if &update_time > &bkt.h1.close_time {
                // push to btree store
                push_h1_to_store(bkt.cross.clone(), bkt.h1.clone());
                // start next empty OHLC
                let new_close = bkt.h1.close_time.clone() + H1_AS_NANOS;
                let new_open_price = bkt.h1.close.clone();
                bkt.h1_start_next_bar(bkt.h1.close_time, new_close, new_open_price);
            }
            // is within bar window
            if &update_time >= &bkt.h1.open_time && &update_time <= &bkt.h1.close_time {
                // check
                if &bkt.h1.open.cross_price == &0.0 { bkt.h1.open = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio } } // catch first quote since init
                if &av_price > &bkt.h1.high.cross_price { bkt.h1.high = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio } }
                if &av_price < &bkt.h1.low.cross_price { bkt.h1.low = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio } }
                // update close
                bkt.h1.close = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio };
            }  
    
            // d1
            if &update_time > &bkt.d1.close_time {
                // push to btree store
                push_d1_to_store(bkt.cross.clone(), bkt.d1.clone());
                // start next empty OHLC
                let new_close = bkt.d1.close_time.clone() + D1_AS_NANOS;
                let new_open_price = bkt.d1.close.clone();
                bkt.d1_start_next_bar(bkt.d1.close_time, new_close, new_open_price);
            }
            // is within bar window
            if &update_time >= &bkt.d1.open_time && &update_time <= &bkt.d1.close_time {
                // check
                if &bkt.d1.open.cross_price == &0.0 { bkt.d1.open = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio } } // catch first quote since init
                if &av_price > &bkt.d1.high.cross_price { bkt.d1.high = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio } }
                if &av_price < &bkt.d1.low.cross_price { bkt.d1.low = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio } }
                // update close
                bkt.d1.close = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio };
            } 
    
            // w1
            if &update_time > &bkt.w1.close_time {
                // push to btree store
                push_w1_to_store(bkt.cross.clone(), bkt.w1.clone());
                // start next empty OHLC
                let new_close = bkt.w1.close_time.clone() + W1_AS_NANOS;
                let new_open_price = bkt.w1.close.clone();
                bkt.w1_start_next_bar(bkt.w1.close_time, new_close, new_open_price);
            }
            // is within bar window
            if &update_time >= &bkt.w1.open_time && &update_time <= &bkt.w1.close_time {
                // check
                if &bkt.w1.open.cross_price == &0.0 { bkt.w1.open = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio } } // catch first quote since init
                if &av_price > &bkt.w1.high.cross_price { bkt.w1.high = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio } }
                if &av_price < &bkt.w1.low.cross_price { bkt.w1.low = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio } }
                // update close
                bkt.w1.close = PriceTuple{ cross_price: av_price.clone(), usd_price: av_price*cross_to_usd_ratio };
            } 
        }
        updated.push(bkt);
    }
    
    // store updated values
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().processing = updated;
    })
}

pub fn get_prices_from_processing_bucket(cross: String) -> Option<OHLCBucket> {
    let data = RUNTIME_STATE.with(|s|s.borrow().processing.clone());
    for bkt in data {
        if &cross == &bkt.cross {
            return Some(bkt);
        }
    }
    return None;
}
   


