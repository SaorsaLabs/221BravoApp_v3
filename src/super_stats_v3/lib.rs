mod core;
mod stats;
mod timers;
mod test_data;


mod tests {
    use crate::{
        core::{constants::D1_AS_NANOS, runtime::RUNTIME_STATE, stable_memory::STABLE_STATE, types::IDKey}, 
        stats::{account_tree::{fill_missing_days, get_account_last_days, GetAccountBalanceHistory}, api::{ get_account_overview, get_principal_overview, get_top_account_holders, get_top_principal_holders}, constants::{DAY_AS_NANOS, HOUR_AS_NANOS}, custom_types::{IndexerType, ProcessedTX}, fetch_data::dfinity_icrc2_types::DEFAULT_SUBACCOUNT, process_data::{
            process_index::{process_smtx_to_index, process_smtx_to_principal_index}, process_time_stats::{calculate_time_stats, top_x_by_txvalue, StatsType}, 
            small_tx::{processedtx_to_principal_only_smalltx, processedtx_to_smalltx}}, utils::{nearest_day_start, nearest_past_hour, parse_icrc_account, principal_subaccount_to_string}}, 
            test_data::{self, ptx_test_data, ptx_test_data_for_history, test_state_init}
        };

    #[test]
    fn test_string_to_key(){
        let input: String = "8c79044b039ee8afbf1e8cc679d93cdfddfdf28710691aa9c81b85d7ef253206".to_string();
        let as_key: IDKey = IDKey::from_string(&input).unwrap();
        let output:String  = as_key.to_string().unwrap();
        assert_eq!(input, output);

        let input2: String = "q6osm-57cdv-5zmcc-p7dtq-v2lpi-uuzkr-pzhgf-lncpe-ns2yr-cxqsc-uqe.0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let as_key2:IDKey = IDKey::from_string(&input2).unwrap();
        let output2: String = as_key2.to_string().unwrap();
        assert_eq!(input2, output2);

        let input3: String = "q6osm".to_string();
        let as_key3:IDKey  = IDKey::from_string(&input3).unwrap();
        let output3: String = as_key3.to_string().unwrap();
        assert_eq!(input3, output3);
    }

