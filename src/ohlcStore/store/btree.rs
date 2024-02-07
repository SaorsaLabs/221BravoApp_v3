use candid::CandidType;
use ic_cdk_macros::query;
use serde::{Serialize, Deserialize};

use crate::{
    core::{stable_memory::MAP, utils::log}, 
    timers::{processing::{add_cross_to_processing_bucket, remove_cross_from_processing_bucket, get_prices_from_processing_bucket}, 
    utils::next_midnight_time}
};
use super::{price_data::{init_new_price_data, OHLC, PriceData, PriceTuple}, constants::{M5_MAX, M15_MAX, H1_MAX, D1_MAX, W1_MAX}};

pub fn add_cross_to_store(cross: String){
    if cross.chars().count() > 15 { ic_cdk::trap("Crosses cannot be more than 15 characters") }
    let pd = init_new_price_data(cross.clone());
    // add to btree map/ store
    MAP.with(|s|{
        s.borrow_mut().insert(cross.clone(), pd)
    });
    // add to processing array
    let time_now = next_midnight_time(); // *** MUST start at 00:00 for bars to align. 
    add_cross_to_processing_bucket(cross, time_now);
}

pub fn remove_cross_from_store(cross: String) -> String {
    // remove from btree/store
    let rm = MAP.with(|s|{
        s.borrow_mut().remove(&cross)
    });
    // remove from processing array
    remove_cross_from_processing_bucket(cross);
    match rm {
        Some(_) => { return "Cross removed".to_string() },
        None => { return "Nothing removed - check if cross exists!".to_string() }
    }
}

pub fn get_all_keys() -> Vec<String> {
    MAP.with(|s|{
        let mut keys = Vec::new();
        for (k, _v) in s.borrow().iter() {
            keys.push(k.clone());
        }
        return keys;
    })
}

pub fn count_num_crosses() -> u64 {
    MAP.with(|s|{
         s.borrow().len()
    })
}

pub fn get_price_data(cross: String) -> Option<PriceData> {
    // NOTE DOES NOT RETURN FORMING BARS - ONLY COMPLETE!!
    MAP.with(|s|{
        s.borrow().get(&cross)
    })
}

pub fn get_m5_price_data(cross: String, len: usize) -> Option<Vec<OHLC>> {
    if len == 0 {return None;}
    let data = MAP.with(|s|{
        s.borrow().get(&cross)
    });
    match data {
        Some(v)=> {
            let mut count = len;
            if len > M5_MAX { count = M5_MAX }
            let mut ret_vec: Vec<OHLC> = Vec::new();
            // get latest (forming) bar
            let forming = get_prices_from_processing_bucket(cross);
            match forming {
                Some(fv) => {
                    if len == 1 {
                        ret_vec.push(fv.m5);
                        return Some(ret_vec);
                    } else {
                        let mut m5 = v.get_all_m5();
                        m5.push(fv.m5); // latest bar
                        m5.reverse();
                        if count > m5.len() { count = m5.len() };
                        for i in 0..count {
                            ret_vec.push(m5[i].clone());
                        }
                        return Some(ret_vec);
                    }
                },  
                None => {
                    return None;
                }
            }
        },
        None => { return None; }
    }
}

pub fn get_m15_price_data(cross: String, len: usize) -> Option<Vec<OHLC>> {
    if len == 0 {return None;}
    let data = MAP.with(|s|{
        s.borrow().get(&cross)
    });
    match data {
        Some(v)=> {
            let mut count = len;
            if len > M15_MAX { count = M15_MAX }
            let mut ret_vec: Vec<OHLC> = Vec::new();
            // get latest (forming) bar
            let forming = get_prices_from_processing_bucket(cross);
            match forming {
                Some(fv) => {
                    if len == 1 {
                        ret_vec.push(fv.m15);
                        return Some(ret_vec);
                    } else {
                        let mut m15 = v.get_all_m15();
                        m15.push(fv.m15); // latest bar
                        m15.reverse();
                        if count > m15.len() { count = m15.len() };
                        for i in 0..count {
                            ret_vec.push(m15[i].clone());
                        }
                        return Some(ret_vec);
                    }
                },  
                None => {
                    return None;
                }
            }
        },
        None => { return None; }
    }
}

pub fn get_h1_price_data(cross: String, len: usize) -> Option<Vec<OHLC>> {
    if len == 0 {return None;}
    let data = MAP.with(|s|{
        s.borrow().get(&cross)
    });
    match data {
        Some(v)=> {
            let mut count = len;
            if len > H1_MAX { count = H1_MAX }
            let mut ret_vec: Vec<OHLC> = Vec::new();
            // get latest (forming) bar
            let forming = get_prices_from_processing_bucket(cross);
            match forming {
                Some(fv) => {
                    if len == 1 {
                        ret_vec.push(fv.h1);
                        return Some(ret_vec);
                    } else {
                        let mut h1 = v.get_all_h1();
                        h1.push(fv.h1); // latest bar
                        h1.reverse();
                        if count > h1.len() { count = h1.len() };
                        for i in 0..count {
                            ret_vec.push(h1[i].clone());
                        }
                        return Some(ret_vec);
                    }
                },  
                None => {
                    return None;
                }
            }
        },
        None => { return None; }
    }
}

