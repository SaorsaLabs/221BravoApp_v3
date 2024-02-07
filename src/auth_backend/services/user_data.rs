use std::{collections::BTreeMap, borrow::Cow};
use candid::{CandidType, Encode, Decode};
use ic_stable_structures::{Storable, storable::Bound};
use serde::{Serialize, Deserialize};
use crate::core::stable_memory::USER_MAP;

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct UserData {
    pub user_account: String,
    pub user_name: String,
    pub user_oc_principal: Option<String>,
    pub user_tokens: u32,
    pub user_saved_accounts: BTreeMap<String, String> // account, name. 
}

impl UserData {
    pub fn new() -> UserData {
        UserData {
            user_account: String::new(),
            user_name: String::new(),
            user_oc_principal: None,
            user_tokens: 0,
            user_saved_accounts: BTreeMap::new(),
        }
    }
}

impl Storable for UserData {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 1500000, // min 915_469 but using more to allow for upgrades
        is_fixed_size: false,
    };
}

pub fn add_new_user_impl(user_account: String) -> String {
    let mut ud = UserData::new();
    let check_exists = USER_MAP.with(|s|{
        s.borrow().get(&user_account)
    });
    match check_exists {
        Some(_value) => {
            return String::from("Account already exists");
        }
        None => {
            ud.user_account = String::from(&user_account);
            ud.user_name = String::from("Anon User");
            USER_MAP.with(|s|{
                s.borrow_mut().insert(user_account, ud)
            });
            return String::from("Account added");
        }
    }
}

pub fn get_user_data_impl(user_account: String) -> Option<UserData> {
    USER_MAP.with(|s|{
        s.borrow().get(&user_account)
    })
}

pub fn get_user_named_accounts_impl(owner_account: String, query_vec: Vec<String>) -> Option<Vec<(String, String)>> {
    let mut ret: Vec<(String, String)> = Vec::new();
    let mut hits: bool = false;
    let all_acs = USER_MAP.with(|s|{
       s.borrow().get(&owner_account)
    });
    match all_acs {
         Some(value) => {
            // LOOP!
             for id in query_vec {
                 match value.user_saved_accounts.get(&id){
                     Some(inner_value) => {
                                 ret.push((id.clone(), inner_value.clone()));
                                 hits = true;
                     }
                     None => {}
                 }
            }
         },
         None => { return None}, 
    }
    if hits == true {return Some(ret);} else { return None };
}

pub fn get_all_user_named_accounts_impl(owner_account: String) -> Option<Vec<(String, String)>> {
   let mut ret: Vec<(String, String)> = Vec::new();
   let mut hits: bool = false;
   let user_data = USER_MAP.with(|s|{
        s.borrow().get(&owner_account)
        });
   match user_data {
        Some(usr_data) => {
            hits = true;
            for (key, value) in usr_data.user_saved_accounts.iter() {
                ret.push((key.to_owned(), value.to_owned()));
            }
        },
        None => {}
   }
   if hits == true {return Some(ret);} else { return None };
}

pub fn add_user_named_accounts_impl(owner_account: String, save_account: String, save_name: String) -> String {
    let known = USER_MAP.with(|s|{
        match s.borrow().get(&owner_account) {
            Some(_value) => {
                return true;
            }
            None => {
                return false;
            }
        }
    });

    match known {
        true => {
            let data = USER_MAP.with(|s|{
                s.borrow().get(&owner_account)
            });

            match data {
                Some(mut value) => {
                    value.user_saved_accounts.insert(save_account, save_name);
                    USER_MAP.with(|s|{
                        s.borrow_mut().insert(owner_account, value)
                    });
                    return String::from("Account added to address book");
                },
                None => {
                    return String::from("User account doesn't exist");
                },
            }
        },
        false => {
            return String::from("User account doesn't exist");
        }
    }
}

pub fn remove_user_named_accounts_impl(owner_account: String, save_account: String) -> String {
    let known = USER_MAP.with(|s|{
        match s.borrow().get(&owner_account) {
            Some(_value) => {
                return true;
            }
            None => {
                return false;
            }
        }
    });

    match known {
        true => {
            let data = USER_MAP.with(|s|{
                s.borrow().get(&owner_account)
            });

            match data {
                Some(mut value) => {
                    value.user_saved_accounts.remove(&save_account);
                    USER_MAP.with(|s|{
                        s.borrow_mut().insert(owner_account, value)
                    });
                    return String::from("Account removed from address book");
                },
                None => {
                    return String::from("User account doesn't exist");
                },
            }
        },
        false => {
            return String::from("User account doesn't exist");
        }
    }
}

pub fn update_username_impl(user_account: String, user_name: String) -> String {
    let is_some = USER_MAP.with(|s|{
        s.borrow().get(&user_account).is_some()
    });

    match is_some {
        true => {
            USER_MAP.with(|s|{
                let data = s.borrow().get(&user_account);
                match data {
                    Some(mut value) => {
                        value.user_name = user_name;
                        s.borrow_mut().insert(user_account, value);
                        return String::from("Username updated");
                    },
                    None => {
                        // should never get here!
                        return String::from("Can't update username - Account doesn't exist");
                    } 
                }
            })
        },
        false => {
            return String::from("Can't update username - Account doesn't exist");
        }
    }
}

pub fn add_user_tokens_impl(user_account: String, user_tokens: u32) -> String {
    let is_some = USER_MAP.with(|s|{
        s.borrow().get(&user_account).is_some()
    });

    match is_some {
        true => {
            USER_MAP.with(|s|{
                let data = s.borrow().get(&user_account);
                match data {
                    Some(mut value) => {
                        value.user_tokens += user_tokens;
                        s.borrow_mut().insert(user_account, value);
                        return String::from("Tokens updated");
                    },
                    None => {
                        // should never get here!
                        return String::from("Can't update tokens - Account doesn't exist");
                    } 
                }
            })
        },
        false => {
            return String::from("Can't update tokens - Account doesn't exist");
        }
    }
}

pub fn backup_user_named_accounts_impl() -> Option<Vec<(String, String, String)>> {
   let mut op_vec: Vec<(String, String, String)> = Vec::new();
   USER_MAP.with(|s|{
    for (_key, value) in s.borrow().iter() {
        let owner = value.user_account.clone();
        for (k2, v2) in value.user_saved_accounts.iter() {
            op_vec.push((owner.clone(), k2.clone(), v2.clone()));
        }
    }
   });
   return Some(op_vec);
}