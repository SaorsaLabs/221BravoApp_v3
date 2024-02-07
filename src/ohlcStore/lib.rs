mod core;
mod store;
mod timers;



#[cfg(test)]
mod tests {

use std::mem;
use defi_oracle_shared::shared_types::OverviewV1;
use crate::{
    store::{
        price_data::{OHLC, PriceData, init_new_price_data, PriceTuple}, 
        btree::add_cross_to_store, constants::{M5_MAX, M15_MAX, H1_MAX, D1_MAX, W1_MAX}
    }, 
    core::{stable_memory::MAP, runtime::RUNTIME_STATE}, 
    timers::{processing::{add_cross_to_processing_bucket, update_prices_in_processing_bucket, remove_cross_from_processing_bucket}, constants::M5_AS_NANOS}
    };

    #[test]
    fn ohlc_size() {
        assert_eq!(56, mem::size_of::<OHLC>());
    }

    #[test]
    fn pricedata_size() {
        // struct PriceData {
        //     cross: String, 
        //     active_from: u64,
        //     last_update: u64,
        //     m5: Vec<OHLC>,  // 8640 bars 30 days
        //     m15: Vec<OHLC>, // 2880 bars 30 days
        //     h1: Vec<OHLC>,  // 1440 bars 60 days
        //     d1: Vec<OHLC>,  // 1780 bars 5 years
        //     w1: Vec<OHLC>   // 520 bars 10 years
        // } 

        let bytes_88: [u64; 11] = [0; 11]; // 88 bytes = size of OHLC struct;
        let init_vec: Vec<[i32; 14]>  = Vec::new();
        let init_size = std::mem::size_of_val(&init_vec); // size of empty vec
        let m5_ams = std::mem::size_of_val(&bytes_88) * 8640; // allocated memory size 
        let m15_ams = std::mem::size_of_val(&bytes_88) * 2880;
        let h1_ams = std::mem::size_of_val(&bytes_88) * 1440;
        let d1_ams = std::mem::size_of_val(&bytes_88) * 1780;
        let w1_ams = std::mem::size_of_val(&bytes_88) * 520;

        // totals
        let m5_total = init_size + m5_ams;
        let m15_total = init_size + m15_ams;
        let h1_total = init_size + h1_ams;
        let d1_total = init_size + d1_ams;
        let w1_total = init_size + w1_ams;

        let init_string = "".to_string();
        let total_string = std::mem::size_of_val(&init_string) + (4 * 15); // based on 15 characters and UTF-8 encoding. Total is 84 bytes 

        let total_size = 
        total_string +  // cross
        8 +             // active_from
        8 +             // last_update
        m5_total +      // m5
        m15_total +     // m15
        h1_total +      // h1
        d1_total +      // d1
        w1_total;       // w1

        println!("total size bytes : {}", total_size);
        assert_eq!(total_size, 1_343_100);

    }
    
    // test fill of vecs with max size + 5
    #[test]
    fn pricedata_overflow() {
        // init cross
        add_cross_to_store("TEST/ICP".to_string());

        let op = PriceTuple{ cross_price: 50.0, usd_price: 50.0 };
        let cl = PriceTuple{ cross_price: 50.0, usd_price: 50.0 };
        let hi = PriceTuple{ cross_price: 100.0, usd_price: 100.0 };
        let lo = PriceTuple{ cross_price: 20.0, usd_price: 20.0 };

        let data = 
        OHLC::new(0, u64::MAX, op, hi, lo, cl, 0);
        
        // populate vecs
        MAP.with(|s| {
            let x = s.borrow().get(&"TEST/ICP".to_string());
            match x {
                Some(v) => {
                    let mut pd: PriceData = v;
    
                    // m5
                    for _i in 0..M5_MAX +5 {
                        pd.push_m5(data.clone());
                    }
    
                    // m15
                    for _i in 0..M15_MAX +5 {
                        pd.push_m15(data.clone());
                    }
    
                    // h1
                    for _i in 0..H1_MAX +5 {
                        pd.push_h1(data.clone());
                    }
    
                    // d1
                    for _i in 0..D1_MAX +5 {
                        pd.push_d1(data.clone());
                    }
    
                    // w1
                    for _i in 0..W1_MAX +5 {
                        pd.push_w1(data.clone());
                    }
    
                    s.borrow_mut().insert("TEST/ICP".to_string(), pd);

                },
                None => {}
            }
        });
        
        let data = MAP.with(|s|{
            s.borrow().get(&"TEST/ICP".to_string()).unwrap()});

        assert_eq!(data.m5.len(), M5_MAX);
        assert_eq!(data.m15.len(), M15_MAX);
        assert_eq!(data.h1.len(), H1_MAX);
        assert_eq!(data.d1.len(), D1_MAX);
        assert_eq!(data.w1.len(), W1_MAX);

    }

