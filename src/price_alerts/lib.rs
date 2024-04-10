mod core;
mod price_alerts;
mod test_data;
mod timers;


#[cfg(test)]
mod tests {
    use crate::{
        core::{runtime::RUNTIME_STATE, stable_memory::STABLE_STATE, types::IDKey}, 
        price_alerts::btree::{InputAlert}, test_data::init_test_state, 
        timers::timers::fetch_and_update_prices_test
    };

    #[test]
    fn populate_and_remove(){
        init_test_state();
        let test_alert = InputAlert{
            id: 0,
            user: "testid".to_string(),
            cross: "TEST/ICP".to_string(),
            oc_id: "XXXX-XXXX-XXXX-XXXX-XXX".to_string(),
            price: 101.0,
            direction: 1
        };
        // add data
        STABLE_STATE.with(|s|{
            let mut ptr = s.borrow_mut();
            let state = ptr.as_mut().unwrap();
            let _ = state.alert_data.add_alert(test_alert.clone());
        });
        // check cross
        let cross_vec = STABLE_STATE.with(|s|{
            let ptr = s.borrow();
            let state = ptr.as_ref().unwrap();
            let idk = IDKey::from_string(test_alert.cross.clone()).unwrap();
            state.alert_data.get_all_alerts_by_cross(idk)
        });
        if let Some(v) = cross_vec {
            assert_eq!(v[0].id, 0)   
        } else {
            panic!("Cross_vec returned NONE");
        }
        // check user
        let user_vec = STABLE_STATE.with(|s|{
            let ptr = s.borrow();
            let state = ptr.as_ref().unwrap();
            let idk = IDKey::from_string(test_alert.user.clone()).unwrap();
            state.alert_data.get_all_alerts_by_user(idk)
        });
        if let Some(v) = user_vec {
            assert_eq!(v[0].id, 0)   
        } else {
            panic!("user_vec returned NONE");
        }

        // REMOVE ALERTS
        STABLE_STATE.with(|s|{
            let mut ptr = s.borrow_mut();
            let state = ptr.as_mut().unwrap();
            state.alert_data.remove_alert(test_alert.clone())
        });
        // check cross/ user
        let cross_vec2 = STABLE_STATE.with(|s|{
            let ptr = s.borrow();
            let state = ptr.as_ref().unwrap();
            let idk = IDKey::from_string(test_alert.cross.clone()).unwrap();
            state.alert_data.get_all_alerts_by_cross(idk)
        });
        let user_vec2 = STABLE_STATE.with(|s|{
            let ptr = s.borrow();
            let state = ptr.as_ref().unwrap();
            let idk = IDKey::from_string(test_alert.user.clone()).unwrap();
            state.alert_data.get_all_alerts_by_user(idk)
        });
        assert_eq!(cross_vec2.unwrap().len(), 0);
        assert_eq!(user_vec2.unwrap().len(), 0);
    }

