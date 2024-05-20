use std::collections::HashMap;

use ic_cdk_macros::{update, query};
use ic_cdk::api::time as ic_time;

use crate::core::{runtime::RUNTIME_STATE, stable_memory::STABLE_STATE, working_stats::api_count, utils::log};

use super::{
    account_tree::{GetAccountBalanceHistory, HistoryData, Overview}, active_accounts::ActivitySnapshot, constants::HOUR_AS_NANOS, custom_types::{HolderBalance, HolderBalanceResponse, IndexerType, ProcessedTX, TimeStats, TotalHolderResponse}, directory::lookup_directory, fetch_data::{
        dfinity_icp::{t1_impl_set_target_canister, SetTargetArgs}, 
        dfinity_icrc2::t2_impl_set_target_canister, meme_icrc::{add_pre_mint_to_ledger, t3_impl_set_target_canister}
    }, process_data::process_time_stats::{calculate_time_stats, StatsType}
};

// [][] -- ADMIN GATED -- [][]
#[update]
pub async fn init_target_ledger(args: SetTargetArgs, index_type: IndexerType) -> String {
    // check admin
    RUNTIME_STATE.with(|s|{s.borrow().data.check_admin(ic_cdk::caller().to_text())});
    // select route
    match index_type {
        IndexerType::DfinityIcp => {
            let res = t1_impl_set_target_canister(args).await;
            match res {
                Ok(v) => {
                    RUNTIME_STATE.with(|s|{s.borrow_mut().data.set_index_type(index_type)}); 
                    return v;
                },
                Err(e) => { return e}
            }
        },
        IndexerType::DfinityIcrc2 => {
            let res = t2_impl_set_target_canister(args).await;
            match res {
                Ok(v) => { 
                    RUNTIME_STATE.with(|s|{s.borrow_mut().data.set_index_type(index_type)}); 
                    return v;
                },
                Err(e) => { return e}
            }
        },
        IndexerType::MemeIcrc => {
            let res = t3_impl_set_target_canister(args).await;
            match res {
                Ok(v) => { 
                    RUNTIME_STATE.with(|s|{s.borrow_mut().data.set_index_type(index_type)}); 
                    return v;
                },
                Err(e) => { return e}
            }
        }
    }
}

//[][] -- ADMIN GATED - TESTS/ FIXES -- [][]
#[update]
fn fix_exe_missing_blocks_issue(next_block: u64){
    // check admin
    RUNTIME_STATE.with(|s|{s.borrow().data.check_admin(ic_cdk::caller().to_text())});
    RUNTIME_STATE.with(|s|{s.borrow_mut().stats.set_next_block(next_block)});
}

#[update] // fix for meme pre-mint issue
async fn init_pre_mint(to_account: String, tx_value: u64) -> String {
    // check admin
    RUNTIME_STATE.with(|s|{s.borrow().data.check_admin(ic_cdk::caller().to_text())});
    add_pre_mint_to_ledger(to_account, tx_value as u128).await
}

// [][] -- AUTH GATED -- [][]
// total holders ☑️
// top holders ☑️
// account balance  ☑️
// principal balance ☑️
// get hourly stats ☑️
// get daily stats ☑️

#[update]
pub fn get_top_account_holders(number_to_return: u64) -> Vec<HolderBalanceResponse>{
    // check authorised
    RUNTIME_STATE.with(|s|{s.borrow().data.check_authorised(ic_cdk::caller().to_text())});

    let top: Vec<HolderBalance> = STABLE_STATE.with(|s|{
        let mut ac_vec: Vec<HolderBalance> = Vec::new();
        for ac in s.borrow().as_ref().unwrap().account_data.accounts.iter() {
            let refs = ac.0.clone();
            let ov = ac.1.clone();
            ac_vec.push(
                HolderBalance {
                    holder: refs,
                    data: ov
                }
            )
        }

        // catch 0 result
        if ac_vec.len() == 0 { return ac_vec }

        ac_vec.sort_unstable_by_key(|element| element.data.balance);
        ac_vec.reverse();
        let mut top_ac: Vec<HolderBalance> = Vec::new();
        for i in 0..number_to_return as usize {
            top_ac.push(ac_vec[i].to_owned());
        }
        api_count();
        return top_ac;
    });

    // replace ref with full accounts
    let mut return_data: Vec<HolderBalanceResponse> = Vec::new();
    for ad in top {
        let ac = lookup_directory(ad.holder);
        match ac {
            Some(v) => {return_data.push(
                HolderBalanceResponse{
                    holder: v,
                    data: ad.data,
                }
            )},
            None => {} // do nothing
        }
    }
    api_count();
    return return_data;
}


#[update]
pub fn get_top_principal_holders(number_to_return: u64) -> Vec<HolderBalanceResponse>{
    // check authorised
    RUNTIME_STATE.with(|s|{s.borrow().data.check_authorised(ic_cdk::caller().to_text())});

    let top: Vec<HolderBalance> = STABLE_STATE.with(|s|{
        let mut ac_vec: Vec<HolderBalance> = Vec::new();
        for ac in s.borrow().as_ref().unwrap().principal_data.accounts.iter() {
            let refs = ac.0.clone();
            let ov = ac.1.clone();
            ac_vec.push(
                HolderBalance {
                    holder: refs,
                    data: ov
                }
            )
        }

        // catch 0 result
        if ac_vec.len() == 0 { return ac_vec }
        
        ac_vec.sort_unstable_by_key(|element| element.data.balance);
        ac_vec.reverse();
        let mut top_ac: Vec<HolderBalance> = Vec::new();
        for i in 0..number_to_return as usize {
            top_ac.push(ac_vec[i].to_owned());
        }
        api_count();
        return top_ac;
    });

    // replace ref with full accounts
    let mut return_data: Vec<HolderBalanceResponse> = Vec::new();
    for ad in top {
        let ac = lookup_directory(ad.holder);
        match ac {
            Some(v) => {return_data.push(
                HolderBalanceResponse{
                    holder: v,
                    data: ad.data,
                }
            )},
            None => {} // do nothing
        }
    }
    api_count();
    return return_data;
}

