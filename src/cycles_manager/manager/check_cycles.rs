use crate::core::{utils::{canister_call, log}, runtime::RUNTIME_STATE};


pub async fn check_cycles_impl(canster_list: Vec<String>) {
    for cnstr in canster_list {
        let cycle_balance: Result<(u64,), (ic_cdk::api::call::RejectionCode, String)> 
        = canister_call(cnstr.as_str(), "get_cycles_balance", (), None).await;
        match cycle_balance {
            Ok(v) => {
                let tl = RUNTIME_STATE.with(|s|{s.borrow().data.get_topup_level()});
                // is below topup level?
                if v.0 < tl {
                    // add cycles
                    let ta = RUNTIME_STATE.with(|s|{s.borrow().data.get_topup_amount()});
                    let deposit_call: Result<((),), (ic_cdk::api::call::RejectionCode, String)> 
                    = canister_call(cnstr.as_str(), "deposit_cycles", (), Some(ta as u128)).await;
                    match deposit_call {
                        Ok(_) => { log(format!("Cycles added to canister - {}", cnstr)); },
                        Err(e) => { log(format!("Error (check_cycles_impl :: deposit_call ) - {:?}, {}",e.0, e.1));} 
                    }
                }
            },
            Err(e) => {
                log(format!("Error (check_cycles_impl) - {:?}, {}",e.0, e.1));
            }
        }
    }
    log("Checked all canisters");
}