    // test alert levels
    #[test]
    fn test_levels(){
        init_test_state();

        // add alert - test/icp @ 1.5 long
        let input_alert = InputAlert{
            id: 0,
            user: String::from("user123"),
            cross: String::from("test/icp"),
            oc_id: String::from("ocid123"),
            price: 1.5,
            direction: 1,
        };
        STABLE_STATE.with(|s|{
            let mut ptr = s.borrow_mut();
            let state = ptr.as_mut().unwrap();
            match state.alert_data.add_alert(input_alert) {
                Ok(v) => v as i64,
                Err(_e) => -1_i64
            }
        });

        // add alert - potato/icp @ 1.8 long
        let input_alert2 = InputAlert{
            id: 0,
            user: String::from("user123"),
            cross: String::from("potato/icp"),
            oc_id: String::from("ocid123"),
            price: 1.8,
            direction: 1,
        };
        STABLE_STATE.with(|s|{
            let mut ptr = s.borrow_mut();
            let state = ptr.as_mut().unwrap();
            match state.alert_data.add_alert(input_alert2) {
                Ok(v) => v as i64,
                Err(_e) => -1_i64
            }
        });

        // add alert - test/icp @ 1.3 short
        let input_alert3 = InputAlert{
            id: 0,
            user: String::from("user234"),
            cross: String::from("test/icp"),
            oc_id: String::from("ocid123"),
            price: 1.3,
            direction: 0,
        };
        STABLE_STATE.with(|s|{
            let mut ptr = s.borrow_mut();
            let state = ptr.as_mut().unwrap();
            match state.alert_data.add_alert(input_alert3) {
                Ok(v) => v as i64,
                Err(_e) => -1_i64
            }
        });

        // add alert - potato/icp @ 1.4 short
        let input_alert3 = InputAlert{
            id: 0,
            user: String::from("user234"),
            cross: String::from("potato/icp"),
            oc_id: String::from("ocid123"),
            price: 1.4,
            direction: 0,
        };
        STABLE_STATE.with(|s|{
            let mut ptr = s.borrow_mut();
            let state = ptr.as_mut().unwrap();
            match state.alert_data.add_alert(input_alert3) {
                Ok(v) => v as i64,
                Err(_e) => -1_i64
            }
        });

        // first run - inits the previous quote 
        fetch_and_update_prices_test(0); 
        let pending_len = RUNTIME_STATE.with(|s|{
            s.borrow().data.alerts_pending.len()
        });
        let prev_prices = STABLE_STATE.with(|s|{
            let ptr = s.borrow();
            let state = ptr.as_ref().unwrap();
            state.previous_prices.len()
        });
        assert_eq!(pending_len, 0);
        assert_eq!(prev_prices, 2);

        // 2nd run - full run of fetch_and_update however price still under alert
        fetch_and_update_prices_test(1); 
        let pending_len2 = RUNTIME_STATE.with(|s|{
            s.borrow().data.alerts_pending.len()
        });
        let prev_prices2 = STABLE_STATE.with(|s|{
            let ptr = s.borrow();
            let state = ptr.as_ref().unwrap();
            state.previous_prices.len()
        });
        assert_eq!(pending_len2, 0);
        assert_eq!(prev_prices2, 2);

        // 3rd run - price crosses test/icp alert
        fetch_and_update_prices_test(2); 
        let pending_len3 = RUNTIME_STATE.with(|s|{
            s.borrow().data.alerts_pending.len()
        });
        let prev_prices3 = STABLE_STATE.with(|s|{
            let ptr = s.borrow();
            let state = ptr.as_ref().unwrap();
            state.previous_prices.len()
        });
        let alert = RUNTIME_STATE.with(|s|{
            s.borrow().data.alerts_pending.clone()
        });
        assert_eq!(pending_len3, 1);
        assert_eq!(alert[0].cross, "test/icp");
        assert_eq!(alert[0].direction, 1);
        assert_eq!(prev_prices3, 2);

        // 4th run - price crosses potato/icp alert
        fetch_and_update_prices_test(3); 
        let pending_len4 = RUNTIME_STATE.with(|s|{
            s.borrow().data.alerts_pending.len()
        });
        let prev_prices4 = STABLE_STATE.with(|s|{
            let ptr = s.borrow();
            let state = ptr.as_ref().unwrap();
            state.previous_prices.len()
        });
        let alert2 = RUNTIME_STATE.with(|s|{
            s.borrow().data.alerts_pending.clone()
        });
        assert_eq!(pending_len4, 1);
        assert_eq!(alert2[0].cross, "potato/icp");
        assert_eq!(alert2[0].direction, 1);
        assert_eq!(prev_prices4, 2);


        // 4th run - test cross under - both test/icp and potato/icp triggered. 
        fetch_and_update_prices_test(0); 
        let pending_len5 = RUNTIME_STATE.with(|s|{
            s.borrow().data.alerts_pending.len()
        });
        let prev_prices5 = STABLE_STATE.with(|s|{
            let ptr = s.borrow();
            let state = ptr.as_ref().unwrap();
            state.previous_prices.len()
        });
        let alert3 = RUNTIME_STATE.with(|s|{
            s.borrow().data.alerts_pending.clone()
        });
        assert_eq!(pending_len5, 2);
        assert_eq!(alert3[0].cross, "test/icp");
        assert_eq!(alert3[1].cross, "potato/icp");
        assert_eq!(alert3[0].direction, 0);
        assert_eq!(alert3[1].direction, 0);
        assert_eq!(prev_prices5, 2);

    }

}