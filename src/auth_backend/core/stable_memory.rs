use std::cell::RefCell;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, BTreeMap};
use crate::services::user_data::UserData;

thread_local! {
    // Stable memory manager
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    // 1 - Stable BtreeMap (User Account Store)
    pub static USER_MAP: RefCell<BTreeMap<String, UserData, Memory>> = RefCell::new(
        BTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), // upgrades uses 0
        )
    );
    // 2 - Stable BtreeMap (Public Accounts)
    pub static PUBLIC_MAP: RefCell<BTreeMap<String, String, Memory>> = RefCell::new(
        BTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))), 
        )
    );
}
type Memory = VirtualMemory<DefaultMemoryImpl>;

// memory for pre/ post upgrades (writing runtime to stable memory and back)
const UPGRADES: MemoryId = MemoryId::new(0);
pub fn get_upgrades_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(UPGRADES))
}