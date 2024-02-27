use std::cell::RefCell;
use defi_oracle_shared::shared_types::OverviewV1;
use ic_stable_memory::{collections::SVec, derive::{AsFixedSizeBytes, StableType}};
use crate::{
    store::{btree::PriceDataTree, constants::{D1_MAX, H1_MAX, M15_MAX, M5_MAX, W1_MAX}, directory::Directory, types::{OHLCBucket, PriceData, PriceTuple, OHLC}}, 
    timers::{constants::{D1_AS_NANOS, H1_AS_NANOS, M15_AS_NANOS, M5_AS_NANOS, W1_AS_NANOS}, utils::next_midnight_time}
};

use super::types::IDKey;

thread_local! {
    pub static STABLE_STATE: RefCell<Option<Main>> = RefCell::default();
}

#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct Main {
    pub processing_bucket:  SVec<OHLCBucket>,
    pub price_data_tree: PriceDataTree,
    pub directory_data: Directory,
}

impl Main {
    fn add_cross_to_processing_bucket (&mut self, cross: String, start_time: u64) -> String {
        // convert to IDKey
        if cross.chars().count() > 15 { ic_cdk::trap("Crosses cannot be more than 15 characters") }
        let cross_id = IDKey::from_str(&cross).unwrap(); // safe due to above guard. 
        let bucket = OHLCBucket::init(cross_id.clone(), start_time);

        // check if already added
        for bkt in self.processing_bucket.iter() {
            if cross_id.0 == *bkt.cross.0 {
                return String::from("Cross already exists");
            }
        }

        // doesn't exist, push new bucket
        self.processing_bucket.push(bucket).expect("Storage is full");
        self.directory_data.add_id(cross);
        return String::from("Cross added");
    }

    fn remove_cross_from_processing_bucket(&mut self, cross: String) -> String {
        if cross.chars().count() > 15 { ic_cdk::trap("Crosses cannot be more than 15 characters") }
        let cross_id = IDKey::from_str(&cross).unwrap(); // safe due to above guard. 
        // find matching key
        let mut pos = 0;
        let mut del_pos = -1; 
        for bkt in self.processing_bucket.iter() {
            if cross_id.0 == *bkt.cross.0 {
                del_pos = pos;
            }
            pos +=1;            
        }
        // delete it
        if del_pos != -1 {
            self.processing_bucket.remove(del_pos as usize);
            return "Cross Removed".to_string();
        } 
            
        return "Could not find Cross".to_string();
    }

    pub fn overwrite_processing_bucket(&mut self, data: OHLCBucket) {
        // find and remove any existing
        let mut found = false;
        let mut pos = 0;
        for bkt in self.processing_bucket.iter(){
            if &bkt.cross.0 == &data.cross.0 {
                found = true;
                break;
            }
            pos += 1;
        }
        if found == true { self.processing_bucket.remove(pos as usize); }

        // check sucessfully removed 
        for bkt in self.processing_bucket.iter() {
            if data.cross.0 == *bkt.cross.0 {
                ic_cdk::trap("Cross already exists");
            }
        }

        // push new bucket
        self.processing_bucket.push(data).expect("Storage is full");
    }

    pub fn clone_processing_bucket(&self) -> Vec<OHLCBucket> {
        let mut ret_vec: Vec<OHLCBucket> = Vec::new();
        for bkt in self.processing_bucket.iter() {
            ret_vec.push(bkt.clone());
        }
        return ret_vec;
    }

