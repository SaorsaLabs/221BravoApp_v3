mod core;
mod store;
mod timers;
mod test_data;

#[cfg(test)]
mod tests {
    use oracle_shared_mk2::shared_types::{ExchangeSnapshot, OverviewV1};

    use crate::{
        core::stable_memory::STABLE_STATE, 
        test_data::{init_test_state, init_test_state_with_crosses, M1_AS_NANOS}
    };

    #[test]
    fn populate_store_and_processing_bucket(){
        // init state
        init_test_state();
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
        // add crosses to store 
        match (cross_ref, cross_ref2) {
            (Some(v1), Some(v2)) => {
                STABLE_STATE.with(|s|{ 
                    s.borrow_mut().as_mut().unwrap().add_cross_to_store(cross.clone(), v1, 1234);
                    s.borrow_mut().as_mut().unwrap().add_cross_to_store(cross2.clone(), v2, 1234);
                })
            },
            (_, _) => {
                panic!("Error adding to directory - returned None");
            }
        }

        // check processing bucket
        STABLE_STATE.with(|s|{
            let pb = s.borrow().as_ref().unwrap()
                .clone_processing_bucket();
            assert_eq!(pb[0].cross.to_string().unwrap(), cross);
            assert_eq!(pb[1].cross.to_string().unwrap(), cross2);
        });
        // check store
        STABLE_STATE.with(|s|{
            let binding = s.borrow();
            let st1 = binding.as_ref().unwrap()
            .price_data_tree.price_data.get(&cross_ref.unwrap()).unwrap();
            
            let st2 = binding.as_ref().unwrap()
            .price_data_tree.price_data.get(&cross_ref2.unwrap()).unwrap();

            assert_eq!(st1.get_cross(), cross);
            assert_eq!(st2.get_cross(), cross2);
        });
    }

    #[test]
    fn test_fill_store(){
        init_test_state_with_crosses();
        let mut av_price = 0.0;
        let mut time = 0;

        // Simulate new quotes
        // 525,600 snapshots per year (based on 60 sec intervals)
        for i in 0..525605 { // +5 snapshots to close the forming bar.
            let ex_snap:Vec<ExchangeSnapshot> = Vec::new();
            let price_oracle: Vec<OverviewV1> = Vec::from([
                OverviewV1{
                    token_cross: "TEST/ICP".to_string(),
                    snapshot_time: time,
                    average_price: av_price,
                    exchange_snapshots: ex_snap.clone(),
                    cross_to_usd: 0.0,
                },
                OverviewV1{
                    token_cross: "TEST2/ICP".to_string(),
                    snapshot_time: 0,
                    average_price: av_price,
                    exchange_snapshots: ex_snap,
                    cross_to_usd: 0.0,
                }
            ]);
            
            STABLE_STATE.with(|s|{
                s.borrow_mut().as_mut().unwrap()
                .update_prices_in_processing_bucket(price_oracle)
            });

            av_price += 0.01;
            time += M1_AS_NANOS;
        }// i

        // Check
        STABLE_STATE.with(|s|{
            let binding = s.borrow();
            let data = binding.as_ref().unwrap()
            .price_data_tree.price_data.get(&0).unwrap();

            assert_eq!(data.count_m5_bars(),  8640); // max bars
            assert_eq!(data.count_m15_bars(), 5760); // max bars
            assert_eq!(data.count_h1_bars(),  2880); // max bars
            assert_eq!(data.count_d1_bars(),  365);
            assert_eq!(data.count_w1_bars(),  52);
        });
    }
}