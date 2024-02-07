// get supply every hour 
// get prices 5mins 

use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::core::{runtime::RUNTIME_STATE, utils::{canister_call, log, nat_to_f64}, constants::OHLC_CANISTER_ID};

#[derive(CandidType, Deserialize, Serialize, Default, Clone)]
pub struct TokenData {
    pub cross: String, 
    pub ledger: String,
    pub decimals: u8,
    pub change24: f64,
    pub change7d: f64,
    pub price: f64,
    pub supply: f64,
    pub mcap: f64,
    pub sparkline: Vec<f64>
}

pub async fn update_icrc1_total_supply() -> Result<u8, u8> {
    let mut any_errors = false;
    let mut tt_vec = RUNTIME_STATE.with(|s|s.borrow().data.top_tokens.clone());
    for tkn in &mut tt_vec {
        let tkn_supply: Result<(Nat,), (ic_cdk::api::call::RejectionCode, String)> =
        canister_call(tkn.ledger.as_str(), "icrc1_total_supply", (), None).await;
        match tkn_supply {
            Ok(v) => {
                let divisor = 10f64.powi(tkn.decimals as i32);
                let t_supply = nat_to_f64(v.0);
                match t_supply{
                    Ok(v_f64) => {
                        tkn.supply = v_f64/divisor;
                    }
                    Err(e) => {
                        log(format!("Error (update_icrc1_total_supply :: nat_to_f64) - {}", e)); 
                        any_errors = true;
                    },
                }
            },
            Err(e) => {
                log(format!("Error (update_icrc1_total_supply) - {:?}, {}", e.0, e.1));
                any_errors = true;
            }
        }
    }
    // update
    RUNTIME_STATE.with(|s|s.borrow_mut().data.top_tokens = tt_vec);

    if any_errors {
        return Err(0)
    } else {
        return Ok(0)
    };

}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct TokenPriceChange{
    pub cross: String, 
    pub latest_price: f64,
    pub change_24: f64,
    pub change_7d: f64,
    pub sparkline: Vec<f64>
}

pub async fn update_price_data() -> Result<u8, u8> {
    let change_data: Result<(Vec<TokenPriceChange>,), (ic_cdk::api::call::RejectionCode, String)> =
    canister_call(OHLC_CANISTER_ID, "get_all_change_data", (), None).await;
    
    match change_data {
        Ok(cng) => {
            let mut tt_vec = RUNTIME_STATE.with(|s|s.borrow().data.top_tokens.clone());
            for tpc in cng.0 {
                for td in &mut tt_vec {
                    if td.cross == tpc.cross {
                        td.change24 = tpc.change_24;
                        td.change7d = tpc.change_7d;
                        td.price = tpc.latest_price;
                        td.sparkline = tpc.sparkline.clone();
                        td.mcap = td.supply*tpc.latest_price;
                    }
                }
            }
            // update 
            RUNTIME_STATE.with(|s|{s.borrow_mut().data.top_tokens = tt_vec});
            return Ok(0);
        },
        Err(e) => {
            log(format!("Error (update_price_data) - {:?}, {}", e.0, e.1));
            return Err(0);
        },
    }
}

#[derive(CandidType, Deserialize, Serialize, Default, Clone)]
pub struct TopHolderData {
    pub cross: String, 
    pub stats221: String,
    pub accounts: Vec<HolderBalanceResponse>,
    pub principals: Vec<HolderBalanceResponse>
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct HolderBalanceResponse {
    pub holder: String,
    pub data: Overview
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct Overview {
   pub first_active: u64,
   pub last_active: u64,
   pub sent: (u32, u128), // count, value
   pub received: (u32, u128), // count, value
   pub balance: u128,
}

pub async fn update_top_holders() -> Result<u8, u8> {
    let mut error = false;
    let mut token_vec = RUNTIME_STATE.with(|s|{s.borrow().data.top_holders.clone()});
    for top in &mut token_vec{
        let top_accounts: Result<(Vec<HolderBalanceResponse>,), (ic_cdk::api::call::RejectionCode, String)> =
        canister_call(top.stats221.as_str(), "get_top_account_holders", 100_u64, None).await;
        match top_accounts {
            Ok(v) => {
                top.accounts = v.0;
            },
            Err(e) => {
                error = true;
                log(format!("Error (update_top_holders 1 ) - {:?}, {}", e.0, e.1));
            }
        }

        let top_principals: Result<(Vec<HolderBalanceResponse>,), (ic_cdk::api::call::RejectionCode, String)> =
        canister_call(top.stats221.as_str(), "get_top_principal_holders", 100_u64, None).await;
        match top_principals {
            Ok(v) => {
                top.principals = v.0;
            },
            Err(e) => {
                error = true;
                log(format!("Error (update_top_holders 2) - {:?}, {}", e.0, e.1));
            }
        }
    }
    RUNTIME_STATE.with(|s|{s.borrow_mut().data.top_holders = token_vec});
    if error == false { return Ok(0) } else { return Err(0) }
}