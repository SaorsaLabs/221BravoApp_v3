use candid::CandidType;
use ic_stable_memory::{collections::{SBTreeMap, SVec}, derive::{AsFixedSizeBytes, StableType}};
use serde::{Deserialize, Serialize};

use crate::core::{constants::MAX_USER_ALERTS, types::IDKey};

#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct PriceDataTree{
    pub price_alert_keys: SVec<IDKey>,
    pub price_alerts: SBTreeMap<IDKey, AlertCollection>,  // cross, alert collection
    pub user_data: SBTreeMap<IDKey, AlertCollection>,   // enc user, alert collection
    pub next_id: u64
}

impl PriceDataTree {
    pub fn add_alert(&mut self, mut data: InputAlert) -> Result<u64, String> {
        data.id = self.next_id;
        let mut pa_res: Result<u64, String> = Err("Alert not added to PA Map".to_string());
        let mut ud_res: Result<u64, String> = Err("Alert not added to UD Map".to_string());
        
        // Convert to use IDKey for stable storage. 
        let input_data = Alert {
            id: data.id,
            user: IDKey::from_string(data.user).unwrap(),
            cross: IDKey::from_string(data.cross).unwrap(),
            oc_id: IDKey::from_string(data.oc_id).unwrap(),
            price: data.price,
            direction: data.direction,
        };

        // check if user is known
        match self.user_data.contains_key(&input_data.user.clone()){
            true => {
                if let Some(mut ud) = self.user_data.get_mut(&input_data.user){
                    // check number of existing alerts
                    if ud.alerts.len() > MAX_USER_ALERTS { return Err("Max User Alerts".to_string())}
                    ud.alerts.push(input_data.clone()).expect("Memory is full");
                    ud_res = Ok(self.next_id);
                }
            },
            false => {
                let mut at_col = AlertCollection {
                    alerts: SVec::new(),
                };
                at_col.add(input_data.clone());
                match self.user_data.insert(input_data.user.clone(), at_col) {
                    Ok(_v) => {
                        ud_res = Ok(self.next_id);
                    },
                    _ => {} // do nothing - pa_res is set to Err on init
                }
            }
        }

        // check if cross is known
        match self.price_alerts.contains_key(&input_data.cross.clone()){
            true => {
                if let Some(mut ac) = self.price_alerts.get_mut(&input_data.cross){
                    ac.alerts.push(input_data.clone()).expect("Memory is full");
                    pa_res = Ok(self.next_id);
                }
            },
            false => {
                let mut at_col = AlertCollection {
                    alerts: SVec::new(),
                };
                at_col.add(input_data.clone());
                match self.price_alerts.insert(input_data.cross.clone(), at_col) {
                    Ok(_v) => {
                        pa_res = Ok(self.next_id);
                    },
                    _ => {} // do nothing - pa_res is set to Err on init
                }
                self.price_alert_keys.push(input_data.cross.clone()); // add to key vec
            }
        }

        match (pa_res, ud_res) {
            (Ok(v), Ok(_v2)) => { 
                self.next_id += 1;
                Ok(v)
            },
            (Err(_e), Ok(_v)) => { Err("Error saving data to Price Alert Map".to_string()) },
            (Ok(_v), Err(_e)) => { Err("Error saving data to User Data Map".to_string()) },
            _ => { Err("Error saving data to User Data and Price Alert Map".to_string()) },
        }
    }

    pub fn remove_alert(&mut self, data: InputAlert) -> String {
        let res1;
        let res2;
        let cross_alt = IDKey::from_string(data.cross).unwrap();
        let user_alt = IDKey::from_string(data.user).unwrap();

        match self.user_data.get_mut(&user_alt) {
            Some(mut v) => { 
                v.remove_by_id(data.id.clone());
                res1 = "Alert removed from User Map".to_string();
             },
            None => { res1 = "Error - User not found in User Map".to_string(); }
        }
        match self.price_alerts.get_mut(&cross_alt) {
            Some(mut v) => { 
                v.remove_by_id(data.id.clone());
                res2 = "Alert removed from Price Map".to_string();
             },
            None => { res2 = "Error - Cross not found in Price Map".to_string();}
        }

        String::from(format!("{}. {}", res1, res2))
    }