    // Fetch data from stable btree/ store
    #[test]
    fn fetch_ohlc_data() {
        // NOTE - need to make OHLC open_time struc pub to allow all tests to run. 

        // init cross
        add_cross_to_store("TEST/ICP".to_string());
        let op = PriceTuple{ cross_price: 50.0, usd_price: 50.0 };
        let cl = PriceTuple{ cross_price: 50.0, usd_price: 50.0 };
        let hi = PriceTuple{ cross_price: 100.0, usd_price: 100.0 };
        let lo = PriceTuple{ cross_price: 20.0, usd_price: 20.0 };

        let data = 
        OHLC::new(0, u64::MAX, op, hi, lo, cl, 0);
        
        // populate vecs
        MAP.with(|s| {
            let x = s.borrow().get(&"TEST/ICP".to_string());
            match x {
                Some(v) => {
                    let mut pd: PriceData = v;
                    // m5
                    for i in 0..5 {
                        let op = PriceTuple{ cross_price: 50.0, usd_price: 50.0 };
                        let cl = PriceTuple{ cross_price: 50.0, usd_price: 50.0 };
                        let hi = PriceTuple{ cross_price: 100.0, usd_price: 100.0 };
                        let lo = PriceTuple{ cross_price: 20.0, usd_price: 20.0 };
                
                        let data = 
                        OHLC::new(0, u64::MAX, op, hi, lo, cl, 0);
                        pd.push_m5(data.clone());
                    }
                    s.borrow_mut().insert("TEST/ICP".to_string(), pd);
                },
                None => {}
            }
        });

        let full_vec = MAP.with(|s| {
            s.borrow().get(&"TEST/ICP".to_string()).unwrap()
        });
        let v1 = full_vec.get_all_m5();
        let v2 = full_vec.get_m5(3);

        // get all 
        assert_eq!(v1.len(), 5);
        // get last 3
        assert_eq!(v2.len(), 3);
        // Need to make open_time public to allow these tests
        // assert_eq!(v2[0].open_time, 2); 
        // assert_eq!(v2[1].open_time, 3);
        // assert_eq!(v2[2].open_time, 4);
    }

