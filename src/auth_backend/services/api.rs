use ic_cdk_macros::{query, update};
use crate::core::runtime::RUNTIME_STATE;
use super::{
    account_identifier::{get_multiple_account_impl, get_single_account_impl}, public_accounts::{
        add_public_named_accounts_impl, get_all_public_named_accounts_impl, get_public_named_accounts_impl, remove_public_named_accounts_impl
    }, top_tokens::{update_icrc1_total_supply, update_price_data, update_top_holders, HolderBalanceResponse, TokenData, TopHolderData}, user_data::{
        add_new_user_impl, add_user_named_accounts_impl, add_user_oc_id_impl, add_user_tokens_impl, backup_user_named_accounts_impl, get_all_user_named_accounts_impl, get_user_data_impl, get_user_named_accounts_impl, remove_user_named_accounts_impl, update_username_impl, UserData
    }, utils::{decrypt_account, encrypt_account}
};


#[query] 
fn decrypt (input: String) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text())
    });
    let ret = decrypt_account(&input);
    return ret;
}


#[query]
fn encrypt (input: String) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text())
    });
    let ret = encrypt_account(&input);
    return ret;
}

#[query]
fn get_user_data(user_account: String) -> Option<UserData> {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text())
    });
    let ac = decrypt_account(&user_account);
    get_user_data_impl(ac)
}

// // Get user named accounts
#[query]
fn get_user_named_accounts(owner_account: String, query_vec: Vec<String>) -> Option<Vec<(String, String)>> {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text())
    });
    let oa = decrypt_account(&owner_account);
    get_user_named_accounts_impl(oa, query_vec)
}

// backup user named accounts.
#[query]
fn backup_user_named_accounts() -> Option<Vec<(String, String, String)>> {
    // check admin
    RUNTIME_STATE.with(|s| {
        s.borrow().data.check_admin(ic_cdk::caller().to_text())
    });
    backup_user_named_accounts_impl()
}

// Get ALL user named accounts
#[query]
fn get_all_user_named_accounts(owner_account: String) -> Option<Vec<(String, String)>> {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text())
    });
    let oa = decrypt_account(&owner_account);
    get_all_user_named_accounts_impl(oa)
}

// add user named account
#[update]
fn add_user_named_accounts(owner_account: String, save_account: String, save_name:String) -> String {
    // check admin
    RUNTIME_STATE.with(|state| {
        state.borrow().data.check_admin(ic_cdk::caller().to_text())
    });
    let oa = decrypt_account(&owner_account);
    add_user_named_accounts_impl(oa, save_account, save_name)
}

// delete user named account
#[update]
fn remove_user_named_account(owner_account: String, save_account: String) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    let oa = decrypt_account(&owner_account);
    remove_user_named_accounts_impl(oa, save_account)
}

// get public named accounts
#[query]
fn get_public_named_accounts(input_vec: Vec<String>) -> Option<Vec<(String, String)>> {
    RUNTIME_STATE.with(|s| {
        s.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    get_public_named_accounts_impl(input_vec)
}

// add public named account
#[update]
fn add_public_named_accounts(save_account: String, save_name:String) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
    add_public_named_accounts_impl(save_account, save_name)
}

// delete public named account
#[update]
fn remove_public_named_account(save_account: String) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_admin(ic_cdk::caller().to_text());
    });
   remove_public_named_accounts_impl(save_account)
}

// get ALL public named accounts (For Management Canister)
#[query]
fn get_all_public_named_accounts() -> Option<Vec<(String, String)>> {
    RUNTIME_STATE.with(|s| {
        s.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    get_all_public_named_accounts_impl()
}

#[update]
fn add_new_user(user_account: String) -> String {
    RUNTIME_STATE.with(|s| {
        s.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    add_new_user_impl(user_account)
}

#[update]
fn update_username(user_account: String, user_name: String) -> String {
    RUNTIME_STATE.with(|s| {
        s.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    let ua = decrypt_account(&user_account);
    update_username_impl(ua, user_name)
}

#[update]
fn add_user_tokens(user_account: String, user_tokens: u32) -> String {
    RUNTIME_STATE.with(|s| {
        s.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    let ua = decrypt_account(&user_account);
    add_user_tokens_impl(ua, user_tokens)
}

#[update]
fn set_user_oc_id(user_account: String, oc_id: String) -> String {
    RUNTIME_STATE.with(|s| {
        s.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    let ua = decrypt_account(&user_account);
    add_user_oc_id_impl(ua, oc_id)
}

// Account Identifier tools 
#[query]
fn get_single_account(input_principal: String, input_subaccount: u32) -> String {
    RUNTIME_STATE.with(|s| {
        s.borrow().data.check_authorised(ic_cdk::caller().to_text());
    });
    get_single_account_impl(input_principal, input_subaccount as u8)
}

#[query]
fn get_multiple_account(input_principal: String, start: u32, get_number: u32) -> Vec<String> {
    RUNTIME_STATE.with(|s| {
        s.borrow().data.check_authorised(ic_cdk::caller().to_text());
    });
    get_multiple_account_impl(input_principal, start as u8, get_number as u8)
}

// [][] -- METHODS FOR TOP TOKEN DATA -- [][]
#[update]
fn add_token_to_processing_list(cross: String, ledger: String, decimals: u8, stats221: String) -> String {
    RUNTIME_STATE.with(|s| {
        s.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|s|{s.borrow_mut().data.add_token(cross, ledger, decimals, stats221)})
}

#[update]
fn remove_token_from_processing_list(cross: String) -> String {
    RUNTIME_STATE.with(|s| {
        s.borrow().data.check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|s|{s.borrow_mut().data.remove_token(cross)})
}

#[query]
fn get_top_tokens_data() -> Vec<TokenData> {
    RUNTIME_STATE.with(|s| {
        s.borrow().data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|s|{s.borrow().data.get_all_top_token_data()})
}

#[query]
fn get_top_holders(cross: String) -> Option<TopHolderData> {
    RUNTIME_STATE.with(|s| {
        s.borrow().data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|s|{s.borrow().data.get_top_holders(cross)})
}