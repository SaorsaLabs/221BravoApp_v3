use candid::CandidType;
use defi_oracle_shared::shared_types::Marketplace;
use serde::{Serialize, Deserialize};

// [][] --- Types for Utils --- [][]
#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct MemoryData {
   pub memory: u64,
   pub heap_memory: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub text: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct WorkingStats {
    pub last_update_time: u64, 
    pub assigned_marketplace: Marketplace,
    pub assigned_crosses: Vec<String>,
}
