use candid::Nat;
use ic_stable_memory::stable_memory_init;

use crate::{stats::custom_types::ProcessedTX, core::{runtime::{RUNTIME_STATE, RuntimeState, Data}, working_stats::WorkingStats, stable_memory::{Main, STABLE_STATE}}};

pub fn test_state_init(){
    stable_memory_init();
    // init stable state
    let mut stable_data = Main::default();

    STABLE_STATE.with(|state| {
        *state.borrow_mut() = Some(stable_data);
    });
    
    // init runtime state
    let mut runtime_state = RuntimeState::default();
        runtime_state.data.set_ledger_fee(10_000);
        RUNTIME_STATE.with(|state| {
        *state.borrow_mut() = runtime_state;
    });
}


pub fn ptx_test_data() -> Vec<ProcessedTX> {
    // TEST DATA OVERVIEW
    // 
    // Test cases - mint, burn, transfer, approve, self transfer, 0 values, optional fee
    //
    // 0000000000000000000000000000000000000000000000000000000000000000 (default subaccount)
    // 
    // Accounts used (10) - 
    // 1. okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000000  
    // 2. okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000001
    // 3. okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000002
    // 4. 2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000000
    // 5. 2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000001
    // 6. 5u5em-tfqms-fnxvj-u4p5w-y5p6z-viwwl-brxk3-asjj2-rv5eo-y4eu7-vqe.0000000000000000000000000000000000000000000000000000000000000000
    // 7. yvlxg-m3yuk-i2q7x-nqcms-mpyox-fgyj3-molor-v3ley-4kckn-ptfbf-4qe.0000000000000000000000000000000000000000000000000000000000000000
    // 8. tr3th-kiaaa-aaaaq-aab6q-cai.7776d299b4a804a14862b02bff7b74d1b956e431f5f832525d966d67ff3d7ce8
    // 9. pu7qb-4nvjm-hletz-mddll-43mnq-njy66-kslhu-wa5xn-iavlt-l3kgf-bqe.0000000000000000000000000000000000000000000000000000000000000000
    // 10. ne2vj-6yaaa-aaaag-qb3ia-cai.0a0000000002211b110101000000000000000000000000000000000000000000
    //
    // ACCOUNT BALANCES/ TransferS 
    // 
    // (1)
    // okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000000
    // END Balance: 100_269_470_001
    // OVERVIEW
    //    first_active: 1687939200000000000, 
    //    last_active: 1688888888888888888, 
    //    sent: (7, 730620000), --1 is approve, 1 has extra 50k fee
    //    received: (4, 101000090001),
    //    balance: 100269470001, 
    // txs: transfer 8, burn 0, mint 2, 1 approve;
    //
    // (2)
    // okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000001
    // END Balance: 
    // txs: transfer 3, burn 1, mint 1;
    //
    // (3)
    // okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000002
    // END Balance: 
    // txs: transfer 1, burn 0, mint 1; 
    // TOP PRINCIPAL
    // Overview { first_active: 1687939200000000000, last_active: 1688888888888888888, 
    //    sent: (10, 811740000), received: (8, 301102090001), balance: 300290350001 }
    //
    // (4)
    // 2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000000
    // END Balance: 
    // txs: transfer 1, burn 1, mint 3;
    //
    // (5)
    // 2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000001
    // END Balance: 
    // txs: transfer 2, burn 0, mint 1;
    //
    // (6)
    // 5u5em-tfqms-fnxvj-u4p5w-y5p6z-viwwl-brxk3-asjj2-rv5eo-y4eu7-vqe.0000000000000000000000000000000000000000000000000000000000000000
    // END Balance: 
    // txs: transfer 3, burn 0, mint 1;
    //
    // (7)
    // yvlxg-m3yuk-i2q7x-nqcms-mpyox-fgyj3-molor-v3ley-4kckn-ptfbf-4qe.0000000000000000000000000000000000000000000000000000000000000000
    // END Balance: 
    // txs: transfer 3, burn 0, mint 1;
    // data: Overview { 
    //     first_active: 1687980444040000000, 
    //     last_active: 1687988766132000000, 
    //     sent: (1, 10010000), 
    //     received: (3, 100600000000), 
    //     balance: 100589990000 } 
    //
    // (8)
    // tr3th-kiaaa-aaaaq-aab6q-cai.7776d299b4a804a14862b02bff7b74d1b956e431f5f832525d966d67ff3d7ce8
    // END Balance: 
    // txs: transfer 1, burn 0, mint 1;
    //
    // (9)
    // pu7qb-4nvjm-hletz-mddll-43mnq-njy66-kslhu-wa5xn-iavlt-l3kgf-bqe.0000000000000000000000000000000000000000000000000000000000000000
    // END Balance: 
    // txs: transfer 1, burn 0, mint 1;
    //
    // (10)
    // ne2vj-6yaaa-aaaag-qb3ia-cai.0a0000000002211b110101000000000000000000000000000000000000000000
    // END Balance: 
    // txs: transfer 0, burn 1, mint 2;


    let mut txs = Vec::new();
    // let start_time: u64 = 1_687_939_200_000_000_000; // Wednesday, 28 June 2023 08:00:00

    // *** init Mints 
    txs.push(
        ProcessedTX {
            block: 0,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "Token Ledger".to_string(),
            to_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_939_200_000_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 1,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "Token Ledger".to_string(),
            to_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_939_750_000_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 2,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "Token Ledger".to_string(),
            to_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000002".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_940_455_000_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 3,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "Token Ledger".to_string(),
            to_account: "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_940_459_000_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 4,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "Token Ledger".to_string(),
            to_account: "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_940_459_000_010_001,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 5,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "Token Ledger".to_string(),
            to_account: "5u5em-tfqms-fnxvj-u4p5w-y5p6z-viwwl-brxk3-asjj2-rv5eo-y4eu7-vqe.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_944_466_000_010_001,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 6,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "Token Ledger".to_string(),
            to_account: "yvlxg-m3yuk-i2q7x-nqcms-mpyox-fgyj3-molor-v3ley-4kckn-ptfbf-4qe.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_980_444_040_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 7,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "Token Ledger".to_string(),
            to_account: "tr3th-kiaaa-aaaaq-aab6q-cai.7776d299b4a804a14862b02bff7b74d1b956e431f5f832525d966d67ff3d7ce8".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_980_444_040_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 8,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "Token Ledger".to_string(),
            to_account: "pu7qb-4nvjm-hletz-mddll-43mnq-njy66-kslhu-wa5xn-iavlt-l3kgf-bqe.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_980_448_040_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 9,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "Token Ledger".to_string(),
            to_account: "ne2vj-6yaaa-aaaag-qb3ia-cai.0a0000000002211b110101000000000000000000000000000000000000000000".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_980_449_040_000_000,
            tx_fee: None,
            spender: None,
        }
    );
    
    txs.push(
        ProcessedTX { // 0 Transfer??  Can TX BE 0 value? 
            block: 10,
            hash: "No-hash".to_string(),
            tx_type: "Transfer".to_string(),
            from_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_account: "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            tx_value: 0,
            tx_time: 1_687_980_500_040_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 11,
            hash: "No-hash".to_string(),
            tx_type: "Transfer".to_string(),
            from_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_account: "yvlxg-m3yuk-i2q7x-nqcms-mpyox-fgyj3-molor-v3ley-4kckn-ptfbf-4qe.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: 500_000_000,
            tx_time: 1_687_980_700_040_000_000,
            tx_fee: Some(60_000), // NOTE !! 
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 12,
            hash: "No-hash".to_string(),
            tx_type: "Burn".to_string(),
            from_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            to_account: "Token Ledger".to_string(),
            tx_value: 79_000_000,
            tx_time: 1_687_988_700_540_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 13,
            hash: "No-hash".to_string(),
            tx_type: "Transfer".to_string(),
            from_account: "5u5em-tfqms-fnxvj-u4p5w-y5p6z-viwwl-brxk3-asjj2-rv5eo-y4eu7-vqe.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_account: "5u5em-tfqms-fnxvj-u4p5w-y5p6z-viwwl-brxk3-asjj2-rv5eo-y4eu7-vqe.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: 400_000,
            tx_time: 1_687_988_701_540_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 14,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "Token Ledger".to_string(),
            to_account: "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: 100_000_000,
            tx_time: 1_687_988_703_540_000_000,
            tx_fee: None,
            spender: None,
        }
    );
    
    txs.push(
        ProcessedTX {
            block: 15,
            hash: "No-hash".to_string(),
            tx_type: "Transfer".to_string(),
            from_account: "5u5em-tfqms-fnxvj-u4p5w-y5p6z-viwwl-brxk3-asjj2-rv5eo-y4eu7-vqe.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: 1,
            tx_time: 1_687_988_705_540_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 16,
            hash: "No-hash".to_string(),
            tx_type: "Burn".to_string(),
            from_account: "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_account: "Token Ledger".to_string(),
            tx_value: 500_000,
            tx_time: 1_687_988_707_540_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 17,
            hash: "No-hash".to_string(),
            tx_type: "Transfer".to_string(),
            from_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            to_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: 90_000,
            tx_time: 1_687_988_709_540_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 18,
            hash: "No-hash".to_string(),
            tx_type: "Transfer".to_string(),
            from_account: "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            to_account: "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: 80_000_000,
            tx_time: 1_687_988_712_540_000_000,
            tx_fee: None,
            spender: None,
        }
    );
    
    txs.push(
        ProcessedTX {
            block: 19,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "Token Ledger".to_string(),
            to_account: "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            tx_value: 80_010_000,
            tx_time: 1_687_988_714_540_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 20,
            hash: "No-hash".to_string(),
            tx_type: "Transfer".to_string(),
            from_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_account: "pu7qb-4nvjm-hletz-mddll-43mnq-njy66-kslhu-wa5xn-iavlt-l3kgf-bqe.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: 30_000_000,
            tx_time: 1_687_988_718_000_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 21,
            hash: "No-hash".to_string(),
            tx_type: "Transfer".to_string(),
            from_account: "yvlxg-m3yuk-i2q7x-nqcms-mpyox-fgyj3-molor-v3ley-4kckn-ptfbf-4qe.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_account: "tr3th-kiaaa-aaaaq-aab6q-cai.7776d299b4a804a14862b02bff7b74d1b956e431f5f832525d966d67ff3d7ce8".to_string(),
            tx_value: 10_000_000,
            tx_time: 1_687_988_724_666_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 22,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "Token Ledger".to_string(),
            to_account: "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: 50_000_000,
            tx_time: 1_687_988_728_132_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 23,
            hash: "No-hash".to_string(),
            tx_type: "Transfer".to_string(),
            from_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_account: "yvlxg-m3yuk-i2q7x-nqcms-mpyox-fgyj3-molor-v3ley-4kckn-ptfbf-4qe.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: 100_000_000,
            tx_time: 1_687_988_766_132_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 24,
            hash: "No-hash".to_string(),
            tx_type: "Transfer".to_string(),
            from_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            to_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000002".to_string(),
            tx_value: 20_000_00,
            tx_time: 1_687_988_787_872_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 25,
            hash: "No-hash".to_string(),
            tx_type: "Burn".to_string(),
            from_account: "ne2vj-6yaaa-aaaag-qb3ia-cai.0a0000000002211b110101000000000000000000000000000000000000000000".to_string(),
            to_account: "Token Ledger".to_string(),
            tx_value: 1_000_000_000,
            tx_time: 1_687_988_788_872_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 26,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "Token Ledger".to_string(),
            to_account: "ne2vj-6yaaa-aaaag-qb3ia-cai.0a0000000002211b110101000000000000000000000000000000000000000000".to_string(),
            tx_value: 1_000_000_000,
            tx_time: 1_687_988_788_972_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 27,
            hash: "No-hash".to_string(),
            tx_type: "Transfer".to_string(),
            from_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            tx_value: 100_000_000,
            tx_time: 1_687_988_888_972_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 28,
            hash: "No-hash".to_string(),
            tx_type: "Transfer".to_string(),
            from_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_account: "5u5em-tfqms-fnxvj-u4p5w-y5p6z-viwwl-brxk3-asjj2-rv5eo-y4eu7-vqe.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: 500_000,
            tx_time: 1_688_888_888_888_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 29,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "Token Ledger".to_string(),
            to_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: 1_000_000_000,
            tx_time: 1_688_888_888_888_888_888,
            tx_fee: None,
            spender: None,
        }
    );

    txs.push(
        ProcessedTX {
            block: 30,
            hash: "No-hash".to_string(),
            tx_type: "Approve".to_string(),
            from_account: "okuxs-wiaaa-aaaak-qidcq-cai.0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_account: "Token Ledger".to_string(),
            tx_value: 1_000_000_000,
            tx_time: 1_688_888_888_888_000_000,
            tx_fee: None,
            spender: None,
        }
    );

    return txs;
}