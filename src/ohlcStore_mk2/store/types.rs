use candid::CandidType;
use ic_stable_memory::{collections::SVec, derive::{AsFixedSizeBytes, StableType}};
use serde::Deserialize;
use crate::{core::types::IDKey, timers::constants::{D1_AS_NANOS, H1_AS_NANOS, M15_AS_NANOS, M5_AS_NANOS, W1_AS_NANOS}};

#[derive(CandidType, Deserialize, StableType, Default, AsFixedSizeBytes, Debug, Clone)]
pub struct PriceTuple{
    pub cross_price: f64,
    pub usd_price: f64
}

#[derive(CandidType, Deserialize, StableType, Default, AsFixedSizeBytes, Debug, Clone)]
pub struct OHLC {
    pub open_time: u64, // 8 bytes
    pub close_time: u64, // 8 bytes
    pub open: PriceTuple, // 16 bytes
    pub high: PriceTuple, // 16 bytes
    pub low:  PriceTuple, // 16 bytes
    pub close:PriceTuple, // 16 bytes
    pub volume: u64  // 8 bytes
} // total 88 bytes
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


#[derive(StableType, Default, AsFixedSizeBytes, Debug)]
pub struct PriceData {
    cross: IDKey, // max 15 chars.
    active_from: u64,
    last_update: u64,
    pub m5: SVec<OHLC>,  // 8640 bars 30 days
    pub m15: SVec<OHLC>, // 5760 bars 60 days
    pub h1: SVec<OHLC>,  // 2880 bars 120 days
    pub d1: SVec<OHLC>,  // 1780 bars 5 years
    pub w1: SVec<OHLC>   // 520 bars 10 years
} 

impl PriceData {
    pub fn init(cross: IDKey) -> PriceData {
        return PriceData{ 
            cross, 
            active_from: 0, 
            last_update: 0, 
            m5:  SVec::new(), 
            m15: SVec::new(), 
            h1:  SVec::new(), 
            d1:  SVec::new(), 
            w1:  SVec::new(), 
        };
    }

    pub fn get_cross(&self) -> String {
        self.cross.to_string().unwrap()
    }

    pub fn get_all_m5(&self) -> Vec<OHLC> {
        let mut output: Vec<OHLC> = Vec::new();
        for ohlc in self.m5.iter() {
            output.push(ohlc.clone());
        }
        return output;
    }

    pub fn count_m5_bars(&self) -> usize {
        self.m5.len()
    }

    pub fn get_all_m15(&self) -> Vec<OHLC> {
        let mut output: Vec<OHLC> = Vec::new();
        for ohlc in self.m15.iter() {
            output.push(ohlc.clone());
        }
        return output;
    }

    pub fn count_m15_bars(&self) -> usize {
        self.m15.len()
    }

    pub fn get_all_h1(&self) -> Vec<OHLC> {
        let mut output: Vec<OHLC> = Vec::new();
        for ohlc in self.h1.iter() {
            output.push(ohlc.clone());
        }
        return output;
    }

    pub fn count_h1_bars(&self) -> usize {
        self.h1.len()
    }

    pub fn get_all_d1(&self) -> Vec<OHLC> {
        let mut output: Vec<OHLC> = Vec::new();
        for ohlc in self.d1.iter() {
            output.push(ohlc.clone());
        }
        return output;
    }

    pub fn count_d1_bars(&self) -> usize {
        self.d1.len()
    }

    pub fn get_all_w1(&self) -> Vec<OHLC> {
        let mut output: Vec<OHLC> = Vec::new();
        for ohlc in self.w1.iter() {
            output.push(ohlc.clone());
        }
        return output;
    }

    pub fn count_w1_bars(&self) -> usize {
        self.w1.len()
    }
}

#[derive(StableType, AsFixedSizeBytes, Debug, Default, Clone)]
pub struct OHLCBucket {
    pub cross: IDKey,
    pub m5: OHLC,
    pub m15: OHLC,
    pub h1: OHLC,
    pub d1: OHLC,
    pub w1: OHLC
}
impl OHLCBucket {
    pub fn init(cross: IDKey, start_time: u64) -> OHLCBucket {
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
        self.m5.open = open_price.clone();
        self.m5.high = open_price.clone();
        self.m5.low = open_price.clone();
        self.m5.close = open_price;
        self.m5.volume = 0;
    }

    pub fn m15_start_next_bar(&mut self, open_time: u64, close_time: u64, open_price: PriceTuple){
        self.m15.open_time = open_time;
        self.m15.close_time = close_time;
        self.m15.open = open_price.clone();
        self.m15.high = open_price.clone();
        self.m15.low = open_price.clone();
        self.m15.close = open_price;
        self.m15.volume = 0;
    }

    pub fn h1_start_next_bar(&mut self, open_time: u64, close_time: u64, open_price: PriceTuple){
        self.h1.open_time = open_time;
        self.h1.close_time = close_time;
        self.h1.open = open_price.clone();
        self.h1.high = open_price.clone();
        self.h1.low = open_price.clone();
        self.h1.close = open_price;
        self.h1.volume = 0;
    }

    pub fn d1_start_next_bar(&mut self, open_time: u64, close_time: u64, open_price: PriceTuple){
        self.d1.open_time = open_time;
        self.d1.close_time = close_time;
        self.d1.open = open_price.clone();
        self.d1.high = open_price.clone();
        self.d1.low = open_price.clone();
        self.d1.close = open_price;
        self.d1.volume = 0;
    }

    pub fn w1_start_next_bar(&mut self, open_time: u64, close_time: u64, open_price: PriceTuple){
        self.w1.open_time = open_time;
        self.w1.close_time = close_time;
        self.w1.open = open_price.clone();
        self.w1.high = open_price.clone();
        self.w1.low = open_price.clone();
        self.w1.close = open_price;
        self.w1.volume = 0;
    }  
}