use candid::encode_args;
use ic_cdk::{query, update};
use crate::core::runtime::RUNTIME_STATE;

use super::{factory::{create_new_canister, create_new_cansister_with_wasm_impl, install_wasm}, factory_types::InstallMode, jobs::create_new_token_backend_canisters};

// [][] -- UPLOAD/ MANAGE WASM METHODS -- [][]
#[update]
fn add_wasm(wasm_vec: Vec<u8>, name: String, version: Option<String>) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    // add wasm
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.available_wasms.add_wasm(wasm_vec, name, version)
    })
}

#[update]
fn add_wasm_chunk(chunk: Vec<u8>, wasm_name: String) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    // add wasm
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.available_wasms.add_wasm_chunk(chunk, wasm_name)
    })
}

#[update]
fn remove_wasm(wasm_name: String) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    // add wasm
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.available_wasms.remove_wasm(wasm_name)
    })
}

#[update]
fn clear_wasm_vec(wasm_name: String) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    // add wasm
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.available_wasms.clear_wasm_vec(wasm_name)
    })
}

#[query]
fn get_wasm_length(wasm_name: String) -> u64 {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    // add wasm
    let size = RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.available_wasms.get_wasm_vec_length(wasm_name)
    });
    return size as u64;
}

#[query]
fn get_all_wasms() -> Vec<String> {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    // all wasms
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.available_wasms.get_all_available_wasms()
    })
}

// [][] -- CANISTER MANAGEMENT METHODS -- [][]
#[update]
async fn create_new_cansister_with_wasm(wasm_name: String) -> (String, String) {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    match create_new_cansister_with_wasm_impl(wasm_name).await {
        Ok(v) => { return v }
        Err(e) => { ic_cdk::trap(e.as_str()) }
    }
}


#[update]
async fn create_new_token_indexers() -> Vec<(String, String)> {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    match create_new_token_backend_canisters().await {
        Ok(v) => { return v },
        Err(e) => {
            ic_cdk::trap(e.as_str())
        }
    }
}


