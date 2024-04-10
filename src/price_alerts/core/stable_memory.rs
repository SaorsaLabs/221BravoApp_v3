use std::cell::RefCell;
use candid::CandidType;
use ic_stable_memory::{collections::SVec, derive::{AsFixedSizeBytes, StableType}};
use crate::price_alerts::btree::PriceDataTree;

use super::types::IDKey;


thread_local! {
    pub static STABLE_STATE: RefCell<Option<Main>> = RefCell::default();
}

#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct Main {
    pub previous_prices:  SVec<PriceTuple>,
    pub alert_data: PriceDataTree
}

impl Main {
    pub fn overwrite_stored_prices(&mut self, new_prices: Vec<PriceTuple>){
        self.previous_prices.clear();
        for np in new_prices.iter(){
            self.previous_prices.push(np.clone()).expect("Memory is Full");
        }
    }
    pub fn get_all_previous_prices(&self) -> Vec<PriceTuple> {
        let mut ret_vec: Vec<PriceTuple> = Vec::new();
        for pr in self.previous_prices.iter() {
            ret_vec.push(pr.clone())
        }
        ret_vec
    }
}

#[derive(CandidType, StableType, AsFixedSizeBytes, Debug, Default, Clone)]
pub struct PriceTuple{
    pub cross: IDKey,
    pub price: f64
}

   