    pub fn get_all_alerts_by_user(&self, user: IDKey) -> Option<Vec<InputAlert>> {
        if let Some(ud) = self.user_data.get(&user){
            let mut ret_vec: Vec<InputAlert> = Vec::new();
            let mut temp: InputAlert;
            for alt in ud.alerts.iter() {
                temp = InputAlert{
                    id: alt.id,
                    user: alt.user.to_string().unwrap(),
                    cross: alt.cross.to_string().unwrap(),
                    oc_id: alt.oc_id.to_string().unwrap(),
                    price: alt.price,
                    direction: alt.direction,
                };
                ret_vec.push(temp.clone());
            }
            return Some(ret_vec);
        }
        None
    }

    pub fn get_all_alerts_by_cross(&self, cross: IDKey) -> Option<Vec<InputAlert>> {
        if let Some(pa) = self.price_alerts.get(&cross){
            let mut ret_vec: Vec<InputAlert> = Vec::new();
            let mut temp: InputAlert;
            for alt in pa.alerts.iter() {
                temp = InputAlert{
                    id: alt.id,
                    user: alt.user.to_string().unwrap(),
                    cross: alt.cross.to_string().unwrap(),
                    oc_id: alt.oc_id.to_string().unwrap(),
                    price: alt.price,
                    direction: alt.direction,
                };
                ret_vec.push(temp.clone());
            }
            return Some(ret_vec);
        }
        None
    }

    pub fn remove_key_from_vec(&mut self, cross: IDKey) -> String {
        let mut pos:usize = 0;
        let mut found_pos = -1_i32;
        let mut found = false;
        for key in self.price_alert_keys.iter() {
            if key.0 == cross.0 {
                found = true;
                found_pos = pos as i32;
            }
            pos += 1;
        }
        if found == true && found_pos != -1 {  
            self.price_alert_keys.remove(found_pos as usize);
            return String::from(format!("Key {} has been removed from key vec", cross.to_string().unwrap()));
        }
        return String::from(format!("Could not find key {} in key vec", cross.to_string().unwrap()));
    }

    pub fn get_all_cross_keys(&self) -> Vec<IDKey> {
        let mut ret_vec: Vec<IDKey> = Vec::new();
        for key in self.price_alert_keys.iter(){
            ret_vec.push(key.clone())
        }
        ret_vec
    }
}

#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct AlertCollection{
    alerts: SVec<Alert>
}
impl AlertCollection {
    pub fn add(&mut self, data: Alert){
        self.alerts.push(data).expect("Memory is full");
    }
    pub fn remove_by_id(&mut self, alert_id: u64) -> String {
        let mut pos:usize = 0;
        let mut found_pos = -1_i32;
        let mut found = false;
        for alt in self.alerts.iter() {
            if alt.id == alert_id {
                found = true;
                found_pos = pos as i32;
            }
            pos += 1;
        }
        if found == true && found_pos != -1 {  
            self.alerts.remove(found_pos as usize);
            return String::from(format!("Alert {} has been removed", alert_id));
        }
        return String::from(format!("Could not find alert {}", alert_id));
    }
}

#[derive(StableType, AsFixedSizeBytes, Debug, Default, Clone, Deserialize, CandidType)]
pub struct Alert{
    pub id: u64,
    pub user: IDKey,
    pub cross: IDKey,
    pub oc_id: IDKey,
    pub price: f64,
    pub direction:u8 // 0 down, 1 up
}

#[derive(Debug, Default, Clone, Deserialize, CandidType, Serialize)]
pub struct InputAlert{
    pub id: u64,
    pub user: String,
    pub cross: String,
    pub oc_id: String,
    pub price: f64,
    pub direction: u8 // 0 down, 1 up
}