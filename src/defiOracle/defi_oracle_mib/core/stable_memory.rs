use std::cell::RefCell;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl};

thread_local! {
    // Stable memory manager
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}
type Memory = VirtualMemory<DefaultMemoryImpl>;

// memory for pre/ post upgrades (writing runtime to stable memory and back)
const UPGRADES: MemoryId = MemoryId::new(0);
pub fn get_upgrades_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(UPGRADES))
}