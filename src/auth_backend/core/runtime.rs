use std::cell::RefCell;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use crate::services::top_tokens::{TokenData, TopHolderData, HolderBalanceResponse};

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
    pub top_tokens: Vec<TokenData>,
    pub top_holders: Vec<TopHolderData>
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

    pub fn add_token(&mut self, cross: String, ledger: String, decimals: u8, stats221: String) -> String {
        let token_vec:&mut Vec<TokenData> = &mut self.top_tokens;
        let top_holders: &mut Vec<TopHolderData> = &mut self.top_holders;

        for tkn in token_vec.clone() {
            if &tkn.cross == &cross { return String::from("Cross already exists"); }
        }
        let data = TokenData{
            cross: cross.clone(),
            ledger,
            decimals,
            change24: 0.0,
            change7d: 0.0,
            price: 0.0,
            supply: 0.0,
            mcap: 0.0,
            sparkline: Vec::new()
        };
        let holder_data = TopHolderData{
            cross,
            stats221,
            accounts: Vec::new(),
            principals: Vec::new(),
        };

        token_vec.push(data);
        top_holders.push(holder_data);
        return String::from("Cross added to processing list");
    }

    pub fn remove_token(&mut self, cross: String) -> String {
        let token_vec:&mut Vec<TokenData> = &mut self.top_tokens;
        let holder_vec: &mut Vec<TopHolderData> = &mut self.top_holders;
        let contains = token_vec.iter().any(|item| item.cross == cross);
        if contains == false { return String::from("Can't remove - Cross isn't in the processing list"); }
        token_vec.retain(|x| x.cross != cross);
        holder_vec.retain(|x| x.cross != cross);
        return String::from("Cross removed from processing list");
    }

    pub fn get_all_top_token_data(&self) -> Vec<TokenData> {
        return self.top_tokens.clone();
    }

    pub fn get_top_holders(&self, cross: String)-> Option<TopHolderData>{
        let hold_data = self.top_holders.clone();
        for top in hold_data{
            if top.cross == cross {
                return Some(top);
            }
        }
        return None;
    }   
}