    #[test]
    fn test_process_to_small_tx_format(){

        // init test Stable/ Runtime state
        test_state_init();

        let ptx = ptx_test_data();
        let stx = processedtx_to_smalltx(&ptx);

        // TRANSACTION TYPE
        // Processed TX 10
        let first_ptx = ptx[10].clone();
        // Small TX 10
        let first_stx = stx[10].clone();

        // from account to u32 ref (using Directory)
        let id_ref_from = STABLE_STATE.with(|s|{
            s.borrow().as_ref().unwrap().directory_data.get_ref(&first_ptx.from_account)
        });

        // to account to u32 ref (using Directory)
        let id_ref_to = STABLE_STATE.with(|s|{
            s.borrow().as_ref().unwrap().directory_data.get_ref(&first_ptx.to_account)
        });
        
        // check from ac on Small TX = from ac on Processed TX
        assert_eq!(first_stx.from, id_ref_from);
        // check to ac on Small TX = to ac on Processed TX
        assert_eq!(first_stx.to, id_ref_to);
        // check time
        assert_eq!(first_stx.time, first_ptx.tx_time);
        // check type
        assert_eq!(first_stx.tx_type, 0_u8); // 0 = transfer, 1 = Mint, 2 = Burn. 3 = approve.
        // check value
        assert_eq!(first_stx.value, first_ptx.tx_value);
        // check block
        assert_eq!(first_stx.block, first_ptx.block);

        // MINT TYPE - TX0
        let mint_ptx = ptx[0].clone();
        // Small TX 10
        let mint_stx = stx[0].clone();

        // from account to u32 ref (using Directory)
        let mint_ref_from = STABLE_STATE.with(|s|{
            s.borrow().as_ref().unwrap().directory_data.get_ref(&mint_ptx.from_account)
        });

        // to account to u32 ref (using Directory)
        let mint_ref_to = STABLE_STATE.with(|s|{
            s.borrow().as_ref().unwrap().directory_data.get_ref(&mint_ptx.to_account)
        });
        
        // check from ac on Small TX = from ac on Processed TX
        assert_eq!(mint_stx.from, mint_ref_from);
        // check to ac on Small TX = to ac on Processed TX
        assert_eq!(mint_stx.to, mint_ref_to);
        // check time
        assert_eq!(mint_stx.time, mint_ptx.tx_time);
        // check type
        assert_eq!(mint_stx.tx_type, 1_u8); // 0 = transfer, 1 = Mint, 2 = Burn. 3 = approve.
        // check value
        assert_eq!(mint_stx.value, mint_ptx.tx_value);
        // check block
        assert_eq!(mint_stx.block, mint_ptx.block);

        // // BURN TYPE
        let burn_ptx = ptx[16].clone();
        // Small TX 10
        let burn_stx = stx[16].clone();

        // from account to u32 ref (using Directory)
        let burn_ref_from = STABLE_STATE.with(|s|{
            s.borrow().as_ref().unwrap().directory_data.get_ref(&burn_ptx.from_account)
        });

        // to account to u32 ref (using Directory)
        let burn_ref_to = STABLE_STATE.with(|s|{
            s.borrow().as_ref().unwrap().directory_data.get_ref(&burn_ptx.to_account)
        });
        
        // check from ac on Small TX = from ac on Processed TX
        assert_eq!(burn_stx.from, burn_ref_from);
        // check to ac on Small TX = to ac on Processed TX
        assert_eq!(burn_stx.to, burn_ref_to);
        // check time
        assert_eq!(burn_stx.time, burn_ptx.tx_time);
        // check type
        assert_eq!(burn_stx.tx_type, 2_u8); // 0 = transfer, 1 = Mint, 2 = Burn. 3 = approve.
        // check value
        assert_eq!(burn_stx.value, burn_ptx.tx_value);
        // check block
        assert_eq!(burn_stx.block, burn_ptx.block);

        // // APPROVE TYPE
        let ap_ptx = ptx[30].clone();
        // Small TX 10
        let ap_stx = stx[30].clone();

        // from account to u32 ref (using Directory)
        let ap_ref_from = STABLE_STATE.with(|s|{
            s.borrow().as_ref().unwrap().directory_data.get_ref(&ap_ptx.from_account)
        });

        // to account to u32 ref (using Directory)
        let ap_ref_to = STABLE_STATE.with(|s|{
            s.borrow().as_ref().unwrap().directory_data.get_ref(&ap_ptx.to_account)
        });
        
        // check from ac on Small TX = from ac on Processed TX
        assert_eq!(ap_stx.from, ap_ref_from);
        // check to ac on Small TX = to ac on Processed TX
        assert_eq!(ap_stx.to, ap_ref_to);
        // check time
        assert_eq!(ap_stx.time, ap_ptx.tx_time);
        // check type
        assert_eq!(ap_stx.tx_type, 3_u8); // 0 = transfer, 1 = Mint, 2 = Burn. 3 = approve.
        // check value
        assert_eq!(ap_stx.value, ap_ptx.tx_value);
        // check block
        assert_eq!(ap_stx.block, ap_ptx.block);

        // check input length == output length.
        assert_eq!(ptx.len(), stx.len()); 
    }

    #[test]
    fn test_nearest_past_hour() {
        //  Input already on an hour boundary
        let time_ms_1: u64 = 1687856400000000000; // 27/06/23 0900 gmt
        assert_eq!(nearest_past_hour(time_ms_1), time_ms_1);

        //  Input at half past hour
        let time_ms_2: u64 = 1687858200000000000; // 09:30:00 gmt
        assert_eq!(nearest_past_hour(time_ms_2), 1687856400000000000); // 27/06/23 0900 gmt

        // 1 nano before the hour
        let time_ms_3: u64 = 1687856399999999999;
        assert_eq!(nearest_past_hour(time_ms_3), 1687852800000000000); // 27/06/23 0800 gmt

        //  1 nano before end of the hour
        let time_ms_4: u64 = 1687859999999999999;
        assert_eq!(nearest_past_hour(time_ms_4), 1687856400000000000); // 27/06/23 0900 gmt
    }

    #[test]
    fn test_nearest_past_day() {
        //  Input already on an day boundary
        let time_ms_1: u64 = 1687824000000000000; // 27/06/23 0000 gmt
        assert_eq!(nearest_day_start(time_ms_1), time_ms_1);

        //  Input at mid-day (12:00)
        let time_ms_2: u64 = 1687867200000000000; 
        assert_eq!(nearest_day_start(time_ms_2), time_ms_1); // 27/06/23 0000 gmt

        // 1 nano after midnight
        let time_ms_3: u64 = 1687824000000000001;
        assert_eq!(nearest_day_start(time_ms_3), time_ms_1); // 27/06/23 0000 gmt

        //  1 nano before next day 
        let time_ms_4: u64 = 1687910399999999999;
        assert_eq!(nearest_day_start(time_ms_4), time_ms_1); // 27/06/23 0000 gmt
    }