pub fn get_d1_price_data(cross: String, len: usize) -> Option<Vec<OHLC>> {
    if len == 0 {return None;}
    let data = MAP.with(|s|{
        s.borrow().get(&cross)
    });
    match data {
        Some(v)=> {
            let mut count = len;
            if len > D1_MAX { count = D1_MAX }
            let mut ret_vec: Vec<OHLC> = Vec::new();
            // get latest (forming) bar
            let forming = get_prices_from_processing_bucket(cross);
            match forming {
                Some(fv) => {
                    if len == 1 {
                        ret_vec.push(fv.d1);
                        return Some(ret_vec);
                    } else {
                        let mut d1 = v.get_all_d1();
                        d1.push(fv.d1); // latest bar
                        d1.reverse();
                        if count > d1.len() { count = d1.len() };
                        for i in 0..count {
                            ret_vec.push(d1[i].clone());
                        }
                        return Some(ret_vec);
                    }
                },  
                None => {
                    return None;
                }
            }
        },
        None => { return None; }
    }
}

pub fn get_w1_price_data(cross: String, len: usize) -> Option<Vec<OHLC>> {
    if len == 0 {return None;}
    let data = MAP.with(|s|{
        s.borrow().get(&cross)
    });
    match data {
        Some(v)=> {
            let mut count = len;
            if len > W1_MAX { count = W1_MAX }
            let mut ret_vec: Vec<OHLC> = Vec::new();
            // get latest (forming) bar
            let forming = get_prices_from_processing_bucket(cross);
            match forming {
                Some(fv) => {
                    if len == 1 {
                        ret_vec.push(fv.w1);
                        return Some(ret_vec);
                    } else {
                        let mut w1 = v.get_all_w1();
                        w1.push(fv.w1); // latest bar
                        w1.reverse();
                        if count > w1.len() { count = w1.len() };
                        for i in 0..count {
                            ret_vec.push(w1[i].clone());
                        }
                        return Some(ret_vec);
                    }
                },  
                None => {
                    return None;
                }
            }
        },
        None => { return None; }
    }
}

pub fn push_m5_to_store(cross: String, data: OHLC){
    MAP.with(|s| {
        let map_data = s.borrow().get(&cross);
        match map_data {
            Some(v) => {
                let mut pd: PriceData = v;
                pd.push_m5(data.clone());
                s.borrow_mut().insert(cross, pd);
            },
            None => {}
        }
    })
}

pub fn push_m15_to_store(cross: String, data: OHLC){
    MAP.with(|s| {
        let map_data = s.borrow().get(&cross);
        match map_data {
            Some(v) => {
                let mut pd: PriceData = v;
                pd.push_m15(data.clone());
                s.borrow_mut().insert(cross, pd);
            },
            None => {}
        }
    })
}

pub fn push_h1_to_store(cross: String, data: OHLC){
    MAP.with(|s| {
        let map_data = s.borrow().get(&cross);
        match map_data {
            Some(v) => {
                let mut pd: PriceData = v;
                pd.push_h1(data.clone());
                s.borrow_mut().insert(cross, pd);
            },
            None => {}
        }
    })
}

pub fn push_d1_to_store(cross: String, data: OHLC){
    MAP.with(|s| {
        let map_data = s.borrow().get(&cross);
        match map_data {
            Some(v) => {
                let mut pd: PriceData = v;
                pd.push_d1(data.clone());
                s.borrow_mut().insert(cross, pd);
            },
            None => {}
        }
    })
}

pub fn push_w1_to_store(cross: String, data: OHLC){
    MAP.with(|s| {
        let map_data = s.borrow().get(&cross);
        match map_data {
            Some(v) => {
                let mut pd: PriceData = v;
                pd.push_w1(data.clone());
                s.borrow_mut().insert(cross, pd);
            },
            None => {}
        }
    })
}

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
    let crosses = get_all_keys();
    for crs in crosses {
        let mut change_24 = 0.0;
        let mut change_7d = 0.0;
        let mut latest_price = 0.0;
        let mut sparkline: Vec<f64> = Vec::new();
        // 24hr hist price
        let data24 = get_m15_price_data(crs.clone(), 96);
        // 7d hist price
        let data7d = get_d1_price_data(crs.clone(), 7);
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
    }
    return ret_data;
}

pub fn test_populate_all(cross: String) -> String {
    let op = PriceTuple{ cross_price: 50.0, usd_price: 50.0 };
    let cl = PriceTuple{ cross_price: 100.0, usd_price: 100.0 };
    let hi = PriceTuple{ cross_price: 20.0, usd_price: 20.0 };
    let lo = PriceTuple{ cross_price: 50.00, usd_price: 50.0 };

    let data = 
    OHLC::new(0, u64::MAX, op, cl, hi, lo, 0);

    MAP.with(|s| {
        let x = s.borrow().get(&cross);
        match x {
            Some(v) => {
                let mut pd: PriceData = v;

                // m5
                for _i in 0..M5_MAX{
                    pd.push_m5(data.clone());
                }

                // m15
                for _i in 0..M15_MAX{
                    pd.push_m15(data.clone());
                }

                // h1
                for _i in 0..H1_MAX{
                    pd.push_h1(data.clone());
                }

                // d1
                for _i in 0..D1_MAX{
                    pd.push_d1(data.clone());
                }

                // w1
                for _i in 0..W1_MAX{
                    pd.push_w1(data.clone());
                }
                
                s.borrow_mut().insert(cross, pd);
                return "all data added".to_string();
            },
            None => {
                return "can't push all data - no matching cross found".to_string();
            }
        }
    })
}


pub fn test_get_values(cross: String){
    let values = MAP.with(|s|{
        s.borrow().get(&cross)
    });
    match values {
        Some(v) => {
            log(format!("M5 : {}", v.m5.len()));
            log(format!("M15 : {}", v.m15.len()));
            log(format!("H1 : {}", v.h1.len()));
            log(format!("D1 : {}", v.d1.len()));
            log(format!("W1 : {}", v.w1.len()));
        },
        None => {
            log("Could not find test values");
        }
    }
}