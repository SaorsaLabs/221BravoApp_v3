use candid::{encode_args, Principal};
use ic_cdk::api::management_canister::main::{
    canister_status, create_canister, delete_canister,
    deposit_cycles, install_code, stop_canister,
    update_settings, CanisterId, CanisterIdRecord, CanisterInstallMode, CanisterSettings,
    CreateCanisterArgument, InstallCodeArgument, UpdateSettingsArgument,
};

use crate::core::{runtime::RUNTIME_STATE, utils::{canister_call, get_self_cycle_balance, log}};

use super::{constants::{CREATE_CANISTER_CYCLES, CYCLES_MANAGER_CANISTER, SAORSA_ADMIN}, factory_types::InstallMode};

pub async fn create_new_canister(controllers: Vec<String>, cycles: u128) -> Result<String, String> {

    let mut ctls: Vec<Principal> = Vec::new(); 
    for ctl in controllers.iter() {
        let pr = Principal::from_text(ctl);
        match pr {
            Ok(v) => {
                ctls.push(v.clone());
            },
            Err(e) => {
                let error = format!("ERROR - Could not convert text to Principal - {}", e);
                log(error.clone());
                return Err(error);
            }
        }
    }

    let create_res = create_canister(
        CreateCanisterArgument {
            settings: Some(CanisterSettings {
                controllers: Some(ctls),
                compute_allocation: None,
                memory_allocation: None,
                freezing_threshold: None,
                reserved_cycles_limit: None,
            }),
        },
        CREATE_CANISTER_CYCLES + cycles,
    )
    .await;

    match create_res {
        Ok(v) => {
            let cnstr = v.0.canister_id.to_text();
            return Ok(cnstr);
        },
        Err(e) => {
            let error = format!("ERROR - {:?}, {}", e.0, e.1);
            log(error.clone());
            return Err(error);
        }
    }

}

pub async fn install_wasm(wasm_vec: Vec<u8>, mode: InstallMode, canister: String, candid_args: Option<Vec<u8>>) -> Result<String, String> {
    
    let mde: CanisterInstallMode;
    match mode {
        InstallMode::Install => { mde = CanisterInstallMode::Install },
        InstallMode::Reinstall => { mde = CanisterInstallMode::Reinstall },
        InstallMode::Upgrade => { mde = CanisterInstallMode::Upgrade }
    } 
    let cnstr: Principal;
    match Principal::from_text(canister){
        Ok(v) => { cnstr = v },
        Err(e) => {
            let error = format!("ERROR - Could not convert text to Principal (install_wasm) - {}", e);
            log(error.clone());
            return Err(error);
        }
    }
    let mut argument: Vec<u8> = Vec::new();
    if let Some(inpt_arg) = candid_args {
        argument = inpt_arg;
    }

    let arg = InstallCodeArgument {
        mode:mde,
        canister_id: cnstr,
        wasm_module: wasm_vec.clone(),
        arg: argument,
    };

    let res = install_code(arg).await;
    match res {
        Ok(_v) => { return Ok(String::from("Wasm Installed"))},
        Err(e) => {
            let error = format!("ERROR (install_wasm) - {:?}, {}", e.0, e.1);
            log(error.clone());
            return Err(error);
        }
    }
}

// deploys any wasm which includes the admin principal deploy argument :  --argument("XXX-XXX-XX")
pub async fn create_new_cansister_with_wasm_impl(wasm_name: String) -> Result<(String, String), String> {

    let cycles_required:u128 = CREATE_CANISTER_CYCLES;
    let self_id: String = RUNTIME_STATE.with(|s|{
        s.borrow().data.get_self_id()
    });
    let self_cycles: u64 = get_self_cycle_balance().await;

    // Create Canisters + Install WASM
    let mut created_canister: (String, String) = ("".to_string(), "".to_string());
    let ctrlrs: Vec<String> = Vec::from([
        SAORSA_ADMIN.to_string(),
        self_id.clone()
    ]);
    
    if self_cycles > cycles_required as u64 {
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
        let create_res = create_new_canister(ctrlrs.clone(), cycles_required.clone()).await;
        match create_res {
            Ok(v) => {
                // Selected Wasm
                let si_wasm: Option<Vec<u8>> = RUNTIME_STATE.with(|state|{
                    state.borrow().data.available_wasms.get_wasm(wasm_name.clone())
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
                    created_canister = (v.clone(), wasm_name);
                    
                    // Add SAORSA ADMIN
                    let _add_admin: Result<(String,), (ic_cdk::api::call::RejectionCode, String)> = canister_call(
                        v.as_str(),
                        "add_admin",
                        SAORSA_ADMIN.to_string(),
                        None
                    ).await;

                    // Add to cycles manager
                    let _add_cycle_manager: Result<(String,), (ic_cdk::api::call::RejectionCode, String)> = canister_call(
                        CYCLES_MANAGER_CANISTER,
                        "add_canister",
                        v.as_str(),
                        None
                    ).await;
                }
            },
            Err(e) => {
                let error = format!("ERROR - Could not install WASM (New Canister IMPL) - {}", e);
                log(error.clone());
                return Err(error);
            }
        } 
        return Ok(created_canister);
    } else {
        log("ERROR - Not enough cycles to run (create canister_with_wasm_impl)");
        return Err(String::from("ERROR - Not enough cycles to run (create canister_with_wasm_impl)"));
    }
}