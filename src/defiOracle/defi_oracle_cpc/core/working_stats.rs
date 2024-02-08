use candid::CandidType;
use defi_oracle_shared::shared_types::SwapPair;
use serde::{Deserialize, Serialize};

use crate::cpc::utils::AltQuotes;

use super::runtime::RUNTIME_STATE;

#[derive(CandidType, Deserialize, Serialize, Default)]
pub struct WorkingStats {
    pub timer_active: bool,
    pub last_update_time: (u64, u64), // (main quotes, usd/xdr quotes)
    pub metrics: Metrics,
    quote_trade_size: (f64, AltQuotes)
}

impl WorkingStats {
    pub fn set_trade_size(&mut self, size: f64, quote_currency: AltQuotes) -> String {
        self.quote_trade_size = (size, quote_currency);
        return "Quote trade size has been updated".to_string();
    }

    pub fn get_trade_size(&self) -> f64 {
        return self.quote_trade_size.0;
    }

    pub fn get_trade_quote_currency(&self) -> AltQuotes {
        return self.quote_trade_size.1.clone();
    }

    pub fn get_metrics(&self) -> Metrics {
        return self.metrics.clone();
    }
}

#[derive(CandidType, Deserialize, Serialize, Default, Clone)]
pub struct Metrics {
    total_snapshots_taken: u64,
    total_http_outcalls: u64,
    total_errors: u64,
    total_api_requests_v1: u64,
}

impl Metrics {
    pub fn increment_snapshots_taken(&mut self, calls: u64){
        self.total_snapshots_taken += calls;
    }

    pub fn increment_http_outcalls(&mut self){
        self.total_http_outcalls +=1;
    }

    pub fn increment_total_errors(&mut self){
        self.total_errors += 1;
    }

    pub fn increment_total_api_v1(&mut self){
        self.total_api_requests_v1 += 1;
    }
}

pub fn count_api(){
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.metrics.increment_total_api_v1()
    });
}

pub fn count_error(){
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().stats.metrics.increment_total_errors()
    });
}