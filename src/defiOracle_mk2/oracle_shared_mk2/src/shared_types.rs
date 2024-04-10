use core::fmt;
use std::default;

use candid::{CandidType, Int};
use serde::{Serialize, Deserialize};

// [][] -- Index of tokens -- [][]
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct TokenSwaps{
    pub available_swaps: Vec<SwapPairDetails>,
}
impl TokenSwaps {
    pub fn add_swap_pair ( &mut self, details: SwapPairDetails ) -> String {
        // check if exists already
        for sp in self.available_swaps.clone() {
            if sp.swap_id == details.swap_id {
                return String::from("Swap already exists");
            }
        }

        self.available_swaps.push(details);
        return String::from("Swap pair added");
    }

    pub fn remove_swap_pair (&mut self, swap_id: String) -> String {
        self.available_swaps.retain(|x| x.swap_id != swap_id);
        return "Removed any entries which matched the input".to_string();
    }

    pub fn get_all_swap_pairs (&self) -> Vec<String> {
        let mut ret_vec = Vec::new();
        for p in self.available_swaps.clone() {
            ret_vec.push(p.swap_id.to_string());
        }
        return ret_vec;
    }

    pub fn get_single_swap_pair(&self, swap_id: String ) -> Option<SwapPairDetails> {
        for swp in self.available_swaps.clone() {
            if swp.swap_id == swap_id {
                return Some(swp);
            }
        }
        return None;
    }

    pub fn clear_all_swaps(&mut self){
        self.available_swaps.clear();
    }

    pub fn set_swap_status(&mut self, swap_id: String, status: bool) -> String {
        // check if exists already
        for sp in self.available_swaps.iter_mut() {
            if sp.swap_id == swap_id {
                sp.active = status;
                return String::from("Swap status updated");
            }
        }
        return String::from("Swap ID could not be found");
    }

    pub fn add_marketplace(&mut self, swap_id: String, marketplace: MarketplaceDetails) -> String {
        // check if exists already
        for sp in self.available_swaps.iter_mut() {
            if sp.swap_id == swap_id {
                // check exists already
                for mk in sp.marketplaces.iter(){
                    if &marketplace.marketplace == &mk.marketplace {
                        return String::from("Marketplace already exists on swap");
                    }
                }
                sp.marketplaces.push(marketplace);
                return String::from("Marketplace added to swap");
            }
        }
        return String::from("Swap ID could not be found");        
    }

    pub fn remove_marketplace(&mut self, swap_id: String, marketplace: Marketplace) -> String {
        // check if exists already
        for sp in self.available_swaps.iter_mut() {
            if sp.swap_id == swap_id {
                sp.marketplaces.retain(|x| x.marketplace != marketplace);
                return "Removed any marketplaces which matched the input".to_string(); 
            }
        }
        return String::from("Swap ID could not be found");        
    }

    pub fn set_marketplace_status(&mut self, swap_id: String, marketplace: Marketplace, status: bool) -> String {
        // check if exists already
        for sp in self.available_swaps.iter_mut() {
            if sp.swap_id == swap_id {
                // check exists already
                for mk in sp.marketplaces.iter_mut(){
                    if &marketplace == &mk.marketplace {
                        mk.active = status;
                        return String::from("Marketplace status updated");
                    }
                }
                return String::from("Marketplace could not be found");
            }
        }
        return String::from("Swap ID could not be found");    
    }

    pub fn get_all_swap_marketplaces(&self, swap_id: String) -> Option<Vec<MarketplaceDetails>> {
        for sp in self.available_swaps.iter() {
            if &swap_id == &sp.swap_id {
                let ret = sp.marketplaces.clone();
                return Some(ret);
            }
        }
        None
    }

}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SwapPairDetails{
    pub swap_id: String,
    pub token0: Token,
    pub token1: Token,
    pub marketplaces: Vec<MarketplaceDetails>,
    pub swap_type: u8,
    pub active: bool
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub ticker: String, 
    pub ledger: String, 
    pub decimals: u32
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MarketplaceDetails{
    pub marketplace: Marketplace,
    pub canister_id: String,
    pub active: bool,
    pub reverse_cross: bool,
    pub unit_size: u64
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum Marketplace {
    #[default]
    ICDEX,
    ICPSWAP,
    SONIC
}
impl fmt::Display for Marketplace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Marketplace::ICDEX => "ICDEX",
            Marketplace::ICPSWAP => "ICPSWAP",
            Marketplace::SONIC => "SONIC",
        };
        write!(f, "{}", name)
    }
}

impl Marketplace {
    pub fn to_string(&self) -> String {
        match self {
            Marketplace::ICDEX => "IC Dex".to_string(),
            Marketplace::ICPSWAP => "ICP Swap".to_string(),
            Marketplace::SONIC => "Sonic".to_string(),
        }
    }
}