    #[test]
    fn test_combine_ps(){
        let pr = String::from("2vxsx-fae");
        let sa = DEFAULT_SUBACCOUNT.to_string();
        let st = principal_subaccount_to_string(pr, sa);
        let res = String::from("2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000000");
        assert_eq!(res, st);
    }

    #[test]
    fn test_parse_account(){
        let inpt = String::from("2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000000");
        let output = parse_icrc_account(&inpt).unwrap();
        let pr = output.0;
        let ac = output.1;
        assert_eq!(pr, String::from("2vxsx-fae"));
        assert_eq!(ac, String::from("0000000000000000000000000000000000000000000000000000000000000000"));
    }

    // ICP LEDGER TYPE - 30 days
    #[test]
    fn process_daily_data(){
        // set hourly time
        RUNTIME_STATE.with(|s|{
            s.borrow_mut().data.latest_blocks.hours_nano = 30_u64 * DAY_AS_NANOS;
        });
        // set return len
        RUNTIME_STATE.with(|s|{
            s.borrow_mut().data.max_return_length = 5;
        });
        let time_now = 1_688_888_888_889_999_888;
        let process_from = RUNTIME_STATE.with(|s|{
            let hr_nanos = s.borrow().data.latest_blocks.hours_nano;
            return time_now - hr_nanos;
        });

        let test_txs = ptx_test_data();
        RUNTIME_STATE.with(|s|{
            s.borrow_mut().data.latest_blocks.push_tx_vec(test_txs, time_now)
        }); 

        let res = calculate_time_stats(process_from,StatsType::Daily, IndexerType::DfinityIcrc2, time_now);
        
        // totals
        assert_eq!(res.total_transaction_count, 31);
        assert_eq!(res.total_transaction_value, 1_004_132_500_001);
        assert_eq!(res.total_transaction_average, 32391370967.774193);

        // Mint/ Burn/ Transfer/ Approve stats
        let sum_all =   res.burn_stats.total_value +
                        res.mint_stats.total_value +
                        res.transfer_stats.total_value +
                        res.approve_stats.total_value;

        let count_all =   res.burn_stats.count +
                                res.mint_stats.count +
                                res.transfer_stats.count +
                                res.approve_stats.count; 

        assert_eq!(sum_all, 1_005_132_500_001); // 1_000_000_000 higher than total_transaction_value b/c approve is counted      
        assert_eq!(count_all, 31);

        // Total Unique Accounts
        assert_eq!(res.total_unique_accounts, 10);
        
        // Total Unique Principals
        assert_eq!(res.total_unique_principals, 7);

        // Count over time 
        let cot = res.count_over_time.clone();
        assert_eq!(cot.len(), 30); // 30 days of data

        let mut mint_count = 0;
        let mut burn_count= 0;
        let mut trsfr_count= 0;
        let mut approve_count= 0;
        for cnk in cot {
            mint_count += cnk.mint_count;
            burn_count += cnk.burn_count;
            trsfr_count += cnk.transfer_count;
            approve_count += cnk.approve_count;
        }
        let total_cnk = mint_count + burn_count + trsfr_count + approve_count;
        assert_eq!(total_cnk, 31); // all txs accounted for
    }

    #[test]
    fn test_top_by_value(){
        let txs = ptx_test_data();
        let mut trsf: Vec<ProcessedTX> = Vec::new();
        for tx in txs {
            if tx.tx_type == "Transfer" {
                trsf.push(tx);
            }
        }
        let top = top_x_by_txvalue(trsf,5);

        assert_eq!(top[0].tx_value, 500000000);
        assert_eq!(top[1].tx_value, 100000000);
        assert_eq!(top[2].tx_value, 100000000);
        assert_eq!(top[3].tx_value, 80000000);
        assert_eq!(top[4].tx_value, 30000000);
    }