    // adds token to processing bucket, gives it 5 'ticks' of new information
    // on the last tick a new bar is formed which is pushed into the btreemap
    // on the last tick the processing bucket is reset ready for the new bar ticks
    // then removes token from processing bucket.. just to test this feature.
    #[test]
    fn add_update_remove_processing_bucket() {
        // ADD NEW CROSS
        let cross = "TEST/ICP".to_string();
        let pd = init_new_price_data(cross.clone());
        // add to btree map/ store
        MAP.with(|s|{
            s.borrow_mut().insert(cross.clone(), pd)
        });
        // add to processing array
        let next_midnight:u64 = 1672531200000000000; // 01/01/23 00:00 in nanos. 
        add_cross_to_processing_bucket(cross, next_midnight);

        // UPDATE 1 - first quote
        let update1 = OverviewV1{
            token_cross: "TEST/ICP".to_string(),
            snapshot_time: 1672531260000000000, // 01/01/23 00:01 in nanos.
            average_price: 10.0,
            exchange_snapshots: Vec::new(), // not needed
            cross_to_usd: 1.0
        };
        let update1_vec = vec![update1];
        update_prices_in_processing_bucket(update1_vec);

        // check update 
        let buckets = RUNTIME_STATE.with(|s|{
            s.borrow().processing.clone()
        });

        // first quote everything is 10.0
        assert_eq!(buckets[0].m5.open.cross_price, 10.0);
        assert_eq!(buckets[0].m5.high.cross_price, 10.0);
        assert_eq!(buckets[0].m5.low.cross_price, 10.0);
        assert_eq!(buckets[0].m5.close.cross_price, 10.0);

        // UPDATE 2 - new high
        let update2 = OverviewV1{
            token_cross: "TEST/ICP".to_string(),
            snapshot_time: 1672531320000000000, // 01/01/23 00:02 in nanos.
            average_price: 15.123,
            exchange_snapshots: Vec::new(), // not needed
            cross_to_usd: 1.0
        };
        let update2_vec = vec![update2];
        update_prices_in_processing_bucket(update2_vec);
        
        // check update 
        let buckets2 = RUNTIME_STATE.with(|s|{
            s.borrow().processing.clone()
        });

        // new high - close updates as usual & high value updates. 
        assert_eq!(buckets2[0].m5.open.cross_price, 10.0);
        assert_eq!(buckets2[0].m5.high.cross_price, 15.123);
        assert_eq!(buckets2[0].m5.low.cross_price, 10.0);
        assert_eq!(buckets2[0].m5.close.cross_price, 15.123);
        
        // UPDATE 3 - new low
        let update3 = OverviewV1{
            token_cross: "TEST/ICP".to_string(),
            snapshot_time: 1672531380000000000, // 01/01/23 00:03 in nanos.
            average_price: 8.898,
            exchange_snapshots: Vec::new(), // not needed
            cross_to_usd: 1.0
        };
        let update3_vec = vec![update3];
        update_prices_in_processing_bucket(update3_vec);

        // check update 
        let buckets3 = RUNTIME_STATE.with(|s|{
            s.borrow().processing.clone()
        });

        // new low - close updates as usual & low value updates. 
        assert_eq!(buckets3[0].m5.open.cross_price, 10.0);
        assert_eq!(buckets3[0].m5.high.cross_price, 15.123);
        assert_eq!(buckets3[0].m5.low.cross_price, 8.898);
        assert_eq!(buckets3[0].m5.close.cross_price, 8.898);

        // UPDATE 4 - no new high or low 
        let update4 = OverviewV1{
            token_cross: "TEST/ICP".to_string(),
            snapshot_time: 1672531440000000000, // 01/01/23 00:04 in nanos.
            average_price: 12.12,
            exchange_snapshots: Vec::new(), // not needed
            cross_to_usd: 1.0
        };
        let update4_vec = vec![update4];
        update_prices_in_processing_bucket(update4_vec);

        // check update 
        let buckets4 = RUNTIME_STATE.with(|s|{
            s.borrow().processing.clone()
        });

        // no new high/ low -  only close updates
        assert_eq!(buckets4[0].m5.open.cross_price, 10.0);
        assert_eq!(buckets4[0].m5.high.cross_price, 15.123);
        assert_eq!(buckets4[0].m5.low.cross_price, 8.898);
        assert_eq!(buckets4[0].m5.close.cross_price, 12.12);
        
        // UPDATE 5 - first tick of new bar 
        let update5 = OverviewV1{
            token_cross: "TEST/ICP".to_string(),
            snapshot_time: 1672531500000000001, // 01/01/23 00:05 and 1 nano.
            average_price: 12.12,
            exchange_snapshots: Vec::new(), // not needed
            cross_to_usd: 1.0
        };
        let update5_vec = vec![update5];
        update_prices_in_processing_bucket(update5_vec);

        // check update 
        let buckets5 = RUNTIME_STATE.with(|s|{
            s.borrow().processing.clone()
        });

        // bucket 5 is a new bucket as it's a new bar
        assert_eq!(buckets5[0].m5.open_time, 1672531500000000000); // open = close of previous bar
        assert_eq!(buckets5[0].m5.close_time, 1672531500000000000+M5_AS_NANOS); // close = open + M5 in nanos
        //new bar all values init as 12.12
        assert_eq!(buckets5[0].m5.open.cross_price, 12.12);
        assert_eq!(buckets5[0].m5.high.cross_price, 12.12);
        assert_eq!(buckets5[0].m5.low.cross_price, 12.12);
        assert_eq!(buckets5[0].m5.close.cross_price, 12.12);

        // check if bar was pushed into MAP and values match all updates correctly.
        let data = MAP.with(|s|{
            s.borrow().get(&"TEST/ICP".to_string()).unwrap()
        });
        assert_eq!(data.m5[0].open_time, 1672531200000000000);
        assert_eq!(data.m5[0].close_time, 1672531500000000000);
        assert_eq!(data.m5[0].open.cross_price, 10.0);
        assert_eq!(data.m5[0].high.cross_price, 15.123);
        assert_eq!(data.m5[0].low.cross_price, 8.898);
        assert_eq!(data.m5[0].close.cross_price, 12.12);
        
        // remove cross from bucket
        remove_cross_from_processing_bucket("TEST/ICP".to_string());

        //check removed
        let size = RUNTIME_STATE.with(|s|{
            s.borrow().processing.clone()
        });
        assert_eq!(size.len(), 0); 
    }


}