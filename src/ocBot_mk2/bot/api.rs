use bot_api::handle_direct_message::Args;
use ic_cdk::{query, update};

use crate::core::runtime::RUNTIME_STATE;
use super::{constants::BOT_NAME, send_message::send__text_message, setup::register_bot_with_oc};

#[update]
async fn register() -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text())
    });
    register_bot_with_oc().await
}

#[update]
async fn send_message(user_caniser: String, message: String) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text())
    });
    let outcome = send__text_message(user_caniser, message).await;
    match outcome {
        Ok(v) => { v },
        Err(e) => { ic_cdk::trap(e.as_str()) }
    }
}

#[query]
fn get_bot_name() -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text())
    });
    return BOT_NAME.to_string();
}

// #[update]
// fn handle_direct_message(args: Args){

// }