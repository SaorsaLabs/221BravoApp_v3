use std::ops::Add;
use candid::CandidType;
use ic_stable_memory::{derive::{StableType, AsFixedSizeBytes}, collections::{SBTreeMap, SVec}};
use serde::{Deserialize, Serialize};
use crate::core::{runtime::RUNTIME_STATE, stable_memory::{Main, STABLE_STATE}, utils::log};
use super::custom_types::SmallTX;

// Stable Store of account indexed data
#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct AccountTree{
    pub accounts: SBTreeMap<u64, Overview>,
    pub accounts_history: SBTreeMap<u64, HistoryCollection>,
    count: u64, // not used
    last_updated: u64, // not used
}

impl AccountTree {

    fn update_history_balance(&mut self, account_ref: &u64, stx: &SmallTX, tx_type: TransactionType) {
        let day_sum = stx.time as f64/ (86400f64 * 1_000_000_000f64);
        let day_of_transaction = day_sum.floor() as u64;
        let fee: u128;
        if let Some(f) = stx.fee { fee = f } else {
            fee = RUNTIME_STATE.with(|s|{s.borrow().data.get_ledger_fee()})
        };

        // new account
        if !self.accounts_history.contains_key(&account_ref) {
            if tx_type == TransactionType::In {
                let mut history_vec: SVec<HistoryData> = SVec::new();
                let init_balance = HistoryData{ day: day_of_transaction, balance: stx.value, time_of_update: stx.time };
                history_vec.push(init_balance).expect("memory is full");
                let new_collection = HistoryCollection{ collection: history_vec };
                self.accounts_history.insert(*account_ref, new_collection).expect("Storage is full");;
            } else {
                // Cant send tokens from a new account (0 balance!)
                let er = format!("ERROR - fn update_history_balance (block {}) is trying to debit an account which doesn't exist!", stx.block);
                log(er);
            }
        } else {
        // existing account
            if let Some(mut ach) = self.accounts_history.get_mut(account_ref) {
                // check for matching day
                let mut found = false;
                let mut found_idx = 0;
                let mut idx: usize = 0;
                for elem in ach.collection.iter() {
                    if elem.day == day_of_transaction {
                        found = true;
                        found_idx = idx;
                        break;
                    }
                    idx += 1;
                }
                match found {
                    true => {
                        // update current day balance
                        let mut entry = ach.collection.get_mut(found_idx).unwrap(); 
                        entry.balance = match tx_type {
                            TransactionType::In => entry.balance.saturating_add(stx.value),
                            TransactionType::Out => entry.balance.saturating_sub(stx.value).saturating_sub(fee)
                        };
                    },
                    false => {
                        // start new day
                        let last_entry = ach.collection.len()-1;
                        let last_balance = ach.collection.get(last_entry).unwrap().balance;                         
                        let new_balance = match tx_type {
                            TransactionType::In => last_balance.saturating_add(stx.value),
                            TransactionType::Out => last_balance.saturating_sub(stx.value).saturating_sub(fee)
                        };
                        ach.collection.push(HistoryData { day: day_of_transaction, balance: new_balance, time_of_update: stx.time }).expect("Storage is full");
                    }
                }
            }
        }
    }

    pub fn process_transfer_to(&mut self, account_ref: &u64, stx: &SmallTX) -> Result<String, String> {
      self.update_history_balance(account_ref, stx, TransactionType::In);

      if !self.accounts.contains_key(account_ref) {
         let acd = Overview { 
                  first_active: stx.time, 
                  last_active: stx.time, 
                  sent: (0_u32, 0_u128), 
                  received: (1_u32, stx.value), 
                  balance: stx.value 
               };

         self.accounts.insert(*account_ref, acd).expect("Storage is full");
         return Ok("Transfer Processed".to_string())

      } else if let Some(mut ac) = self.accounts.get_mut(account_ref) {
         ac.credit_account(stx.time, stx.value);
        
         return Ok("Transfer Processed".to_string())
      } else {
        let error = 
        format!("Error - cannot process transfer to, Block: {}", stx);
        return Err(error);
      }
    }

    pub fn process_transfer_from(&mut self, account_ref: &u64, stx: &SmallTX ) -> Result<String, String> {
        self.update_history_balance(account_ref, stx, TransactionType::Out);

        match self.accounts.get_mut(account_ref) {
            Some(mut ac) => {
                let fee: u128;
                if let Some(f) = stx.fee { fee = f } else {
                  fee = RUNTIME_STATE.with(|s|{s.borrow().data.get_ledger_fee()})
                };
                ac.debit_account(stx.time, stx.value, fee);
                return Ok("Processed OK".to_string());
            },
            None => { 
               let error = 
               format!("Error - cannot send from a non-existent account (process_transfer_from), Block: {}", stx);
               return Err(error);
            },
        }
    }

    pub fn process_approve_from(&mut self, account_ref: &u64, stx: &SmallTX ) -> Result<String, String> {
        let stx_zero_value = SmallTX {
            value: 0,
            ..*stx
        };
        self.update_history_balance(account_ref, &stx_zero_value, TransactionType::Out);

        match self.accounts.get_mut(account_ref) {
            Some(mut ac) => {
               let fee: u128;
               if let Some(f) = stx.fee { fee = f } else {
                 fee = RUNTIME_STATE.with(|s|{s.borrow().data.get_ledger_fee()})
               };
                ac.debit_account(stx.time, 0, fee);
                return Ok("Approve Processed".to_string());
            },
            None => { return Err("Error - cannot send from a non-existent account (process_approve_from)".to_string())},
        }
    }
}

