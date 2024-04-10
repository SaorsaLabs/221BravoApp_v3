extern crate oracle_shared_mk2;
use candid::Principal;
use oracle_shared_mk2::shared_types::*;
use crate::core::{runtime::RUNTIME_STATE, utils::canister_call};

use super::{misc_types::MIBVersion, utils::log};

pub async fn set_mib_marketplace_impl(mib_canister: String, marketplace: Marketplace) -> String {
    let pr = Principal::from_text(mib_canister.clone());
    match pr {
        Ok(pr_res) => {
            let res: Result<(String,), (ic_cdk::api::call::RejectionCode, String)> 
            = ic_cdk::call(pr_res, "set_assigned_marketplace", (marketplace.clone(), )).await;
            match res {
                Ok(v) => {
                    // set marketplace
                    RUNTIME_STATE.with(|s|{
                        s.borrow_mut().data.mib_manager.set_mib_marketplace(mib_canister.clone(), marketplace.clone());
                    });
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

pub async fn add_swap_pair_to_mib_canister_impl(mib_canister: String, swap_pair: String) -> String {
    let pr = Principal::from_text(mib_canister.clone());
    let spd = RUNTIME_STATE.with(|s|
        {s.borrow().data.all_swaps.get_single_swap_pair(swap_pair.clone())});
    match pr {
        Ok(pr_res) => {
            match spd {
                Some(swap_details) => {
                    let mut sdt:SwapPairDetails = swap_details;
                    sdt.active = false; // new crosses are not-active on init
                    let res: Result<(String,), (ic_cdk::api::call::RejectionCode, String)> 
                    = ic_cdk::call(pr_res, "add_token_cross", (sdt, )).await;
                    match res {
                        Ok(v) => {
                            // add to MIB Manager
                            RUNTIME_STATE.with(|s|{
                                s.borrow_mut().data.mib_manager.add_cross_to_mib(mib_canister.clone(), swap_pair.clone(), false);
                            });
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

pub async fn remove_swap_pair_from_mib_canister_impl(mib_canister: String, swap_pair: String) -> String {
    let pr = Principal::from_text(mib_canister.clone());
    match pr {
        Ok(pr_res) => {      
            let res: Result<(String,), (ic_cdk::api::call::RejectionCode, String)> 
            = ic_cdk::call(pr_res, "remove_token_cross", (swap_pair.clone(), )).await;
            match res {
                Ok(v) => {
                    // add to MIB Manager
                    RUNTIME_STATE.with(|s|{
                        s.borrow_mut().data.mib_manager.remove_cross_from_mib(mib_canister.clone(), swap_pair.clone());
                    });
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

pub async fn update_pair_status_impl(swap_pair: String, marketplace: Option<Marketplace>, status: bool) -> String {
    let mibs: Option<Vec<MIBVersion>>;
    if let Some(mkt) = marketplace {
        mibs = RUNTIME_STATE.with(|s|{
            s.borrow().data.mib_manager.get_mibs_by_cross_and_marketplace(swap_pair.clone(), mkt)
        });
    } else {
        mibs = RUNTIME_STATE.with(|s|{
            s.borrow().data.mib_manager.get_mibs_by_cross(swap_pair.clone())
        });
        // set status on CPC all swaps
        RUNTIME_STATE.with(|s|{
            s.borrow_mut().data.all_swaps.set_swap_status(swap_pair.clone(), status.clone());
        });
    }

    if let Some (mib_result) = mibs {
        let mut targ_canisters: Vec<String> = Vec::new();
        for mibv in mib_result.iter() {
            match mibv {
                MIBVersion::V1(v) => {
                    targ_canisters.push(v.canister.clone())
                },
                _ => {}
            }
        }

        let mut err: bool = false;
        // loop target canisters - set status on mibs.
        for cnstr in targ_canisters.iter() {
            let args: ChangeStatusArgs = ChangeStatusArgs{
                token: swap_pair.clone(),
                status: status.clone(),
            };
            let res: Result<(String,), (ic_cdk::api::call::RejectionCode, String)> = canister_call(
                cnstr.as_str(), "set_token_cross_status", args, None
            ).await;
            match res {
                Err(e) => {
                    log(format!("Error updating MIB cross status: {:?} - {}", e.0, e.1));
                    err = true;
                }
                _ => {}
            }
        }
        if err == true { 
            return String::from("Error updating token status. See logs for more details");
        } else {
            // update CPC status
            RUNTIME_STATE.with(|state| {
                state.borrow_mut().data.all_swaps.set_swap_status(swap_pair, status)
            });
            return String::from("Token status has been updated on all mibs");
        }
    } else {
        return String::from("Could not find a matching swap/cross")
    }
}