#[query]
fn get_total_holders() -> TotalHolderResponse {
    // check authorised
    RUNTIME_STATE.with(|s|{s.borrow().data.check_authorised(ic_cdk::caller().to_text())});

    // account count
    let ac_len = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().account_data.accounts.len()
    });

    // principal count
    let pr_len = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().principal_data.accounts.len()
    });
    api_count();
    return TotalHolderResponse{
        total_accounts: ac_len,
        total_principals: pr_len,
    };
}

#[query]
fn get_hourly_stats() -> TimeStats {
    // check authorised
    RUNTIME_STATE.with(|s|{s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
    api_count();
    RUNTIME_STATE.with(|s|{
        s.borrow().data.hourly_stats.clone()
    })
}

#[query]
fn get_daily_stats() -> TimeStats {
    // check authorised
    RUNTIME_STATE.with(|s|{s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
    api_count();
    RUNTIME_STATE.with(|s|{
        s.borrow().data.daily_stats.clone()
    })
}

#[query]
pub fn get_account_overview(account: String) -> Option<Overview> {
    // check authorised
    RUNTIME_STATE.with(|s|{s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
    api_count();
    // get ac_ref
    let ac_ref = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().directory_data.get_ref(&account)
    });
    match ac_ref {
        Some(v) => {
            let ret = STABLE_STATE.with(|s|{
                match s.borrow().as_ref().unwrap().account_data.accounts.get(&v) {
                    Some(v) => {
                        let ov = v.to_owned();
                        return Some(ov);
                    },
                    None => { return None }
                }

            });
            return ret;
        },
        None => { return None }
    }  
}

#[query]
pub fn get_principal_overview(account: String) -> Option<Overview> {
    // check authorised
    RUNTIME_STATE.with(|s|{s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
    api_count();
    // get ac_ref
    let ac_ref = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().directory_data.get_ref(&account)
    });
    match ac_ref {
        Some(v) => {
            let ret = STABLE_STATE.with(|s|{
                match s.borrow().as_ref().unwrap().principal_data.accounts.get(&v) {
                    Some(v) => {
                        let ov = v.to_owned();
                        return Some(ov);
                    },
                    None => { return None }
                }

            });
            return ret;
        },
        None => { return None }
    }  
}

#[query]
pub fn get_activity_stats(days: u64) -> Vec<ActivitySnapshot> {
    // check authorised
    RUNTIME_STATE.with(|s|{s.borrow().data.check_authorised(ic_cdk::caller().to_text())});
    api_count();
    STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().activity_stats.get_daily_snapshots(days as usize)
    })
}
/// Update so I can see the logs
#[update]
fn get_account_history(args: GetAccountBalanceHistory) -> Vec<(u64, HistoryData)> {
    // check authorised
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text()) });
    api_count();
    let history = get_account_last_days(args.clone());
    let filled_history = fill_missing_days(history, args.days);
    return filled_history;
}
fn fill_missing_days(mut history: Vec<(u64, HistoryData)>, days: u64) -> Vec<(u64, HistoryData)> {
    history.sort_by_key(|&(day, _)| day);

    let mut filled_history = Vec::new();
    let mut last_data: Option<&HistoryData> = None;
    let current_day = ic_time() / (86400 * 1_000_000_000);

    for day_offset in 0..=days {
        let day = current_day - day_offset;

        match history.iter().find(|&&(d, _)| d == day) {
            Some(&(_, ref data)) => {
                filled_history.push((day, data.clone()));
                last_data = Some(data);
            }
            None => {
                if let Some(data) = last_data {
                    filled_history.push((day, data.clone()));
                }
            }
        }
    }

    filled_history
}
pub fn get_account_last_days(args: GetAccountBalanceHistory) -> Vec<(u64, HistoryData)> {
    // get ac_ref
    let ac_ref = STABLE_STATE.with(|s| {
        s.borrow().as_ref().unwrap().directory_data.get_ref(&args.account)
    });
    match ac_ref {
        Some(ac_ref_value) => {
            let result = STABLE_STATE.with(|s| {
                let mut items: HashMap<u64, HistoryData> = HashMap::new();
                let mut days_collected = 0;

                let current_day = ic_time() / (86400 * 1_000_000_000);
                let start_day = if current_day > args.days { current_day - args.days } else { 0 };

                let stable_state = s.borrow();
                let state_ref = stable_state.as_ref().unwrap();

                let history_map = if args.merge_subaccounts {
                    &state_ref.principal_data.accounts_history
                } else {
                    &state_ref.account_data.accounts_history
                };

                for day in (start_day..=current_day).rev() {
                    let key = (ac_ref_value, day);

                    if let Some(history) = history_map.get(&key) {
                        items.insert(day, history.clone());
                        days_collected += 1;
                        if days_collected >= args.days {
                            break;
                        }
                    }
                }
                let msg = format!("final items: {items:?}");
                log(msg);
                let vec: Vec<(u64, HistoryData)> = items
                    .iter()
                    .map(|(&k, v)| (k, v.clone()))
                    .collect();
                return vec;
            });
            return result;
        }
        None => {
            log("return type 0, no ac_ref");
            let ret: Vec<(u64, HistoryData)> = Vec::new();
            ret
        }
    }
}