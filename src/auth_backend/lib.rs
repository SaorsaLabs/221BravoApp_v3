pub mod core;
pub mod services;
pub mod timers;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use crate::{
        services::{
            constants::MAX_USER_ADDRESSES, 
            user_data::{add_new_user_impl, add_user_named_accounts_impl, remove_user_named_accounts_impl, update_username_impl, get_user_data_impl, add_user_tokens_impl}, 
            public_accounts::{add_public_named_accounts_impl, get_all_public_named_accounts_impl, get_public_named_accounts_impl, remove_public_named_accounts_impl}, account_identifier::{get_single_account_impl, get_multiple_account_impl}
        }, 
        core::stable_memory::USER_MAP
    };

    #[test]
    fn userdata_size() {
        // TEST SIZES
        // Because the maximum size of a principal is 29 bytes, the textual representation will be no longer than 63 characters 
        // ICP Accounts - 32-byte string
        // ICRC1 - Also 32-byte string

        // pub user_account: String,
        // pub user_name: String,
        // pub user_oc_principal: Option<String>,
        // pub user_tokens: u32,
        // pub user_saved_accounts: BTreeMap<String, String> // account, name. 

        let empty_string = "".to_string(); // 24 bytes
        let empty_option_string: Option<String> = Some("".to_string()); // 24 bytes either as Some or None
        let empty_btree: BTreeMap<String, String> = BTreeMap::new(); // 24 bytes empty 

        let user_ac = std::mem::size_of_val(&empty_string) + (4 * 64); // based on 64 characters and UTF-8 encoding. In reality uses ASCII and is max 32 bytes
        let user_name = std::mem::size_of_val(&empty_string) + (4 * 20); // based on 20 characters and UTF-8 encoding.
        let user_saved_accounts = std::mem::size_of_val(&empty_btree);
        let oc_principal = std::mem::size_of_val(&empty_option_string) + 29; // principal is max 29 bytes.
        // btree map data
        let key = std::mem::size_of_val(&empty_string) + (4 * 64); // same as user_ac in reality likely to be 32bytes.
        let value: usize = std::mem::size_of_val(&empty_string)+ 62; // principal.subaccount = 29 bytes +32 bytes +1 for full stop

        let total_user_data = 
                user_ac + 
                user_name + 
                oc_principal + 
                8_usize + 
                user_saved_accounts + ((key + value)* MAX_USER_ADDRESSES);

        //println!("Total Size per user {}", total_user_data);
        assert_eq!(total_user_data, 915_469); 
    }

    #[test]
    fn test_add_user(){
        // add account
        let user = String::from("c17ceb800e15d4927d4a9c3bd4814d12819bbe6d641557c7a75dd901e86bc806");
        add_new_user_impl(user.clone());
        let len = USER_MAP.with(|s|{
            s.borrow().len()
        });
        let user_data = USER_MAP.with(|s|{
            s.borrow().get(&user)
        });
        let udres = user_data.unwrap();
        assert_eq!(len, 1_u64);
        assert_eq!(&udres.user_account, &"c17ceb800e15d4927d4a9c3bd4814d12819bbe6d641557c7a75dd901e86bc806");
        assert_eq!(&udres.user_name, &"Anon User");
        assert_eq!(&udres.user_oc_principal, &None);
        assert_eq!(&udres.user_tokens, &0);
        assert_eq!(&udres.user_saved_accounts.len(), &0);
    }

    #[test]
    fn test_add_remove_addressbook(){
        // add main account
        let user = String::from("c17ceb800e15d4927d4a9c3bd4814d12819bbe6d641557c7a75dd901e86bc806");
        add_new_user_impl(user.clone());
        let save_ac = String::from("d18ceb800e15d4927d4a9c3bd4814d12819bbe6d641557c7a75dd901e86bc900");
        let save_name = String::from("Test Account");
        // add address entry
        add_user_named_accounts_impl(user.clone(), save_ac.clone(), save_name.clone());
        let user_data = USER_MAP.with(|s|{
            s.borrow().get(&user)
        });
        // check
        let udres = user_data.unwrap();
        let entry = udres.user_saved_accounts.get(&save_ac);
        assert_eq!(entry, Some(&save_name));
        
        // remove address
        remove_user_named_accounts_impl(user.clone(), save_ac.clone());
        let user_data2 = USER_MAP.with(|s|{
            s.borrow().get(&user)
        });
        // check
        let udres2 = user_data2.unwrap();
        let entry2 = udres2.user_saved_accounts.get(&save_ac);
        assert_eq!(entry2, None);
    }

    #[test]
    fn test_public_named_accounts(){
        let user = String::from("c17ceb800e15d4927d4a9c3bd4814d12819bbe6d641557c7a75dd901e86bc806");
        let save_name = String::from("Test Account");
        add_public_named_accounts_impl(user, save_name); 

        let user2 = String::from("d19ceb800e15d4927d4a9c3bd4814d12819bbe6d641557c7a75dd901e86bc900");
        let save_name2 = String::from("Test Account 2");
        add_public_named_accounts_impl(user2, save_name2);

        let user3 = String::from("e22ceb800e15d4927d4a9c3bd4814d12819bbe6d641557c7a75dd901e86bc950");
        let save_name3 = String::from("Test Account 3");
        add_public_named_accounts_impl(user3, save_name3);

        // check
        let all = get_all_public_named_accounts_impl().unwrap();
        // println!("{:?}", all);
        assert_eq!(all.len(), 3);
        let v = vec![
            "c17ceb800e15d4927d4a9c3bd4814d12819bbe6d641557c7a75dd901e86bc806".to_string(),
            "e22ceb800e15d4927d4a9c3bd4814d12819bbe6d641557c7a75dd901e86bc950".to_string()];
        let double = get_public_named_accounts_impl(v).unwrap();
        //println!("{:?}", double);
        assert_eq!(double.len(), 2);

        // remove
        remove_public_named_accounts_impl("c17ceb800e15d4927d4a9c3bd4814d12819bbe6d641557c7a75dd901e86bc806".to_string());
        let all2 = get_all_public_named_accounts_impl().unwrap();
        // check
        //println!("{:?}", all2);
        assert_eq!(all2.len(), 2);
    }

    // update name + tokens.
    #[test]
    fn test_update_username_token(){
        // add main account
        let user = String::from("c17ceb800e15d4927d4a9c3bd4814d12819bbe6d641557c7a75dd901e86bc806");
        add_new_user_impl(user.clone());
        // update name
        update_username_impl(user.clone(), "New Name".to_string());
        let data = get_user_data_impl(user.clone()).unwrap();
        let name = data.user_name;
        assert_eq!(name, "New Name".to_string());
        // update tokens
        add_user_tokens_impl(user.clone(), 10);
        let data2 = get_user_data_impl(user.clone()).unwrap();
        let tokens = data2.user_tokens;
        assert_eq!(tokens, 10);
    }

    #[test]
    fn test_get_single_subaccount(){
        let ac = get_single_account_impl("2vxsx-fae".to_string(), 0);
        assert_eq!(ac, "1c7a48ba6a562aa9eaa2481a9049cdf0433b9738c992d698c31d8abf89cadc79".to_string());
        
        let acs = get_multiple_account_impl("2vxsx-fae".to_string(), 0, 2);
        assert_eq!(acs[0], "1c7a48ba6a562aa9eaa2481a9049cdf0433b9738c992d698c31d8abf89cadc79".to_string());
        assert_eq!(acs[1], "b8fab0be4ad596a3739ab93e7316a8647ee72e167709441da49ce9171828629d".to_string());
        assert_eq!(acs[2], "4e7a07d00341ce88e808dd846f6a6005ff63bc7fd6e46adca031271c91c1faf7".to_string());
    }
}
   