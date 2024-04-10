use ic_stable_memory::stable_memory_init;
use oracle_shared_mk2::shared_types::{ExchangeSnapshot, OverviewV1};
use crate::core::stable_memory::{Main, STABLE_STATE};

pub fn init_test_state(){
    stable_memory_init();
    let stable_data = Main::default();
    STABLE_STATE.with(|state| {
        *state.borrow_mut() = Some(stable_data);
    });
}

pub fn test_oracle_data(input: u8) -> Vec<OverviewV1> {
    let mut ret: Vec<OverviewV1> = Vec::new();
    let ex: Vec<ExchangeSnapshot> = Vec::new();
    let p1: f64;
    let p2: f64;

    
    if input > 1 {
        p1 = input as f64;
        p2 = input as f64 - 1.0;
    } else {
        p1 = 1.0;
        p2 = 0.0;
    }
    
    let token1: OverviewV1 = OverviewV1{
        token_cross: String::from("test/icp"),
        snapshot_time: 0,
        average_price: p1,
        exchange_snapshots: ex.clone(),
        cross_to_usd: 0.0,
    };

    let token2: OverviewV1 = OverviewV1{
        token_cross: String::from("potato/icp"),
        snapshot_time: 0,
        average_price: p2,
        exchange_snapshots: ex,
        cross_to_usd: 0.0,
    };

    ret.push(token1);
    ret.push(token2);
    ret
}