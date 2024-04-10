use candid::{Principal, CandidType};
use serde::{Serialize, Deserialize};
use crate::core::utils::{canister_call, log};
use super::constants::{BOT_HANDLE, BOT_NAME, BOT_REGISTRATION_FEE, OC_USER_INDEX_CANISTER};

// Types for bot registration
pub type Cycles = u128;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct RegArgs {
    pub username: String,
    pub display_name: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum RegResponse {
    Success,
    AlreadyRegistered,
    UserLimitReached,
    UsernameTaken,
    UsernameInvalid,
    UsernameTooShort(u16),
    UsernameTooLong(u16),
    InsufficientCyclesProvided(Cycles),
    InternalError(String),
}

pub async fn register_bot_with_oc() -> String {
    let args = RegArgs{
        username: String::from(BOT_HANDLE),
        display_name: Some(String::from(BOT_NAME)),
    };

    let call: Result<(RegResponse,), (ic_cdk::api::call::RejectionCode, String)> =
    canister_call(OC_USER_INDEX_CANISTER, "c2c_register_bot", args, Some(BOT_REGISTRATION_FEE)).await;

    match call {
        Ok(v) => {
            format!("Call Success :: {:?}", v.0)
        },
        Err(e) => {format!("Call Error :: {:?}, {}", e.0, e.1)}
    }
}

pub async fn change_display_name(name: String){
    
}