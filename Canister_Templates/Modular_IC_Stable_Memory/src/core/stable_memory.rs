use std::cell::RefCell;

use ic_stable_memory::derive::{AsFixedSizeBytes, StableType};

thread_local! {
    pub static STABLE_STATE: RefCell<Option<Main>> = RefCell::default();
}

#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct Main {
    // pub canister_data: CanisterSettings,
    // pub processed_data: AccountTree,
    // pub directory_data: Directory,
}