#[derive(CandidType, StableType, Deserialize, Serialize, Clone, Default, AsFixedSizeBytes, Debug)]
pub struct HistoryData {
    pub day: u64,
    pub balance: u128,
    pub time_of_update: u64
}
impl Add for HistoryData {
    type Output = HistoryData;

    fn add(self, other: Self) -> Self::Output {
        HistoryData {
            day: self.day,
            balance: self.balance + other.balance,
            time_of_update: self.time_of_update
        }
    }
}
#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct HistoryCollection{
    pub collection: SVec<HistoryData> 
}

// TODO: Move out of here
#[derive(CandidType, StableType, Deserialize, Serialize, Clone, Default, Debug)]
pub struct GetAccountBalanceHistory {
    pub account: String,
    pub days: u64,
    pub merge_subaccounts: bool,
}
#[derive(PartialEq)]
enum TransactionType {
    In,
    Out
}
#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct AccountData {
    pub overview: Overview
 }

 #[derive(CandidType, StableType, Deserialize, Serialize, Clone, Default, AsFixedSizeBytes, Debug)]
 pub struct Overview {
    pub first_active: u64,
    pub last_active: u64,
    pub sent: (u32, u128), // count, value
    pub received: (u32, u128), // count, value
    pub balance: u128,
 }
 impl Overview {
    pub fn debit_account(&mut self, time:u64, value: u128, tx_fee: u128){
        if self.first_active == 0 || time < self.first_active { self.first_active = time }
        if self.last_active < time { self.last_active = time }

        // update balances
        let total_deduction = value.saturating_add(tx_fee);
        self.balance = self.balance.saturating_sub(total_deduction);
        let (mut s1, mut s2) = self.sent;
        s1 += 1;
        s2 = s2.saturating_add(total_deduction);
        self.sent = (s1,s2);
    }

    pub fn credit_account(&mut self, time:u64, value: u128){
        if self.first_active == 0 || time < self.first_active { self.first_active = time }
        if self.last_active < time {self.last_active = time}

        // update balances
        self.balance = self.balance.saturating_add(value);
        let (mut r1, mut r2) = self.received;
        r1 += 1;
        r2 = r2.saturating_add(value);
        self.received = (r1,r2);
    }
 }


pub fn fill_missing_days(mut history: Vec<(u64, HistoryData)>, time_now: u64, days: u64) -> Vec<HistoryData> {
    if history.len() == 0 { return Vec::new() }
    history.sort_by_key(|&(day, _)| day);
    let last_entry = history.last().unwrap(); 
    let mut filled_history = Vec::new();
    let mut last_data: Option<&HistoryData> = None;
    let day_sum =  time_now as f64/ (86400f64 * 1_000_000_000f64);
    let current_day = day_sum.floor() as u64;
    let mut last_day = 1;

    for day_offset in 0..=days {
        if last_day == 0 { break }
        let day = current_day.saturating_sub(day_offset);
        match history.iter().find(|&&(d, _)| d == day) {
            Some(&(_, ref data)) => {
                filled_history.push((data.clone()));
                last_data = Some(data);
            }
            None => {
                if let Some(data) = last_data {
                    filled_history.push((data.clone()));
                } else {
                    // day is prior to last balance change
                    filled_history.push(
                        HistoryData{ day: day.clone(), balance: last_entry.1.balance.clone(), time_of_update: last_entry.1.time_of_update }
                    );
                }
            }
        }
        last_day = day;
    }

    filled_history
}

// for get_account_history method
pub fn get_account_last_days(args: GetAccountBalanceHistory) -> Vec<(u64, HistoryData)> {
    // get ac_ref
    let ac_ref = STABLE_STATE.with(|s| {
        s.borrow().as_ref().unwrap().directory_data.get_ref(&args.account)
    });
    match ac_ref {
        Some(ac_ref_value) => {
            let result = STABLE_STATE.with(|s| {
                let mut items: Vec<(u64, HistoryData)> = Vec::new();

                let stable_state = s.borrow();
                let state_ref = stable_state.as_ref().unwrap();

                let history_map = if args.merge_subaccounts {
                    &state_ref.principal_data.accounts_history
                } else {
                    &state_ref.account_data.accounts_history
                };

                // copy from SVec into Vec
                if let Some(balance_data) = history_map.get(&ac_ref_value){
                    for db in balance_data.collection.iter() {
                        items.push((db.day.clone(), db.clone()));
                    }
                }

                // take latest X entries
                let req_days = args.days as usize;
                let res_vec = if req_days <= items.len() {
                    items.split_off(items.len() - req_days)
                } else {
                    // return all (data is less than args.days)
                    return items;
                };
                // return latest args.days number of entries. 
                return res_vec; 
            });
            return result;
        }
        None => {
           // log("return type 0, no ac_ref"); removed for testing
            let ret: Vec<(u64, HistoryData)> = Vec::new();
            ret
        }
    }
}

