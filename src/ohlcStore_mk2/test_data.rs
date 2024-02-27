use ic_stable_memory::stable_memory_init;
use crate::core::stable_memory::{Main, STABLE_STATE};

pub const M1_AS_NANOS: u64 = 60_000_000_000;

pub fn init_test_state(){
    stable_memory_init();
    let stable_data = Main::default();
    STABLE_STATE.with(|state| {
        *state.borrow_mut() = Some(stable_data);
    });
}

pub fn init_test_state_with_crosses(){
    stable_memory_init();
    let stable_data = Main::default();
    STABLE_STATE.with(|state| {
        *state.borrow_mut() = Some(stable_data);
    });
    let cross: String = "TEST/ICP".to_string();
    let cross2: String = "TEST2/ICP".to_string();
    let mut cross_ref: Option<u64> = None;
    let mut cross_ref2: Option<u64> = None;
    // add crosses to directory
    STABLE_STATE.with(|s|{
        cross_ref = s.borrow_mut().as_mut().unwrap()
            .directory_data.add_id(cross.clone());
        cross_ref2 = s.borrow_mut().as_mut().unwrap()
            .directory_data.add_id(cross2.clone());
    });
    // add crosses to store/ processing bucket 
    match (cross_ref, cross_ref2) {
        (Some(v1), Some(v2)) => {
            STABLE_STATE.with(|s|{ 
                s.borrow_mut().as_mut().unwrap().add_cross_to_store(cross.clone(), v1, 0);
                s.borrow_mut().as_mut().unwrap().add_cross_to_store(cross2.clone(), v2, 0);
            })
        },
        (_, _) => {
            panic!("Error adding to directory - returned None");
        }
    }
}