    // test top holders and balances
    #[test]
    fn test_holders_and_balances(){
        // to run this test you need to comment out the auth checks on 
        // get_top_account_holders, get_top_principal_holders, get_account_overview, get_principal_overview
        // in API.rs

        // init test Stable/ Runtime state
        test_state_init();
        
        let ptx = ptx_test_data();
        let stx = processedtx_to_smalltx(&ptx);
        let index_res = process_smtx_to_index(stx);
        let principal_tx = processedtx_to_principal_only_smalltx(&ptx);
        let index_res_pr = process_smtx_to_principal_index(principal_tx);

        // Both indexes process ok
        assert_eq!(index_res, Ok(30_u64));
        assert_eq!(index_res_pr, Ok(30_u64));

        let top = get_top_account_holders(5);
        let top_pr = get_top_principal_holders(5);
        // Top Account
        assert_eq!(top[0].holder, "yvlxg-m3yuk-i2q7x-nqcms-mpyox-fgyj3-molor-v3ley-4kckn-ptfbf-4qe.0000000000000000000000000000000000000000000000000000000000000000".to_string());
        assert_eq!(top[0].data.received, (3, 100600000000));
        assert_eq!(top[0].data.sent, (1, 10010000) );
        assert_eq!(top[0].data.balance, 100589990000);

        // Top Principal
        assert_eq!(top_pr[0].holder, "okuxs-wiaaa-aaaak-qidcq-cai".to_string());
        assert_eq!(top_pr[0].data.received, (8, 301102090001));
        assert_eq!(top_pr[0].data.sent, (10, 811740000) );
        assert_eq!(top_pr[0].data.balance, 300290350001);

        // account overview
        let ac_ov = get_account_overview("yvlxg-m3yuk-i2q7x-nqcms-mpyox-fgyj3-molor-v3ley-4kckn-ptfbf-4qe.0000000000000000000000000000000000000000000000000000000000000000".to_string());
        let ac_ov2 = ac_ov.unwrap();
        assert_eq!(ac_ov2.balance, 100589990000);

        let pr_ov = get_principal_overview("okuxs-wiaaa-aaaak-qidcq-cai".to_string());
        let pr_ov2 = pr_ov.unwrap();
        assert_eq!(pr_ov2.balance, 300290350001);
    }

    #[test]
    fn test_activity_stats(){
         // init test Stable/ Runtime state
         test_state_init();

         let ptx = ptx_test_data();
         let _stx = processedtx_to_smalltx(&ptx);
         let snapshots = STABLE_STATE.with(|s|{
            s.borrow().as_ref().unwrap().activity_stats.get_daily_snapshots(99)
         });
         println!("{:?}", snapshots);
        // First tx date - 28 June 2023
        // last tx date - Sunday, 9 July 2023         
        // 12 days  
        assert_eq!(snapshots.len(), 12);
        assert_eq!(snapshots[0].total_unique_accounts, 10);
        assert_eq!(snapshots[0].active_during_snapshot, 10);
        assert_eq!(snapshots[0].start_time, 1687939200000000000); // first tx time
        assert_eq!(snapshots[0].end_time, 1687996800000000000); // next midnight
        assert_eq!(snapshots[1].end_time - snapshots[0].end_time, D1_AS_NANOS); // window is 24 hours. 
    }

    #[test]
    fn test_account_history() {
        test_state_init();

        let ptx = ptx_test_data_for_history();
        let stx = processedtx_to_smalltx(&ptx);
        let _ = process_smtx_to_index(stx);

        let account_one_args = GetAccountBalanceHistory {
            account: "220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5".to_string(),
            days: 10,
            merge_subaccounts: false,
        };

        let account_one_history = get_account_last_days(account_one_args.clone());
        let time = 8 * 86_400 * 1_000_000_000; 
        let full_history = fill_missing_days(account_one_history, time, account_one_args.days);

        assert_eq!(full_history.len(), 9); // request 10 but day 'today' is 8.. count from 0 makes 9 in total.  

        let init_value = full_history.last().unwrap();
        assert_eq!(init_value.balance, 100_000_000_000); // first balance

        let seccond_day =full_history[7].clone();
        assert_eq!(seccond_day.balance, 200_000_000_000); // next day

        let final_value = full_history[0].clone(); 
        assert_eq!(final_value.balance, 159_999_980_000);
        assert_eq!(final_value.day, 8);
        assert_eq!(final_value.time_of_update, 3 * 86_400 * 1_000_000_000); // last update was day 3
        
    }
    
}