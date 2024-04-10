use std::default;

use candid::CandidType;
use oracle_shared_mk2::shared_types::{Marketplace, Token};
use serde::{Serialize, Deserialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct XdrRates {
    pub icp_xdr: Option<u64>,
}

// Types for exchange rate canister (ICP/USD)
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GetExchangeRateRequest {
    pub timestamp: Option<u64>,
    pub quote_asset: Asset,
    pub base_asset: Asset,
  }

  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub struct Asset { pub class: AssetClass, pub symbol: String }

  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub enum AssetClass { Cryptocurrency, FiatCurrency }

  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub enum GetExchangeRateResult { Ok(ExchangeRate), Err(ExchangeRateError) }
  
  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub struct ExchangeRateMetadata {
    pub decimals: u32,
    pub forex_timestamp: Option<u64>,
    pub quote_asset_num_received_rates: u64,
    pub base_asset_num_received_rates: u64,
    pub base_asset_num_queried_sources: u64,
    pub standard_deviation: u64,
    pub quote_asset_num_queried_sources: u64,
  }
  
  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub struct ExchangeRate {
    pub metadata: ExchangeRateMetadata,
    pub rate: u64,
    pub timestamp: u64,
    pub quote_asset: Asset,
    pub base_asset: Asset,
  }
  
  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub enum ExchangeRateError {
    AnonymousPrincipalNotAllowed,
    CryptoQuoteAssetNotFound,
    FailedToAcceptCycles,
    ForexBaseAssetNotFound,
    CryptoBaseAssetNotFound,
    StablecoinRateTooFewRates,
    ForexAssetsNotFound,
    InconsistentRatesReceived,
    RateLimited,
    StablecoinRateZeroRate,
    Other{ code: u32, description: String },
    ForexInvalidTimestamp,
    NotEnoughCycles,
    ForexQuoteAssetNotFound,
    StablecoinRateNotFound,
    Pending,
  }

  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct AddSwapInput{
    pub swap_id: String,
    pub token0: Token,
    pub token1: Token,
    pub swap_type: u8,
    pub init_quote: f64
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct MIBManager {
  mibs: Vec<MIBVersion>,
}

impl MIBManager {
  pub fn add_mib(&mut self, mib_data: MIBVersion) -> String {
    // check if exists
    for mib in self.mibs.iter() {
      match (&mib, &mib_data) {
        // V1
        (MIBVersion::V1(v), MIBVersion::V1(v2)) => {
            if v.canister == v2.canister {
              return String::from("MIB Canister already exists");
            }
        },
        _ => {}
      }
    }
    self.mibs.push(mib_data);
    return String::from("MIB Canister added");
  }

  pub fn remove_mib(&mut self, mib_canister: String) -> String {
    let mut position: usize = 0;
    let mut found: bool = false;
    let mut count = 0;
    for mib in self.mibs.iter() {
      match &mib {
        // V1
        MIBVersion::V1(v) => {
            if v.canister == mib_canister {
              position = count as usize;
              found = true;
            }
        },
        _ => {}
      }
      count += 1; 
    }
    if found == true {
      self.mibs.remove(position);
      return String::from("MIB Canister removed");
    } else {
      return String::from("MIB Canister could not be found");
    }
  }

  pub fn set_mib_marketplace (&mut self, mib_canister: String, marketplace: Marketplace) -> String {
    for mib in self.mibs.iter_mut() {
      match mib {
        // V1
        MIBVersion::V1(v) => {
            if v.canister == mib_canister {
              v.assigned_marketplace = marketplace;
              return String::from("Marketplace has been updated");
            }
        },
        _ => {}
      }
    }
    return String::from("Could not find marketplace");
  }

  pub fn get_mibs_by_cross(&self, cross:String) -> Option<Vec<MIBVersion>> {
    let mut ret_data: Vec<MIBVersion> = Vec::new();
    let mut hit: bool = false;
    for mib in self.mibs.iter() {
      match mib {
        // V1
        MIBVersion::V1(v) => {
            let mut found = false; 
            for mib_cross in v.crosses.iter() {
              if mib_cross.0 == cross {
                found = true;
                hit = true;
              }
            }
            if found == true {
              ret_data.push(mib.clone());
            }
        },
        _ => {}
      }
    }
    if hit == true {
      return Some(ret_data);
    } else {
      return None;
    }
  }

  pub fn get_mibs_by_marketplace(&self, marketplace: Marketplace) -> Option<Vec<MIBVersion>> {
    let mut ret_data: Vec<MIBVersion> = Vec::new();
    let mut hit: bool = false;
    for mib in self.mibs.iter() {
      match mib {
        // V1
        MIBVersion::V1(v) => {
            if v.assigned_marketplace == marketplace {
              ret_data.push(mib.clone());
              hit = true;
            }
        },
        _ => {}
      }
    }
    if hit == true {
      return Some(ret_data);
    } else {
      return None;
    }
  }

  pub fn get_mibs_by_cross_and_marketplace (&self, cross:String, marketplace: Marketplace) -> Option<Vec<MIBVersion>> {
    let mut ret_data: Vec<MIBVersion> = Vec::new();
    let mut hit: bool = false;
    for mib in self.mibs.iter() {
      match mib {
        // V1
        MIBVersion::V1(v) => {
            let mut found = false; 
            if v.assigned_marketplace == marketplace {
              for mib_cross in v.crosses.iter() {
                if mib_cross.0 == cross {
                  found = true;
                  hit = true;
                }
              }
            }
            if found == true {
              ret_data.push(mib.clone());
            }
        },
        _ => {}
      }
    }
    if hit == true {
      return Some(ret_data);
    } else {
      return None;
    }
  }

  pub fn get_all_mib_canisters(&self) -> Vec<(String, String)> {
    let mut ret: Vec<(String, String)> = Vec::new();
    for mib in self.mibs.iter() {
      match mib {
        // V1
        MIBVersion::V1(v) => {
            ret.push((v.canister.clone(), v.name.clone()));
        },
        _ => {}
      }
    }
    ret
  }

  pub fn get_all_v1_mib_canisters_raw(&self) -> Vec<MIBV1> {
    let mut ret: Vec<MIBV1> = Vec::new();
    for mib in self.mibs.iter() {
      match mib {
        // V1
        MIBVersion::V1(v) => {
            ret.push(v.clone());
        },
        _ => {}
      }
    }
    ret
  }

  pub fn add_cross_to_mib(&mut self, mib_canister: String, cross: String, active: bool) -> String {
    for mib in self.mibs.iter_mut() {
      match mib {
        // V1
        MIBVersion::V1(v) => {
            if v.canister == mib_canister {
              v.crosses.push((cross, active));
              return String::from("Cross has been added");
            }
        },
        _ => {}
      }
    }
    return String::from("Could not find MIB to add cross");
  } 

  pub fn remove_cross_from_mib(&mut self, mib_canister: String, cross: String) -> String {
    for mib in self.mibs.iter_mut() {
      match mib {
        // V1
        MIBVersion::V1(v) => {
            if v.canister == mib_canister {
              v.crosses.retain(|x: &(String, bool)| x.0 != cross);
              return String::from("Cross has been removed");
            }
        },
        _ => {}
      }
    }
    return String::from("Could not find MIB to remove cross");

    // if auth_vec.contains(&principal_id){
    //         auth_vec.retain(|x: &String| x != &principal_id);
    //     } else { 
  }
  }


#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum MIBVersion {
  V1(MIBV1), 
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct MIBV1 {
  pub name: String, 
  pub canister: String, 
  pub crosses: Vec<(String, bool)>, // cross + active
  pub assigned_marketplace: Marketplace
}


// turn off exchange = lookup all mibs, find mibs for X exchange.. mark all pairs not-active
// turn off pair = lookup all mibs which have the pair and turn the pair not-active
// turn off pair on ONLY X exchange - lookup all mibs for X exchange.. loop to set pair not active. 