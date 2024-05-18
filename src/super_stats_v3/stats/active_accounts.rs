use candid::CandidType;
use ic_stable_memory::{collections::{SHashMap, SVec}, derive::{AsFixedSizeBytes, StableType}, StableType};

use crate::core::{constants::D1_AS_NANOS, stable_memory::STABLE_STATE, types::IDKey};

// Struct and impl for simple activity stats - Count of active accounts over time, total unique accounts. 
#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct ActivityStats {
    // store for historic snapshots
    daily_snapshots: SVec<ActivitySnapshot>,
    // working data for latest snapshot 'window'
    pub chunk_start_time: u64,
    pub chunk_end_time: u64,
    chunk_window_size: u64,
    count_during_current_snapshot: u64
}

impl ActivityStats {
    pub fn add_to_current_snapshot(&mut self, count: u64){
        self.count_during_current_snapshot += count;
    }

    pub fn set_chunk_window_size(&mut self, window_size: u64){
        self.chunk_window_size = window_size;   
    }

    pub fn take_activity_snapshot(&mut self, total_accounts: u64) -> (u64, u64) {
        let ret = ActivitySnapshot{
            start_time: self.chunk_start_time.clone(),
            end_time: self.chunk_end_time.clone(),
            total_unique_accounts: total_accounts,
            active_during_snapshot: self.count_during_current_snapshot.clone(),
        };
        self.daily_snapshots.push(ret);
        self.chunk_start_time = self.chunk_end_time;
        self.chunk_end_time = self.chunk_start_time + self.chunk_window_size;
        self.count_during_current_snapshot = 0;

        return (self.chunk_start_time.clone(), self.chunk_end_time.clone());
    }
    
    // padding when no transactions are made within 24 hours
    pub fn add_empty_snapshot(&mut self, start_time: u64, end_time: u64, total_accounts: u64) -> (u64, u64) {
        let ret = ActivitySnapshot{
            start_time,
            end_time: end_time.clone(),
            total_unique_accounts: total_accounts,
            active_during_snapshot: 0,
        };
        self.daily_snapshots.push(ret);
        self.chunk_start_time = end_time.clone();
        self.chunk_end_time = end_time + self.chunk_window_size;
        self.count_during_current_snapshot = 0;

        return (self.chunk_start_time.clone(), self.chunk_end_time.clone());
    }

    pub fn init(&mut self, start: u64, end: u64){
        self.chunk_start_time = start;
        self.chunk_end_time = end;
        self.chunk_window_size = D1_AS_NANOS;
    }

    pub fn get_daily_snapshots(&self, mut ret_len: usize) -> Vec<ActivitySnapshot> {
        let mut ret: Vec<ActivitySnapshot> = Vec::new();
        let len = self.daily_snapshots.len();
        if ret_len > len { ret_len = len+1 };
        let start = (len) - (ret_len as usize -1); // -1 as current snapshot is then added
        for i in start..len{
            let sh = self.daily_snapshots.get(i).unwrap();
            ret.push(ActivitySnapshot{
                start_time: sh.start_time.clone(),
                end_time: sh.end_time.clone(),
                total_unique_accounts: sh.total_unique_accounts.clone(),
                active_during_snapshot: sh.active_during_snapshot.clone(),
            });
        }
        // add current window
        let last_total = self.daily_snapshots.get(len-1).unwrap().clone();
        ret.push(ActivitySnapshot{
            start_time: self.chunk_start_time,
            end_time: self.chunk_end_time,
            total_unique_accounts: last_total.total_unique_accounts,
            active_during_snapshot: self.count_during_current_snapshot,
        });
        ret
    }
}

#[derive(StableType, AsFixedSizeBytes, Debug, Default, Clone, CandidType)]
pub struct ActivitySnapshot {
    pub start_time: u64,
    pub end_time: u64,
    pub total_unique_accounts: u64,
    pub active_during_snapshot: u64
}

pub fn get_count_of_unique_accounts(stx_ref_vec: Vec<Option<u64>>) -> u64 {
    if stx_ref_vec.len() == 0 { return 0_u64 }
    let mut u64_values: Vec<u64> = Vec::new();
    // Remove none values and get u64
    for stx_ref in stx_ref_vec {
        match stx_ref {
            None => {},
            Some(v) => {u64_values.push(v)}
        }
    }
    // sort and get unique values
    u64_values.sort();
    let mut count:u64 = 1; // first index is counted as loop starts at 1 not 0
    for i in 1..u64_values.len() {
        if u64_values[i] != u64_values[i-1] { count += 1}
    }
    count
}

pub fn push_activity_snapshot() -> (u64, u64) {
    let count =     STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().directory_data.get_total_entries()
    });
    let new_times = STABLE_STATE.with(|s|{
        s.borrow_mut().as_mut().unwrap().activity_stats.take_activity_snapshot(count)
    });
    new_times
}

pub fn push_padding_snapshot(start: u64, end: u64) -> (u64, u64) {
    let count =     STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().directory_data.get_total_entries()
    });
    let new_times = STABLE_STATE.with(|s|{
        s.borrow_mut().as_mut().unwrap().activity_stats.take_activity_snapshot(count)
    });
    new_times
}

pub fn nearest_past_day_start(time_nano: u64) -> u64 {
    let remainder = time_nano % D1_AS_NANOS;
    let nearest_day_start = time_nano - remainder;
    return nearest_day_start;
}

pub fn next_midnight_time(time_now: u64) -> u64 {
    let last_midnight: u64 = nearest_past_day_start(time_now);
    let next_midnight: u64 = last_midnight + D1_AS_NANOS;
    return next_midnight;
}

pub fn init_activity_stats(first_block_time: u64) -> u64 {
    let end_time = next_midnight_time(first_block_time.clone());
    STABLE_STATE.with(|s|{
        s.borrow_mut().as_mut().unwrap().activity_stats.init(first_block_time, end_time.clone())
    });
    end_time
}