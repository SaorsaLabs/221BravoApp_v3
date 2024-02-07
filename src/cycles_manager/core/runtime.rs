use std::cell::RefCell;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use super::constants::MAX_LOGS;
use super::types::LogEntry;
use super::working_stats::WorkingStats;

thread_local! {
    // Runtime state (not stored in stable memory)
    pub static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default(); 
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RuntimeState {
    pub data: Data,
    pub stats: WorkingStats
}

impl Default for RuntimeState {
    fn default() -> Self {
        RuntimeState {
            data: Data::default(),
            stats: WorkingStats::default()
        }
    }
}

#[derive(CandidType, Deserialize, Serialize, Default, Clone)]
pub struct Data {
    admin: Vec<String>,
    authorised: Vec<String>,
    canister_logs: Vec<LogEntry>,
    self_id: Option<String>,
    check_list: Vec<String>,
    topup_level: u64,
    topup_amount: u64,
    check_days: u8
}

impl Data {
    pub fn check_authorised(&self, principal_id: String){
        let auth_vec: &Vec<String> = &self.authorised;
        let mut auth:bool = false;
        if auth_vec.contains(&principal_id) {
            auth = true;
        }
        match auth {
            true => (),
            _ => ic_cdk::trap("Caller Not Authorised")
        }
    }

    pub fn add_authorised(&mut self, principal_id: String) -> String {
        let auth_vec:&mut Vec<String> = &mut self.authorised;
        if auth_vec.contains(&principal_id){
            let rtn:String = String::from("Principal is already authorised");
            return rtn;
        } else {
            auth_vec.push(principal_id);
        }
        let rtn:String = String::from("Principal authorised");
        return rtn;
    }

    pub fn remove_authorised(&mut self, principal_id: String) -> String {
        let auth_vec:&mut Vec<String> = &mut self.authorised;
        if auth_vec.contains(&principal_id){
            auth_vec.retain(|x: &String| x != &principal_id);
        } else {
            let rtn:String = String::from("Can't remove - Principal isn't in the list of authorised users");
            return rtn;
        }
        let rtn:String = String::from("Principal removed from authorised list");
        return rtn;
    }

    pub fn get_all_authorised(&self) -> Vec<String> {
        let auth_vec: &Vec<String> = &self.authorised;
        return auth_vec.to_owned();
    }
   
    pub fn check_admin(&self, principal_id: String){
        let auth_vec: &Vec<String> = &self.admin;
        let mut auth:bool = false;
        if auth_vec.contains(&principal_id) {
            auth = true;
        }
        match auth {
            true => (),
            _ => ic_cdk::trap("Caller Not Admin")
        }
    }

    pub fn add_admin(&mut self, principal_id: String) -> String {
        let auth_vec:&mut Vec<String> = &mut self.admin;
        if auth_vec.contains(&principal_id){
            let rtn:String = String::from("Principal is already admin");
            return rtn;
        } else {
            auth_vec.push(principal_id);
        }
        let rtn:String = String::from("Admin added");
        return rtn;
    }

    pub fn remove_admin(&mut self, principal_id: String) -> String {
        let auth_vec:&mut Vec<String> = &mut self.admin;
        if auth_vec.contains(&principal_id){
            auth_vec.retain(|x: &String| x != &principal_id);
        } else {
            let rtn:String = String::from("Can't remove - Principal isn't in the list of admins");
            return rtn;
        }
        let rtn:String = String::from("Admin removed");
        return rtn;
    }

    pub fn get_all_admins(&self) -> Vec<String> {
        let auth_vec: &Vec<String> = &self.admin;
        return auth_vec.to_owned();
    }

    pub fn add_log(&mut self, text: String){
        let nano_time: u64 = ic_cdk::api::time();
        let log_entry: LogEntry = LogEntry {
            timestamp: nano_time.to_string(),
            text,
            };
        self.canister_logs.push(log_entry);
        if &self.canister_logs.len() > &MAX_LOGS { self.canister_logs.remove(0);}
    }

    pub fn get_logs(&self) -> Option<Vec<LogEntry>>{
        match &self.canister_logs.len() {
            0 => {
                return None;
            },
            _ => {
                return Some(self.canister_logs.clone());
            }
        }
    }
    
    pub fn clear_logs(&mut self){
        self.canister_logs.clear();
    }

    pub fn set_self_id(&mut self, principal_id: Principal) -> String {
        let principal = principal_id.to_text();
        self.self_id = Some(principal);
        return "Self-Principal Saved".to_string();
    }

    pub fn get_self_id(&self) -> String {
        return self.self_id.clone().unwrap();
    }

    pub fn set_topup_level(&mut self, level: u64){
        self.topup_level = level;
    }

    pub fn get_topup_level(&self) -> u64 {
        return self.topup_level.clone();
    }

    pub fn set_topup_amount(&mut self, level: u64){
        self.topup_amount = level;
    }

    pub fn get_topup_amount(&self) -> u64 {
        return self.topup_amount.clone();
    }

    pub fn set_check_days(&mut self, days: u8){
        self.check_days = days;
    }

    pub fn get_check_days(&self) -> u8 {
        return self.check_days.clone();
    }

    pub fn add_canister(&mut self, principal_id: String) -> String {
        let can_vec:&mut Vec<String> = &mut self.check_list;
        if can_vec.contains(&principal_id){
            let rtn:String = String::from("Canister is already on the checking list");
            return rtn;
        } else {
            can_vec.push(principal_id);
        }
        let rtn:String = String::from("Canister added to checking list");
        return rtn;
    }

    pub fn remove_canister(&mut self, principal_id: String) -> String {
        let can_vec:&mut Vec<String> = &mut self.check_list;
        if can_vec.contains(&principal_id){
            can_vec.retain(|x: &String| x != &principal_id);
        } else {
            let rtn:String = String::from("Can't remove - Canister isn't on the checking list");
            return rtn;
        }
        let rtn:String = String::from("Canister removed from checking list");
        return rtn;
    }

    pub fn get_canister_checklist(&self) -> Vec<String> {
        return self.check_list.clone();    
    }

}