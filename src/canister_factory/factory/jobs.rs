use candid::encode_args;

use crate::core::utils::{canister_call, get_self_cycle_balance, log};
use crate::core::runtime::RUNTIME_STATE;

use super::constants::{CREATE_CANISTER_CYCLES, CYCLES_REQUIRED_NEW_TOKEN_BACKEND, SAORSA_ADMIN, SUPER_INDEX_WASM_NAME, SUPER_STATS_WASM_NAME, TX_STORE_WASM_NAME};
use super::factory::{create_new_canister, install_wasm};
use super::factory_types::InstallMode;


// Adds Super Index, Super Stats and TX Store Canisters.
pub async fn create_new_token_backend_canisters() -> Result<Vec<(String, String)>, String> {

    // check self-cycles balance
    let cycles_required:u64 = CYCLES_REQUIRED_NEW_TOKEN_BACKEND;
    let self_id: String = RUNTIME_STATE.with(|s|{
        s.borrow().data.get_self_id()
    });
    let self_cycles: u64 = get_self_cycle_balance().await;

    // Create Canisters + Install WASM
    let mut created_canisters: Vec<(String, String)> = Vec::new();  // canister, name
    let ctrlrs: Vec<String> = Vec::from([
        SAORSA_ADMIN.to_string(),
        self_id.clone()
    ]);
    
    if self_cycles > cycles_required {
        // install arg
        let mut arg: Vec<u8> = Vec::new();
        let impt_arg = encode_args((self_id.clone(),));
        match impt_arg {
            Ok(v) => {arg = v},
            Err(e) => { 
                let error = format!("ERROR - Could not ecode args (create_new_token_backend_canisters) - {}", e);
                log(error.clone());
                return Err(error);
            }
        }

        let total_cycles = CREATE_CANISTER_CYCLES + 3_000_000_000_000; // 3T extra
        // SUPER INDEX 
        let si_res = create_new_canister(ctrlrs.clone(), total_cycles.clone()).await;
        match si_res {
            Ok(v) => {
                // Super Index Wasm
                let si_wasm: Option<Vec<u8>> = RUNTIME_STATE.with(|state|{
                    state.borrow().data.available_wasms.get_wasm(SUPER_INDEX_WASM_NAME.to_string())
                });

                // install wasm
                if let Some(wsm) = si_wasm {
                    let callres = install_wasm(
                        wsm, 
                        InstallMode::Install, 
                        v.clone(), 
                        Some(arg.clone())
                    )
                    .await;
                    match callres {
                        Err(e) => {
                            let error = format!("ERROR - Could not install WASM (SuperIndex) - {}", e);
                            log(error.clone());
                            return Err(error);
                            }
                        _ => {} 
                    }

                    // Add canister to response vec
                    created_canisters.push((v.clone(), SUPER_INDEX_WASM_NAME.to_string()));
                    
                    // Add SAORSA ADMIN
                    let _add_admin: Result<(String,), (ic_cdk::api::call::RejectionCode, String)> = canister_call(
                        v.as_str(),
                        "add_admin",
                        SAORSA_ADMIN.to_string(),
                        None
                    ).await;
                }
            },
            Err(e) => {
                let error = format!("ERROR - Could not install WASM (SuperIndex) - {}", e);
                log(error.clone());
                return Err(error);
            }
        } // end add Super Index. 

        // SUPER STATS 
        let ss_res = create_new_canister(ctrlrs.clone(), total_cycles.clone()).await;
        match ss_res {
            Ok(v) => {
                // Super Stats Wasm
                let si_wasm: Option<Vec<u8>> = RUNTIME_STATE.with(|state|{
                    state.borrow().data.available_wasms.get_wasm(SUPER_STATS_WASM_NAME.to_string())
                });

                // install wasm
                if let Some(wsm) = si_wasm {
                    let callres = install_wasm(
                        wsm, 
                        InstallMode::Install, 
                        v.clone(), 
                        Some(arg.clone())
                    )
                    .await;
                    match callres {
                        Err(e) => {
                            let error = format!("ERROR - Could not install WASM (SuperStats) - {}", e);
                            log(error.clone());
                            return Err(error);
                            }
                        _ => {} 
                    }

                    // Add canister to response vec
                    created_canisters.push((v.clone(), SUPER_STATS_WASM_NAME.to_string()));
                    
                    // Add SAORSA ADMIN
                    let _add_admin: Result<(String,), (ic_cdk::api::call::RejectionCode, String)> = canister_call(
                        v.as_str(),
                        "add_admin",
                        SAORSA_ADMIN.to_string(),
                        None
                    ).await;
                }
            },
            Err(e) => {
                let error = format!("ERROR - Could not install WASM (SuperStats) - {}", e);
                log(error.clone());
                return Err(error);
            }
        } // end add Super Stats.

        // TX STORE 
        let tx_res = create_new_canister(ctrlrs, total_cycles.clone()).await;
        match tx_res {
            Ok(v) => {
                // TX Store Wasm
                let si_wasm: Option<Vec<u8>> = RUNTIME_STATE.with(|state|{
                    state.borrow().data.available_wasms.get_wasm(TX_STORE_WASM_NAME.to_string())
                });

                // install wasm
                if let Some(wsm) = si_wasm {
                    let callres = install_wasm(
                        wsm, 
                        InstallMode::Install, 
                        v.clone(), 
                        Some(arg.clone())
                    )
                    .await;
                    match callres {
                        Err(e) => {
                            let error = format!("ERROR - Could not install WASM (Tx Store) - {}", e);
                            log(error.clone());
                            return Err(error);
                            }
                        _ => {} 
                    }

                    // Add canister to response vec
                    created_canisters.push((v.clone(), TX_STORE_WASM_NAME.to_string()));
                    
                    // Add SAORSA ADMIN
                    let _add_admin: Result<(String,), (ic_cdk::api::call::RejectionCode, String)> = canister_call(
                        v.as_str(),
                        "add_admin",
                        SAORSA_ADMIN.to_string(),
                        None
                    ).await;
                }
            },
            Err(e) => {
                let error = format!("ERROR - Could not install WASM (tx store) - {}", e);
                log(error.clone());
                return Err(error);
            }
        } // end add Super Stats.        

        return Ok(created_canisters);
    } else {
        log("ERROR - Not enough cycles to run (create_new_token_backend_canisters)");
        return Err(String::from("ERROR - Not enough cycles to run (create_new_token_backend_canisters)"));
    }
}