    pub fn update_prices_in_processing_bucket(&mut self, latest_quotes: Vec<OverviewV1> ){
        let old_bkt =  self.clone_processing_bucket();
        for ov1 in latest_quotes {
            let cross_id = IDKey::from_string(ov1.token_cross.clone()).unwrap(); // all keys from OverviewV1 will be less than 15 chars. Unwrap is ok
            let mut pos = 0_usize;
            for bkt in old_bkt.iter() {
                if cross_id.0 == bkt.cross.0 {
                    if ov1.average_price > 0.0 {
                        // lookup processing bucket
                        if let Some(mut v) = self.processing_bucket.get_mut(pos){
                        let id_ref = self.directory_data.get_ref(ov1.token_cross.clone()).unwrap(); // should always exist in the directory if present in bucket
                        // M5 ACTIONS
                            // new bar required 
                            if &ov1.snapshot_time > &bkt.m5.close_time {
                                // push to btree store
                                if let Some(mut price_store) = self.price_data_tree.price_data.get_mut(&id_ref) {
                                    price_store.m5.push(bkt.m5.clone()).expect("Storage is full");
                                    if &price_store.m5.len() > &M5_MAX { price_store.m5.remove(0); }
                                }
                                // start next bar
                                let new_close = bkt.m5.close_time.clone() + M5_AS_NANOS;
                                v.m5_start_next_bar(bkt.m5.close_time, new_close, bkt.m5.close.clone());
                            }
                            // update existing bar
                            if &ov1.snapshot_time >= &bkt.m5.open_time && &ov1.snapshot_time <= &bkt.m5.close_time {
                                // catch first quote since init
                                if &bkt.m5.open.cross_price == &0.0 {
                                    v.m5.open = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd };
                                }
                                // update high
                                if &ov1.average_price > &bkt.m5.high.cross_price { 
                                    v.m5.high = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd }
                                }
                                // update low
                                if &ov1.average_price < &bkt.m5.low.cross_price { 
                                    v.m5.low = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd }
                                }
                                // update close
                                v.m5.close = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd };
                            }

                        // M15 ACTIONS
                            // new bar required 
                            if &ov1.snapshot_time > &bkt.m15.close_time {
                                // push to btree store
                                if let Some(mut price_store) = self.price_data_tree.price_data.get_mut(&id_ref) {
                                    price_store.m15.push(bkt.m15.clone()).expect("Storage is full");
                                    if &price_store.m15.len() > &M15_MAX { price_store.m15.remove(0); }
                                }
                                // start next bar
                                let new_close = bkt.m15.close_time.clone() + M15_AS_NANOS;
                                v.m15_start_next_bar(bkt.m15.close_time, new_close, bkt.m15.close.clone());
                            }
                            // update existing bar
                            if &ov1.snapshot_time >= &bkt.m15.open_time && &ov1.snapshot_time <= &bkt.m15.close_time {
                                // catch first quote since init
                                if &bkt.m15.open.cross_price == &0.0 {
                                    v.m15.open = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd };
                                }
                                // update high
                                if &ov1.average_price > &bkt.m15.high.cross_price { 
                                    v.m15.high = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd }
                                }
                                // update low
                                if &ov1.average_price < &bkt.m15.low.cross_price { 
                                    v.m15.low = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd }
                                }
                                // update close
                                v.m15.close = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd };
                            }

                        // H1 ACTIONS
                            // new bar required 
                            if &ov1.snapshot_time > &bkt.h1.close_time {
                                // push to btree store
                                if let Some(mut price_store) = self.price_data_tree.price_data.get_mut(&id_ref) {
                                    price_store.h1.push(bkt.h1.clone()).expect("Storage is full");
                                    if &price_store.h1.len() > &H1_MAX { price_store.h1.remove(0); }
                                }
                                // start next bar
                                let new_close = bkt.h1.close_time.clone() + H1_AS_NANOS;
                                v.h1_start_next_bar(bkt.h1.close_time, new_close, bkt.h1.close.clone());
                            }
                            // update existing bar
                            if &ov1.snapshot_time >= &bkt.h1.open_time && &ov1.snapshot_time <= &bkt.h1.close_time {
                                // catch first quote since init
                                if &bkt.h1.open.cross_price == &0.0 {
                                    v.h1.open = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd };
                                }
                                // update high
                                if &ov1.average_price > &bkt.h1.high.cross_price { 
                                    v.h1.high = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd }
                                }
                                // update low
                                if &ov1.average_price < &bkt.h1.low.cross_price { 
                                    v.h1.low = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd }
                                }
                                // update close
                                v.h1.close = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd };
                            }

                        // D1 ACTIONS
                            // new bar required 
                            if &ov1.snapshot_time > &bkt.d1.close_time {
                                // push to btree store
                                if let Some(mut price_store) = self.price_data_tree.price_data.get_mut(&id_ref) {
                                    price_store.d1.push(bkt.d1.clone()).expect("Storage is full");
                                    if &price_store.d1.len() > &D1_MAX { price_store.d1.remove(0); }
                                }
                                // start next bar
                                let new_close = bkt.d1.close_time.clone() + D1_AS_NANOS;
                                v.d1_start_next_bar(bkt.d1.close_time, new_close, bkt.d1.close.clone());
                            }
                            // update existing bar
                            if &ov1.snapshot_time >= &bkt.d1.open_time && &ov1.snapshot_time <= &bkt.d1.close_time {
                                // catch first quote since init
                                if &bkt.d1.open.cross_price == &0.0 {
                                    v.d1.open = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd };
                                }
                                // update high
                                if &ov1.average_price > &bkt.d1.high.cross_price { 
                                    v.d1.high = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd }
                                }
                                // update low
                                if &ov1.average_price < &bkt.d1.low.cross_price { 
                                    v.d1.low = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd }
                                }
                                // update close
                                v.d1.close = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd };
                            }

                        // W1 ACTIONS
                            // new bar required 
                            if &ov1.snapshot_time > &bkt.w1.close_time {
                                // push to btree store
                                if let Some(mut price_store) = self.price_data_tree.price_data.get_mut(&id_ref) {
                                    price_store.w1.push(bkt.w1.clone()).expect("Storage is full");
                                    if &price_store.w1.len() > &W1_MAX { price_store.w1.remove(0); }
                                }
                                // start next bar
                                let new_close = bkt.w1.close_time.clone() + W1_AS_NANOS;
                                v.w1_start_next_bar(bkt.w1.close_time, new_close, bkt.w1.close.clone());
                            }
                            // update existing bar
                            if &ov1.snapshot_time >= &bkt.w1.open_time && &ov1.snapshot_time <= &bkt.w1.close_time {
                                // catch first quote since init
                                if &bkt.w1.open.cross_price == &0.0 {
                                    v.w1.open = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd };
                                }
                                // update high
                                if &ov1.average_price > &bkt.w1.high.cross_price { 
                                    v.w1.high = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd }
                                }
                                // update low
                                if &ov1.average_price < &bkt.w1.low.cross_price { 
                                    v.w1.low = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd }
                                }
                                // update close
                                v.w1.close = PriceTuple{ cross_price: ov1.average_price.clone(), usd_price: ov1.average_price*ov1.cross_to_usd };
                            }
                        }// get mut bucket
                    }// av price > 0
                }
                pos += 1;
            }
        }
    }

    pub fn get_prices_from_processing_bucket(&self, cross: String) -> Option<OHLCBucket> {
        if cross.chars().count() > 15 { ic_cdk::trap("Crosses cannot be more than 15 characters") }
        let cross_id = IDKey::from_string(cross).unwrap(); // all keys from OverviewV1 will be less than 15 chars. Unwrap is ok
        for bkt in self.processing_bucket.iter() {
            if cross_id.0 == *bkt.cross.0 {
                return Some(bkt.clone());
            }
        }
        None
    }

    // Adds to store AND processing bucket
    pub fn add_cross_to_store(&mut self, cross: String, cross_ref: u64, process_from: u64){
        if cross.chars().count() > 15 { ic_cdk::trap("Crosses cannot be more than 15 characters") }
        let cross_id = IDKey::from_string(cross.clone()).unwrap();
        let pd = PriceData::init(cross_id);
    
        // add to btree map/ store
        self.price_data_tree.price_data.insert(cross_ref, pd).expect("Storage is full");
        
        // add to processing array
        self.add_cross_to_processing_bucket(cross, process_from);
    }

    // Remove cross from store AND processing bucket
    pub fn remove_cross_from_store(&mut self, cross: String) -> String {
        if cross.chars().count() > 15 { ic_cdk::trap("Crosses cannot be more than 15 characters") };
        let cross_ref = self.directory_data.get_ref(cross.clone());
        match cross_ref {
            Some(v) => {
                self.price_data_tree.price_data.remove(&v);
                self.remove_cross_from_processing_bucket(cross);
                return "Cross Removed from store and processing bucket".to_string();
            },
            None => { return "Could not find cross, nothing removed".to_string() }
        }
    }

    pub fn get_m5_price_data(&self, cross: String, len: usize) -> Option<Vec<OHLC>> {
        if len == 0 {return None;}
        let cross_ref = self.directory_data.get_ref(cross.clone());
        match cross_ref {
            Some(v) => {
                if let Some(data) = self.price_data_tree.price_data.get(&v){
                    let mut count = len;
                    if len > M5_MAX { count = M5_MAX }
                    let mut ret_vec: Vec<OHLC> = Vec::new();
                    // get latest (forming) bar
                    let forming = self.get_prices_from_processing_bucket(cross);
                    match forming {
                        Some(fv) => {
                            if len == 1 {
                                ret_vec.push(fv.m5);
                                return Some(ret_vec);
                            } 

                            let mut m5 = data.get_all_m5();
                            m5.push(fv.m5); // latest bar
                            m5.reverse();
                            if count > m5.len() { count = m5.len() };
                            for i in 0..count {
                                ret_vec.push(m5[i].clone());
                            }
                            return Some(ret_vec);
                        },  
                        None => { return None;} // no data in processing bucket
                    }
                }
                return None; // if nothing found in map
            },
            None => { return None} // no cross ref
        }
    }

    pub fn get_m15_price_data(&self, cross: String, len: usize) -> Option<Vec<OHLC>> {
        if len == 0 {return None;}
        let cross_ref = self.directory_data.get_ref(cross.clone());
        match cross_ref {
            Some(v) => {
                if let Some(data) = self.price_data_tree.price_data.get(&v){
                    let mut count = len;
                    if len > M15_MAX { count = M15_MAX }
                    let mut ret_vec: Vec<OHLC> = Vec::new();
                    // get latest (forming) bar
                    let forming = self.get_prices_from_processing_bucket(cross);
                    match forming {
                        Some(fv) => {
                            if len == 1 {
                                ret_vec.push(fv.m15);
                                return Some(ret_vec);
                            } 

                            let mut m15 = data.get_all_m5();
                            m15.push(fv.m15); // latest bar
                            m15.reverse();
                            if count > m15.len() { count = m15.len() };
                            for i in 0..count {
                                ret_vec.push(m15[i].clone());
                            }
                            return Some(ret_vec);
                        },  
                        None => { return None;} // no data in processing bucket
                    }
                }
                return None; // if nothing found in map
            },
            None => { return None} // no cross ref
        }
    }

    pub fn get_h1_price_data(&self, cross: String, len: usize) -> Option<Vec<OHLC>> {
        if len == 0 {return None;}
        let cross_ref = self.directory_data.get_ref(cross.clone());
        match cross_ref {
            Some(v) => {
                if let Some(data) = self.price_data_tree.price_data.get(&v){
                    let mut count = len;
                    if len > H1_MAX { count = H1_MAX }
                    let mut ret_vec: Vec<OHLC> = Vec::new();
                    // get latest (forming) bar
                    let forming = self.get_prices_from_processing_bucket(cross);
                    match forming {
                        Some(fv) => {
                            if len == 1 {
                                ret_vec.push(fv.h1);
                                return Some(ret_vec);
                            } 

                            let mut h1 = data.get_all_h1();
                            h1.push(fv.h1); // latest bar
                            h1.reverse();
                            if count > h1.len() { count = h1.len() };
                            for i in 0..count {
                                ret_vec.push(h1[i].clone());
                            }
                            return Some(ret_vec);
                        },  
                        None => { return None;} // no data in processing bucket
                    }
                }
                return None; // if nothing found in map
            },
            None => { return None} // no cross ref
        }
    }

    pub fn get_d1_price_data(&self, cross: String, len: usize) -> Option<Vec<OHLC>> {
        if len == 0 {return None;}
        let cross_ref = self.directory_data.get_ref(cross.clone());
        match cross_ref {
            Some(v) => {
                if let Some(data) = self.price_data_tree.price_data.get(&v){
                    let mut count = len;
                    if len > D1_MAX { count = D1_MAX }
                    let mut ret_vec: Vec<OHLC> = Vec::new();
                    // get latest (forming) bar
                    let forming = self.get_prices_from_processing_bucket(cross);
                    match forming {
                        Some(fv) => {
                            if len == 1 {
                                ret_vec.push(fv.h1);
                                return Some(ret_vec);
                            } 

                            let mut h1 = data.get_all_h1();
                            h1.push(fv.h1); // latest bar
                            h1.reverse();
                            if count > h1.len() { count = h1.len() };
                            for i in 0..count {
                                ret_vec.push(h1[i].clone());
                            }
                            return Some(ret_vec);
                        },  
                        None => { return None;} // no data in processing bucket
                    }
                }
                return None; // if nothing found in map
            },
            None => { return None} // no cross ref
        }
    }

    pub fn get_w1_price_data(&self, cross: String, len: usize) -> Option<Vec<OHLC>> {
        if len == 0 {return None;}
        let cross_ref = self.directory_data.get_ref(cross.clone());
        match cross_ref {
            Some(v) => {
                if let Some(data) = self.price_data_tree.price_data.get(&v){
                    let mut count = len;
                    if len > W1_MAX { count = W1_MAX }
                    let mut ret_vec: Vec<OHLC> = Vec::new();
                    // get latest (forming) bar
                    let forming = self.get_prices_from_processing_bucket(cross);
                    match forming {
                        Some(fv) => {
                            if len == 1 {
                                ret_vec.push(fv.w1);
                                return Some(ret_vec);
                            } 

                            let mut w1 = data.get_all_h1();
                            w1.push(fv.w1); // latest bar
                            w1.reverse();
                            if count > w1.len() { count = w1.len() };
                            for i in 0..count {
                                ret_vec.push(w1[i].clone());
                            }
                            return Some(ret_vec);
                        },  
                        None => { return None;} // no data in processing bucket
                    }
                }
                return None; // if nothing found in map
            },
            None => { return None} // no cross ref
        }
    }

    pub fn get_all_crosses(&self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut cross: String;
        for bkt in self.processing_bucket.iter(){
            cross = bkt.cross.to_string().unwrap();
            res.push(cross);
        }
        return res;
    }
}

   


