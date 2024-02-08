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
    pub fn add_swap_pair (
        &mut self,
        details: SwapPairDetails,
    ) -> String {

        // check if exists already
        for sp in self.available_swaps.clone() {
            if sp.swap_id == details.swap_id {
                return String::from("Swap already exists");
            }
        }

        self.available_swaps.push(details);
        return String::from("Swap pair added");
    }

    pub fn remove_swap_pair (&mut self, swap_id: SwapPair) -> String {
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

    pub fn get_single_swap_pair(&self, pair: SwapPair ) -> Option<SwapPairDetails> {
        for swp in self.available_swaps.clone() {
            if swp.swap_id == pair {
                return Some(swp);
            }
        }
        return None;
    }

    pub fn clear_all_swaps(&mut self){
        self.available_swaps.clear();
    }
    
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum SwapPair {
    #[default]
    ICP_ICP,
    ICP_XDR,
    ICP_USD,
    CKBTC_ICP,
    CHAT_ICP,
    OGY_ICP,
    SNEED_ICP,
    SNS1_ICP,
    SONIC_ICP,
    MOD_ICP,
    BOOM_ICP,
    GHOST_ICP,
    HOT_ICP,
    CAT_ICP,
    KINIC_ICP,
    NUA_ICP,
    ICX_ICP,
    TAL_ICP,
    CKETH_ICP,
    TRAX_ICP,
    GLDGOV_ICP,
    NTN_ICP,
    EXE_ICP,
    QUERIO_ICP,
    MOTOKO_ICP
}

impl SwapPair {
    pub fn to_string(&self) -> String {
        match self {
            SwapPair::ICP_ICP{..} => "ICP/ICP".to_string(),
            SwapPair::ICP_XDR{..} => "ICP/XDR".to_string(),
            SwapPair::ICP_USD{..} => "ICP/USD".to_string(),
            SwapPair::CKBTC_ICP{..} => "CKBTC/ICP".to_string(),
            SwapPair::CHAT_ICP{..} => "CHAT/ICP".to_string(),
            SwapPair::OGY_ICP{..} => "OGY/ICP".to_string(),
            SwapPair::SNEED_ICP{..} => "SNEED/ICP".to_string(),
            SwapPair::SNS1_ICP{..} => "SNS1/ICP".to_string(),
            SwapPair::SONIC_ICP{..} => "SONIC/ICP".to_string(),
            SwapPair::MOD_ICP{..} => "MOD/ICP".to_string(),
            SwapPair::BOOM_ICP{..} => "BOOM/ICP".to_string(),
            SwapPair::GHOST_ICP{..} => "GHOST/ICP".to_string(),
            SwapPair::HOT_ICP{..} => "HOT/ICP".to_string(),
            SwapPair::CAT_ICP{..} => "CAT/ICP".to_string(),
            SwapPair::KINIC_ICP{..} => "KINIC/ICP".to_string(),
            SwapPair::NUA_ICP{..} => "NUA/ICP".to_string(),
            SwapPair::ICX_ICP{..} => "ICX/ICP".to_string(),
            SwapPair::TAL_ICP{..} => "TAL/ICP".to_string(),
            SwapPair::CKETH_ICP{..} => "CKETH/ICP".to_string(),
            SwapPair::TRAX_ICP{..} => "TRAX/ICP".to_string(),
            SwapPair::GLDGOV_ICP{..} => "GLDGOV/ICP".to_string(),
            SwapPair::NTN_ICP{..} => "NTN/ICP".to_string(),
            SwapPair::EXE_ICP{..} => "EXE/ICP".to_string(),
            SwapPair::QUERIO_ICP{..} => "QUERIO/ICP".to_string(),
            SwapPair::MOTOKO_ICP{..} => "MOTOKO/ICP".to_string(),
        }
    }
}

impl fmt::Display for SwapPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            SwapPair::ICP_ICP{..} => "ICP/ICP".to_string(),
            SwapPair::ICP_XDR{..} => "ICP/XDR".to_string(),
            SwapPair::ICP_USD{..} => "ICP/USD".to_string(),
            SwapPair::CKBTC_ICP{..} => "CKBTC/ICP".to_string(),
            SwapPair::CHAT_ICP{..} => "CHAT/ICP".to_string(),
            SwapPair::OGY_ICP{..} => "OGY/ICP".to_string(),
            SwapPair::SNEED_ICP{..} => "SNEED/ICP".to_string(),
            SwapPair::SNS1_ICP{..} => "SNS1/ICP".to_string(),
            SwapPair::SONIC_ICP{..} => "SONIC/ICP".to_string(),
            SwapPair::MOD_ICP{..} => "MOD/ICP".to_string(),
            SwapPair::BOOM_ICP{..} => "BOOM/ICP".to_string(),
            SwapPair::GHOST_ICP{..} => "GHOST/ICP".to_string(),
            SwapPair::HOT_ICP{..} => "HOT/ICP".to_string(),
            SwapPair::CAT_ICP{..} => "CAT/ICP".to_string(),
            SwapPair::KINIC_ICP{..} => "KINIC/ICP".to_string(),
            SwapPair::NUA_ICP{..} => "NUA/ICP".to_string(),
            SwapPair::ICX_ICP{..} => "ICX/ICP".to_string(),
            SwapPair::TAL_ICP{..} => "TAL/ICP".to_string(),
            SwapPair::CKETH_ICP{..} => "CKETH/ICP".to_string(),
            SwapPair::TRAX_ICP{..} => "TRAX/ICP".to_string(),
            SwapPair::GLDGOV_ICP{..} => "GLDGOV/ICP".to_string(),
            SwapPair::NTN_ICP{..} => "NTN/ICP".to_string(),
            SwapPair::EXE_ICP{..} => "EXE/ICP".to_string(),
            SwapPair::QUERIO_ICP{..} => "QUERIO/ICP".to_string(),
            SwapPair::MOTOKO_ICP{..} => "MOTOKO/ICP".to_string(),
        };
        write!(f, "{}", name)
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SwapPairDetails{
    pub swap_id: SwapPair,
    pub token0: Token,
    pub token1: Token,
    pub marketplaces: Vec<MarketplaceDetails>,
    pub last_quote: u64,
    pub active: bool 
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub ticker: String, 
    pub ledger: String, 
    pub decimals: u32,
    pub standard: TokenStandard
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
    pub swap_pair: SwapPair,
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
    pub swap_pair: SwapPair, 
    pub quote: f64
}
// this struct should match enum swap pair! 
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct InternalRates {
    data: Vec<InternalRateEntry>,
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

    pub fn does_swap_exist(&self, swap_pair: SwapPair) -> bool {
        for qt in &self.data {
            if qt.swap_pair == swap_pair { return true; }
        }
        return false;
    }

    pub fn update_all_quotes(&mut self, rates_vec: Vec<InternalRateEntry>){
        // retain Stable quotes
        let mut retained: Vec<InternalRateEntry> = Vec::new();
        for qte in &self.data{
            if  qte.swap_pair == SwapPair::ICP_ICP || 
                qte.swap_pair == SwapPair::ICP_USD || 
                qte.swap_pair == SwapPair::ICP_XDR {
                    retained.push(qte.clone());
            }
        }
        // overwrite all other quotes
        self.data = rates_vec;
        // add stable quotes back in 
        for qte in retained {
            self.data.push(qte)
        }
    }

    pub fn update_single_quote(&mut self, swap_pair: &SwapPair, quote: f64){
        // check if already added
        for ire in self.data.iter_mut() {
            if swap_pair == &ire.swap_pair {
                ire.quote = quote;
                return
            }
        }
    }

    pub fn get_all_rates_vec(&self) -> Vec<InternalRateEntry> {
        return self.data.clone();
    }

    pub fn get_single_rate(&self, swap_pair: &SwapPair) -> Option<f64> {
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

}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct QuoteBucket{
    pub swap_pair: SwapPair,
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