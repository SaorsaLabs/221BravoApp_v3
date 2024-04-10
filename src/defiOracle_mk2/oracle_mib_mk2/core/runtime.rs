use std::cell::RefCell;
use candid::{CandidType, Principal};
use oracle_shared_mk2::shared_types::{TokenSwaps, InternalRates, Marketplace};
use serde::{Deserialize, Serialize};
use super::constants::MAX_LOGS;
use super::types::{LogEntry, WorkingStats};

thread_local! {
    // Runtime state (not stored in stable memory)
    pub static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default(); 
}

#[derive(Serialize, Deserialize)]
pub struct RuntimeState {
    pub data: Data
}

impl Default for RuntimeState {
    fn default() -> Self {
        RuntimeState {
            data: Data::default()
        }
    }
}

#[derive(CandidType, Deserialize, Serialize, Default)]
pub struct Data {
    admin: Vec<String>,
    authorised: Vec<String>,
    canister_logs: Vec<LogEntry>,
    self_id: Option<String>,
    working_stats: WorkingStats,
    pub assigned_crosses: TokenSwaps,
    pub cpc_link_active: bool,
    pub internal_rates: InternalRates,
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

    pub fn set_self_id(&mut self, principal_id: Principal) -> String {
        let principal = principal_id.to_text();
        self.self_id = Some(principal);
        return "Self-Principal Saved".to_string();
    }

    pub fn get_self_id(&self) -> String {
        return self.self_id.clone().unwrap();
    }

    pub fn get_working_stats(&self) -> WorkingStats {
        return self.working_stats.clone();
    }

    pub fn update_working_stats(&mut self, time: Option<u64>, exchange: Option<Marketplace>, add_cross: Option<String>, remove_cross: Option<String> ) -> String {
        if let Some(time_value) = time {
            self.working_stats.last_update_time = time_value;
        }
        if let Some(ex_value) = exchange {
            self.working_stats.assigned_marketplace = ex_value;
        }
        if let Some(cx_value) = add_cross {
            self.working_stats.assigned_crosses.push(cx_value);
        }
        if let Some(dx_value) = remove_cross {
            if self.working_stats.assigned_crosses.contains(&dx_value){
                self.working_stats.assigned_crosses.retain(|x: &String| x != &dx_value);
            }
        }
        return "Working stats updated".to_string();
    }

    pub fn get_assigned_marketplace(&self) -> Marketplace {
        return self.working_stats.assigned_marketplace.clone();
    }
}