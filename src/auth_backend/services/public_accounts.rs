use crate::core::stable_memory::PUBLIC_MAP;

pub fn get_public_named_accounts_impl(input_vec: Vec<String>) -> Option<Vec<(String, String)>> {
    let mut ret:Vec<(String, String)> = Vec::new();
    let mut hits: bool = false;
    for id in input_vec {
        let res = PUBLIC_MAP.with(|s|{
            s.borrow().get(&id)
        });
        match res {
            Some(v) => {
                ret.push((id.clone(), v.clone()));
                hits = true;
            }, 
            None => {}
        }
    }
    if hits == true { return Some(ret); } else { return None };
}

pub fn add_public_named_accounts_impl(save_account: String, save_name: String) -> String {
    PUBLIC_MAP.with(|s|{
        s.borrow_mut().insert(save_account, save_name)
    });
    return "ID added to public account list".to_string();
}

pub fn remove_public_named_accounts_impl(save_account: String) -> String {
    let del = PUBLIC_MAP.with(|s|{
        s.borrow_mut().remove(&save_account)
    });
    match del {
        Some(_v) => {  return "ID Removed from public account list".to_string(); },
        None => { return "ID isnt in the public list - cannot remove".to_string(); }
    }
}

pub fn get_all_public_named_accounts_impl() -> Option<Vec<(String, String)>> {
    let mut ret:Vec<(String, String)> = Vec::new();
    PUBLIC_MAP.with(|s|{
        for (key, value) in s.borrow().iter() {
            ret.push((key.clone(), value.clone()));
        }
    });
    let mut hits: bool = false;
    if ret.len() > 0 { hits = true };
    if hits == true { return Some(ret); } else { return None };
}
