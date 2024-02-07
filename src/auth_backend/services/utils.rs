use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use candid::Principal;
use super::account_identifier::{ AccountIdentifier, Subaccount };
use super::constants::PHRASE;

pub fn get_subaccount_from_principal(principal_id: String, subaccount: u8) -> String {
    let pncpl = Principal::from_text(principal_id).expect("Could not decode the principal.");
    let mut sub = [0; 32];
    sub[31] = subaccount;
    let sub_ac: Subaccount = Subaccount(sub);
    let sub_account = AccountIdentifier::new(pncpl, Some(sub_ac));
    return AccountIdentifier::to_hex(&sub_account);
}

pub fn get_multiple_subaccounts_from_principal(
    principal_id: String,
    start: u8,
    end: u8
) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    let mut pncpl;
    let mut sub = [0; 32];
    let mut sub_ac: Subaccount;
    let mut sub_account;
    for x in start..=end {
        pncpl = Principal::from_text(principal_id.clone()).expect(
            "Could not decode the principal."
        );
        sub[31] = x;
        sub_ac = Subaccount(sub);
        sub_account = AccountIdentifier::new(pncpl, Some(sub_ac));
        output.push(AccountIdentifier::to_hex(&sub_account));
    }
    return output;
}

// decrypt account
pub fn decrypt_account(input: &String) -> String {
    let passphrase = PHRASE; 
    let mc = new_magic_crypt!(passphrase, 256);
        mc.decrypt_base64_to_string(input).unwrap()
}

// encrypt account
pub fn encrypt_account(input: &String) -> String {
    let passphrase = PHRASE;
    let mc = new_magic_crypt!(passphrase, 256);
        mc.encrypt_str_to_base64(input)
}