use candid::Principal;

use super::{runtime::RUNTIME_STATE, constants::{OC_DEV_BOT, OC_DEV_PRINCIPAL}};

pub fn log(text: impl AsRef<str>){
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.add_log(text.as_ref().to_string())
    });
}

pub async fn oc_dev_message(message: String){
    // OC-Bot Canister
    let can = Principal::from_text(OC_DEV_BOT);
    match can {
        Ok(cnster) => {
            // Dev OC A/C
            let user = String::from(OC_DEV_PRINCIPAL);
            let _res: Result<((),), (ic_cdk::api::call::RejectionCode, String)> 
            = ic_cdk::call(cnster, "send", (user, message, )).await;
        },
        Err(_error) => {}
    }
}

pub async fn critical_err(message: String){
    let canister = RUNTIME_STATE.with(|s|{
        s.borrow().data.get_self_id()
    });
    let error = format!("{} - Canister: {}",message, canister);
    log(error.clone());
    oc_dev_message(error).await;
}