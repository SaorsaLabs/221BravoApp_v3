use ic_stable_memory::{collections::SBTreeMap, derive::{AsFixedSizeBytes, StableType}};
use super::{constants::{D1_MAX, H1_MAX, M15_MAX, M5_MAX, W1_MAX}, types::{PriceData, OHLC}};

#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct PriceDataTree{
    pub price_data: SBTreeMap<u64, PriceData>,
    count: u64,
    last_updated: u64,
}

impl PriceDataTree {
    pub fn push_m5_to_store(&mut self, cross_ref: u64, data: OHLC){
        if let Some(mut tree) = self.price_data.get_mut(&cross_ref) {
            tree.m5.push(data).expect("Storage Full");
            if &tree.m5.len() > &M5_MAX { tree.m5.remove(0); }
        }
    }

    pub fn import_m5_history(&mut self, cross_ref: u64, data: Vec<OHLC>){
        if let Some(mut tree) = self.price_data.get_mut(&cross_ref) {
            if &data.len() > &M5_MAX { ic_cdk::trap("M5 import data is > M5_MAX"); }
            for ohlc in data {
                tree.m5.push(ohlc).expect("Storage Full");
            }
        }
    }

    pub fn push_m15_to_store(&mut self, cross_ref: u64, data: OHLC){
        if let Some(mut tree) = self.price_data.get_mut(&cross_ref) {
            tree.m15.push(data).expect("Storage Full");
            if &tree.m15.len() > &M15_MAX { tree.m15.remove(0); }
        }
    }

    pub fn import_m15_history(&mut self, cross_ref: u64, data: Vec<OHLC>){
        if let Some(mut tree) = self.price_data.get_mut(&cross_ref) {
            if &data.len() > &M15_MAX { ic_cdk::trap("M15 import data is > M15_MAX"); }
            for ohlc in data {
                tree.m15.push(ohlc).expect("Storage Full");
            }
        }
    }

    pub fn push_h1_to_store(&mut self, cross_ref: u64, data: OHLC){
        if let Some(mut tree) = self.price_data.get_mut(&cross_ref) {
            tree.h1.push(data).expect("Storage Full");
            if &tree.h1.len() > &H1_MAX { tree.h1.remove(0); }
        }
    }

    pub fn import_h1_history(&mut self, cross_ref: u64, data: Vec<OHLC>){
        if let Some(mut tree) = self.price_data.get_mut(&cross_ref) {
            if &data.len() > &H1_MAX { ic_cdk::trap("H1 import data is > H1_MAX"); }
            for ohlc in data {
                tree.h1.push(ohlc).expect("Storage Full");
            }
        }
    }

    pub fn push_d1_to_store(&mut self, cross_ref: u64, data: OHLC){
        if let Some(mut tree) = self.price_data.get_mut(&cross_ref) {
            tree.d1.push(data).expect("Storage Full");
            if &tree.d1.len() > &D1_MAX { tree.d1.remove(0); }
        }
    }

    pub fn import_d1_history(&mut self, cross_ref: u64, data: Vec<OHLC>){
        if let Some(mut tree) = self.price_data.get_mut(&cross_ref) {
            if &data.len() > &D1_MAX { ic_cdk::trap("D1 import data is > D1_MAX"); }
            for ohlc in data {
                tree.d1.push(ohlc).expect("Storage Full");
            }
        }
    }

    pub fn push_w1_to_store(&mut self, cross_ref: u64, data: OHLC){
        if let Some(mut tree) = self.price_data.get_mut(&cross_ref) {
            tree.w1.push(data).expect("Storage Full");
            if &tree.w1.len() > &W1_MAX { tree.w1.remove(0); }
        }
    }

    pub fn import_w1_history(&mut self, cross_ref: u64, data: Vec<OHLC>){
        if let Some(mut tree) = self.price_data.get_mut(&cross_ref) {
            if &data.len() > &W1_MAX { ic_cdk::trap("W1 import data is > W1_MAX"); }
            for ohlc in data {
                tree.w1.push(ohlc).expect("Storage Full");
            }
        }
    }
}


