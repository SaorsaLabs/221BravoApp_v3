use std::borrow::Cow;
use candid::{CandidType, Encode, Decode};
use ic_stable_structures::{Storable, storable::Bound};
use serde::{Deserialize, Serialize};

use super::constants::{M5_MAX, M15_MAX, H1_MAX, D1_MAX, W1_MAX};

#[derive(CandidType, Deserialize, Serialize, Clone, Default, Debug, Copy)]
pub struct PriceTuple{
    pub cross_price: f64,
    pub usd_price: f64
}

#[derive(CandidType, Deserialize, Serialize, Clone, Default, Debug)]
pub struct OHLC {
    pub open_time: u64, // 8 bytes
    pub close_time: u64, // 8 bytes
    pub open: PriceTuple, // 16 bytes
    pub high: PriceTuple, // 16 bytes
    pub low:  PriceTuple, // 16 bytes
    pub close:PriceTuple, // 16 bytes
    pub volume: u64  // 8 bytes
} // total 88 bytes
impl Storable for OHLC {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 88,
        is_fixed_size: true,
    };
}
impl OHLC {
    pub fn new(
        open_time: u64, 
        close_time: u64,
        open_price: PriceTuple,
        high_price: PriceTuple,
        low_price: PriceTuple,
        close_price: PriceTuple,
        volume: u64
    ) -> OHLC {
        let ret = OHLC { 
            open_time, 
            close_time, 
            open: open_price, 
            high: high_price, 
            low: low_price, 
            close: close_price, 
            volume, 
        };
        return ret;
    }
    
    pub fn update_open(&mut self, price: f64, usd_price: f64){
        self.open = PriceTuple{ cross_price: price, usd_price};
    }

    pub fn update_high(&mut self, price: f64, usd_price: f64){
        self.high = PriceTuple{ cross_price: price, usd_price};
    }

    pub fn update_low(&mut self, price: f64, usd_price: f64){
        self.low = PriceTuple{ cross_price: price, usd_price};
    }

    pub fn update_close(&mut self, price: f64, usd_price: f64){
        self.close = PriceTuple{ cross_price: price, usd_price};
    }

    pub fn add_volume(&mut self, volume: u64){
        let total = self.volume.saturating_add(volume);
        self.volume = total;
    }
}


#[derive(CandidType, Deserialize, Clone, Default, Debug)]
pub struct PriceData {
    cross: String, // max 15 chars.
    active_from: u64,
    last_update: u64,
    pub m5: Vec<OHLC>,  // 8640 bars 30 days
    pub m15: Vec<OHLC>, // 2880 bars 30 days
    pub h1: Vec<OHLC>,  // 1440 bars 60 days
    pub d1: Vec<OHLC>,  // 1780 bars 5 years
    pub w1: Vec<OHLC>   // 520 bars 10 years
} // TOTAL Size in bytes = 1_343_100 byes


impl Storable for PriceData {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 1_343_100,
        is_fixed_size: false,
    };
}

impl PriceData{
    pub fn push_m5(&mut self, data: OHLC){
        if &self.m5.len() >= &M5_MAX { self.m5.remove(0);} // drop oldest entry if full
        self.m5.push(data);
    }
    pub fn push_m15(&mut self, data: OHLC){
        if &self.m15.len() >= &M15_MAX { self.m15.remove(0);} // drop oldest entry if full
        self.m15.push(data);
    }
    pub fn push_h1(&mut self, data: OHLC){
        if &self.h1.len() >= &H1_MAX { self.h1.remove(0);} // drop oldest entry if full
        self.h1.push(data);
    }
    pub fn push_d1(&mut self, data: OHLC){
        if &self.d1.len() >= &D1_MAX { self.d1.remove(0);} // drop oldest entry if full
        self.d1.push(data);
    }
    pub fn push_w1(&mut self, data: OHLC){
        if &self.w1.len() >= &W1_MAX { self.w1.remove(0);} // drop oldest entry if full
        self.w1.push(data);
    }

    pub fn get_all_m5(&self) -> Vec<OHLC> {
        return self.m5.clone();
    }

    pub fn get_m5(&self, bars: usize) -> Vec<OHLC> {
        let last_entry = self.m5.len();
        if bars >= last_entry {
            return self.m5.clone();
        } else {
            let mut ret:Vec<OHLC> = Vec::with_capacity(bars);
            let start = last_entry - bars;
            for i in start..last_entry{
                ret.push(self.m5[i].clone());
            }
            return ret;
        }
    }

    pub fn get_all_m15(&self) -> Vec<OHLC> {
        return self.m15.clone();
    }

    pub fn get_m15(&self, bars: usize) -> Vec<OHLC> {
        let last_entry = self.m15.len();
        if bars >= last_entry {
            return self.m15.clone();
        } else {
            let mut ret:Vec<OHLC> = Vec::with_capacity(bars);
            let start = last_entry - bars;
            for i in start..last_entry{
                ret.push(self.m15[i].clone());
            }
            return ret;
        }
    }

    pub fn get_all_h1(&self) -> Vec<OHLC> {
        return self.h1.clone();
    }

    pub fn get_h1(&self, bars: usize) -> Vec<OHLC> {
        let last_entry = self.h1.len();
        if bars >= last_entry {
            return self.h1.clone();
        } else {
            let mut ret:Vec<OHLC> = Vec::with_capacity(bars);
            let start = last_entry - bars;
            for i in start..last_entry{
                ret.push(self.h1[i].clone());
            }
            return ret;
        }
    }

    pub fn get_all_d1(&self) -> Vec<OHLC> {
        return self.d1.clone();
    }

    pub fn get_d1(&self, bars: usize) -> Vec<OHLC> {
        let last_entry = self.d1.len();
        if bars >= last_entry {
            return self.d1.clone();
        } else {
            let mut ret:Vec<OHLC> = Vec::with_capacity(bars);
            let start = last_entry - bars;
            for i in start..last_entry{
                ret.push(self.d1[i].clone());
            }
            return ret;
        }
    }

    pub fn get_all_w1(&self) -> Vec<OHLC> {
        return self.w1.clone();
    }

    pub fn get_w1(&self, bars: usize) -> Vec<OHLC> {
        let last_entry = self.w1.len();
        if bars >= last_entry {
            return self.w1.clone();
        } else {
            let mut ret:Vec<OHLC> = Vec::with_capacity(bars);
            let start = last_entry - bars;
            for i in start..last_entry{
                ret.push(self.w1[i].clone());
            }
            return ret;
        }
    }
}

pub fn init_new_price_data(cross: String) -> PriceData {
    let new = PriceData{
        cross,
        active_from: 0,
        last_update: 0,
        m5: Vec::new(),
        m15: Vec::new(),
        h1: Vec::new(),
        d1: Vec::new(),
        w1: Vec::new(),
    };
    return new;
}