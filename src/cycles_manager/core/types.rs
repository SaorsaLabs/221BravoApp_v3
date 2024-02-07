use candid::CandidType;
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

// used for ic-canister calls which require no args.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Empty {
}
impl Empty {
    pub fn new() -> Empty {
        return Empty{};
    }
}