#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TokenStandard {
    ICRC1,
    ICRC2
}

//[][] -- WORKING/ PROCESSING STRUCTS -- [][]
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TokenOverview {
    V1(OverviewV1)
}
impl TokenOverview {
    pub fn get_v1_data(&self) -> Option<OverviewV1> {
        match self {
            TokenOverview::V1(data) => Some(data.clone()),
            _ => None
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverviewV1{
    pub token_cross: String, 
    pub snapshot_time: u64,
    pub average_price: f64,
    pub exchange_snapshots: Vec<ExchangeSnapshot>,
    pub cross_to_usd: f64
}

#[derive(Clone, Debug, PartialEq, CandidType, Serialize, Deserialize)]
pub enum StableCurrency{
    XDR,
    USD,
    ICP,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExchangeSnapshot{
    pub snapshot_time: u64,
    pub swap_pair: String,
    pub exchange: Marketplace,
    pub price: f64,
    pub bid: u128,
    pub ask: u128,
    pub spread_pct: f64,
    pub liquidity: (u64,u64) // (bid, ask)
}
impl fmt::Display for ExchangeSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ExchangeSnapshot {{ snapshot_time: {}, swap_pair: {}, exchange: {}, price: {}, bid: {}, ask: {}, liquidity: ({}, {}) }}",
            self.snapshot_time,
            self.swap_pair,
            self.exchange,
            self.price,
            self.bid,
            self.ask,
            self.liquidity.0,
            self.liquidity.1
        )
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InternalRateEntry {
    pub swap_pair: String, 
    pub quote: f64,
    pub timestamp: u64
}
// this struct should match enum swap pair! 
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct InternalRates {
    pub data: Vec<InternalRateEntry>,
}

impl InternalRates {
    pub fn add_swap_pair_to_vec(&mut self, data: InternalRateEntry){
        // check if already added
        for ire in self.data.iter() {
            if data.swap_pair == ire.swap_pair {
                return;
            }
        }
        self.data.push(data);
    }

    pub fn does_swap_exist(&self, swap_pair: String) -> bool {
        for qt in &self.data {
            if qt.swap_pair == swap_pair { return true; }
        }
        return false;
    }

    pub fn update_all_quotes(&mut self, rates_vec: Vec<InternalRateEntry>){
        // Check for new entries and add if needed
        if self.data.len() == 0 {
            self.data = rates_vec.clone();
        } else {
            for entry in rates_vec.iter() {
                let mut exists = false;
                for existing in self.data.iter() {
                    if entry.swap_pair == existing.swap_pair {
                        exists = true;
                    }
                }
                if exists == false {
                    self.data.push(entry.clone());
                }
            }
        }

        // update any existing entries
        for old_qte in self.data.iter_mut() {
            for new_qte in rates_vec.iter() {
                if &old_qte.swap_pair == &new_qte.swap_pair {
                    old_qte.quote = new_qte.quote;
                    old_qte.timestamp = new_qte.timestamp;
                }
            }
        }
    }

    pub fn update_single_quote(&mut self, swap_pair: &String, quote: f64, timestamp: u64){
        // check if already added and then updaate
        for ire in self.data.iter_mut() {
            if swap_pair == &ire.swap_pair {
                ire.quote = quote;
                ire.timestamp = timestamp;
                return
            }
        }
        let time = ic_cdk::api::time();
        // else add quote
        self.data.push(InternalRateEntry{swap_pair: swap_pair.clone(), quote, timestamp: time});
    }

    pub fn get_all_rates_vec(&self) -> Vec<InternalRateEntry> {
        return self.data.clone();
    }

    pub fn get_single_rate(&self, swap_pair: &String) -> Option<f64> {
        for ire in self.data.iter() {
            if swap_pair == &ire.swap_pair {
                return Some(ire.quote);
            }
        }
        return None;
    }

    pub fn clear_swap_pair_vec(&mut self){
        let vec: Vec<InternalRateEntry> = Vec::new();
        self.data = vec;
    }

    pub fn remove_swap_pair_from_vec(&mut self, swap_pair: String) -> String {
        self.data.retain(|x: &InternalRateEntry| x.swap_pair != swap_pair);
        return String::from("Any matching values have been removed");
    }

}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct QuoteBucket{
    pub swap_pair: String,
    pub snapshots: Vec<ExchangeSnapshot>
}
impl fmt::Display for QuoteBucket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let snapshots_str: Vec<String> = self.snapshots.iter().map(|snapshot| format!("{}", snapshot)).collect();
        write!(f, "QuoteBucket {{ snapshots: [{}] }}", snapshots_str.join(", "))
    }
}

// [][] -- OUTPUT STRUCTS -- [][]
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SwapPairResult{
    age: u64,
    data: TokenOverview,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ChangeStatusArgs{
    pub token: String, 
    pub status: bool
}