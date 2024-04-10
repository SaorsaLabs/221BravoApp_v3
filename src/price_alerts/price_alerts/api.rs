use ic_cdk::{query, update};
use crate::core::{runtime::RUNTIME_STATE, stable_memory::{PriceTuple, STABLE_STATE}, types::IDKey};
use super::btree::{InputAlert};

#[update]
fn add_price_alert(data: InputAlert) -> i64 {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())}); 
    STABLE_STATE.with(|s|{
        let mut ptr = s.borrow_mut();
        let state = ptr.as_mut().unwrap();
        match state.alert_data.add_alert(data) {
            Ok(v) => v as i64,
            Err(_e) => -1_i64
        }
    })
}

#[update]
fn remove_price_alert(data: InputAlert) -> String {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())}); 
    STABLE_STATE.with(|s|{
        let mut ptr = s.borrow_mut();
        let state = ptr.as_mut().unwrap();
        state.alert_data.remove_alert(data)
    })
}

#[query] 
fn get_all_user_alerts(user: String) -> Option<Vec<InputAlert>> {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())}); 
    match IDKey::from_string(user) {
        Some(v) => {
            STABLE_STATE.with(|s|{
                let ptr = s.borrow();
                let state = ptr.as_ref().unwrap();
                state.alert_data.get_all_alerts_by_user(v)
            })
        },
        None => { None }
    }
}

#[query]
fn get_all_cross_alerts(cross: String) -> Option<Vec<InputAlert>> {
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text())}); 
    match IDKey::from_string(cross) {
        Some(v) => {
            STABLE_STATE.with(|s|{
                let ptr = s.borrow();
                let state = ptr.as_ref().unwrap();
                state.alert_data.get_all_alerts_by_cross(v)
            })
        },
        None => { None }
    }
}

#[update]
fn update_oracle_id(canister: String) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});

    RUNTIME_STATE.with(|s|{
        s.borrow_mut().data.set_oracle_id(String::from(canister));
    });
    String::from("Oracle ID Updated")
}

#[query]
fn get_stored_prices() -> Vec<(String, f64)> {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});

    let prices = STABLE_STATE.with(|s|{
        let ptr = s.borrow();
        let state = ptr.as_ref().unwrap();
        state.get_all_previous_prices()
    });

    let mut ret: Vec<(String, f64)> = Vec::new();
    for pt in prices {
        ret.push((pt.cross.to_string().unwrap(), pt.price))
    }
    ret
}

#[query]
fn get_total_alerts_sent() -> u64 {
    // check admin
    RUNTIME_STATE.with(|state| {state.borrow().data.check_admin(ic_cdk::caller().to_text());});

    RUNTIME_STATE.with(|s|{
        s.borrow().stats.metrics.get_alerts_sent()
    })
}
