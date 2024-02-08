extern crate defi_oracle_shared;
use candid::Principal;
use defi_oracle_shared::shared_types::*;
use crate::core::runtime::RUNTIME_STATE;

pub async fn set_mib_marketplace_impl(mib_canister: String, marketplace: Marketplace) -> String {
    let pr = Principal::from_text(mib_canister);
    match pr {
        Ok(pr_res) => {
            let res: Result<(String,), (ic_cdk::api::call::RejectionCode, String)> 
            = ic_cdk::call(pr_res, "set_assigned_marketplace", (marketplace, )).await;
            match res {
                Ok(v) => {
                    return v.0;
                },
                Err(e) => {
                    let ret = format!("Error - could not set_assigned_marketplace. Call failed. {:?} - {}", e.0, e.1);
                    return ret;
                }
            }
        },  
        Err(error) => {
            return "Error - could not parse input to principal".to_string();
        },
    }
}

pub async fn add_swap_pair_to_mib_canister_impl(mib_canister: String, swap_pair: SwapPair) -> String {
    let pr = Principal::from_text(mib_canister);
    let spd = RUNTIME_STATE.with(|s|
        {s.borrow().data.all_swaps.get_single_swap_pair(swap_pair)});
    match pr {
        Ok(pr_res) => {
            match spd {
                Some(swap_details) => {
                    let res: Result<(String,), (ic_cdk::api::call::RejectionCode, String)> 
                    = ic_cdk::call(pr_res, "add_token_cross", (swap_details, )).await;
                    match res {
                        Ok(v) => {
                            return v.0;
                        },
                        Err(e) => {
                            let ret = format!("Error - could not add_token_cross. Call failed. {:?} - {}", e.0, e.1);
                            return ret;
                        }
                    }
                },
                None => {
                    return "Could not find the swap pair to send".to_string();
                }, 
            }
        },  
        Err(error) => {
            return "Error - could not parse input to principal".to_string();
        },
    }
}

pub async fn remove_swap_pair_from_mib_canister_impl(mib_canister: String, swap_pair: SwapPair) -> String {
    let pr = Principal::from_text(mib_canister);
    match pr {
        Ok(pr_res) => {      
            let res: Result<(String,), (ic_cdk::api::call::RejectionCode, String)> 
            = ic_cdk::call(pr_res, "remove_token_cross", (swap_pair, )).await;
            match res {
                Ok(v) => {
                    return v.0;
                },
                Err(e) => {
                    let ret = format!("Error - could not add_token_cross. Call failed. {:?} - {}", e.0, e.1);
                    return ret;
                }
            }
        },  
        Err(error) => {
            return "Error - could not parse input to principal".to_string();
        },
    }
}

pub async fn request_authorisation_on_mib(mib_canister: String) -> String {
    let pr = Principal::from_text(mib_canister);
    match pr {
        Ok(pr_res) => {
            
            let res: Result<(bool,), (ic_cdk::api::call::RejectionCode, String)> 
            = ic_cdk::call(pr_res, "authorise_cpc", ((), )).await;
            match res {
                Ok(v) => {
                    return "MIB Canister added and CPC authorised".to_string();
                },
                Err(e) => {
                    let ret = format!("Error - could not authorise_cmc_canister. Call failed. {:?} - {}", e.0, e.1);
                    return ret;
                }
            }
        },  
        Err(error) => {
            return "Error - could not parse input to principal".to_string();
        },
    }
}