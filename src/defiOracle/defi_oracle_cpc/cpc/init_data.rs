extern crate defi_oracle_shared;
use defi_oracle_shared::shared_types::*;
use crate::core::runtime::RUNTIME_STATE;

// CROSSES
pub fn init_swap_data(){

    // clear swap pairs
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().data.internal_rates.clear_swap_pair_vec();
    });
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().data.all_swaps.clear_all_swaps()
    });


    // CKBTC/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::CKBTC_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "CKBTC".to_string(), 
                    ledger: "mxzaz-hqaaa-aaaar-qaada-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC2 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICDEX, 
                canister_id: "k7tml-iaaaa-aaaak-aecgq-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1000
        });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "xmiu5-jqaaa-aaaag-qbz7q-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // CHAT/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::CHAT_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "CHAT".to_string(), 
                    ledger: "2ouva-viaaa-aaaaq-aaamq-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICDEX, 
                canister_id: "3we4s-lyaaa-aaaak-aegrq-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 10000000
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "ne2vj-6yaaa-aaaag-qb3ia-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 10000000
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });
        
            let details = SwapPairDetails { 
                swap_id, 
                token0, 
                token1, 
                marketplaces, 
                active,
                last_quote: 1_u64,
            };
    
            s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // OGY/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::OGY_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "OGY".to_string(), 
                    ledger: "rd6wb-lyaaa-aaaaj-acvla-cai".to_string(), // Old standard? new - jwcfb-hyaaa-aaaaj-aac4q-cai
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICDEX, 
                canister_id: "2xiqo-wqaaa-aaaak-aek3a-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 10000000
        });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "oxpo6-vaaaa-aaaag-qcmxq-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // SNEED/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::SNEED_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "SNEED".to_string(), 
                    ledger: "hvgxa-wqaaa-aaaaq-aacia-cai".to_string(), // old "r7cp6-6aaaa-aaaag-qco5q-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "osyzs-xiaaa-aaaag-qc76q-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // SNS1/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::SNS1_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "SNS1".to_string(), 
                    ledger: "zfcdd-tqaaa-aaaaq-aaaga-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICDEX, 
                canister_id: "32fn4-qqaaa-aaaak-ad65a-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 100000
        });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "3ejs3-eaaaa-aaaag-qbl2a-cai".to_string(), 
                active: true, 
                reverse_cross: true,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: true,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // SONIC/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::SONIC_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "SONIC".to_string(), 
                    ledger: "qbizb-wiaaa-aaaaq-aabwq-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICDEX, 
                canister_id: "symam-gqaaa-aaaak-ae7ya-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 10000000
        });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "jknac-2aaaa-aaaag-qcmfq-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // MOD/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::MOD_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "MOD".to_string(), 
                    ledger: "xsi2v-cyaaa-aaaaq-aabfq-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICDEX, 
                canister_id: "iq6by-ryaaa-aaaak-ae5pq-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 10000000
        });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "tupjz-uyaaa-aaaag-qcjmq-cai".to_string(), 
                active: true, 
                reverse_cross: true,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: true,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // BOOM/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::BOOM_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "BOOM".to_string(), 
                    ledger: "vtrom-gqaaa-aaaaq-aabia-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICDEX, 
                canister_id: "2nmer-3aaaa-aaaak-ae6na-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 10000000
        });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "fdno6-ayaaa-aaaag-qckuq-cai".to_string(), 
                active: true, 
                reverse_cross: true,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: true,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // GHOST/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::GHOST_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "GHOST".to_string(), 
                    ledger: "4c4fd-caaaa-aaaaq-aaa3a-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICDEX, 
                canister_id: "gyh35-piaaa-aaaak-ae3ta-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 10000000000
        });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "dwahc-eyaaa-aaaag-qcgnq-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // HOT/ ICP *** 
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::HOT_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "HOT".to_string(), 
                    ledger: "6rdgd-kyaaa-aaaaq-aaavq-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICDEX, 
                canister_id: "ntwyo-viaaa-aaaak-ae2pa-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 10000000
        });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "rxwy2-zaaaa-aaaag-qcfna-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // CAT/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::CAT_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "CAT".to_string(), 
                    ledger: "uf2wh-taaaa-aaaaq-aabna-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICDEX, 
                canister_id: "zu4fl-riaaa-aaaak-ae6eq-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 10000000
        });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "dggvb-byaaa-aaaag-qckcq-cai".to_string(), 
                active: true, 
                reverse_cross: true,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: true,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // KINIC/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::KINIC_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "KINIC".to_string(), 
                    ledger: "73mez-iiaaa-aaaaq-aaasq-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICDEX, 
                canister_id: "bog3h-7qaaa-aaaak-aexmq-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 10000000
        });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "335nz-cyaaa-aaaag-qcdka-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // NUA/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::NUA_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "NUA".to_string(), 
                    ledger: "rxdbk-dyaaa-aaaaq-aabtq-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICDEX, 
                canister_id: "qxxz5-ziaaa-aaaak-ae7uq-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 10000000
        });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "irtmd-myaaa-aaaag-qcl4q-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // ICX/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::ICX_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "ICX".to_string(), 
                    ledger: "rffwt-piaaa-aaaaq-aabqq-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICDEX, 
                canister_id: "5xpmz-zyaaa-aaaak-ae67q-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 10000000
        });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "m3don-saaaa-aaaag-qclga-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // TAL/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::TAL_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "TAL".to_string(), 
                    ledger: "gnc5v-fyaaa-aaaar-qab2q-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "jwj2t-naaaa-aaaag-qcmhq-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // CKETH/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::CKETH_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "CKETH".to_string(), 
                    ledger: "ss2fx-dyaaa-aaaar-qacoq-cai".to_string(), 
                    decimals: 18, 
                    standard: TokenStandard::ICRC2 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICDEX, 
                canister_id: "ei3bs-6iaaa-aaaak-afgaa-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1000000000000000
        });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "angxa-baaaa-aaaag-qcvnq-cai".to_string(), 
                active: true, 
                reverse_cross: true,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: true,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // TRAX/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::TRAX_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "TRAX".to_string(), 
                    ledger: "emww2-4yaaa-aaaaq-aacbq-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICDEX, 
                canister_id: "krtvm-uqaaa-aaaak-afhcq-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 10_000_000
        });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "mzv7v-5qaaa-aaaag-qcypq-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // GLD/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::GLDGOV_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "GLDGOV".to_string(), 
                    ledger: "tyyy3-4aaaa-aaaaq-aab7a-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICDEX, 
                canister_id: "ilp5q-kaaaa-aaaak-afhnq-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 10_000_000
        });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "k46ek-4qaaa-aaaag-qcyzq-cai".to_string(), 
                active: true, 
                reverse_cross: true,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: true,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // NTN/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::NTN_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "NTN".to_string(), 
                    ledger: "f54if-eqaaa-aaaaq-aacea-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICDEX, 
                canister_id: "imo3e-hyaaa-aaaak-afhna-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 10_000_000
        });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "kv5pw-kyaaa-aaaag-qcyya-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // EXE/ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::EXE_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "EXE".to_string(), 
                    ledger: "rh2pm-ryaaa-aaaan-qeniq-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "dlfvj-eqaaa-aaaag-qcs3a-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // QUERIO/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::QUERIO_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "QUERIO".to_string(), 
                    ledger: "vi5vh-wyaaa-aaaan-qizxa-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "7flwa-kaaaa-aaaag-qcxhq-cai".to_string(), 
                active: true, 
                reverse_cross: true,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: true,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // MOTOKO/ ICP
    RUNTIME_STATE.with(|s|{
        let mut marketplaces: Vec<MarketplaceDetails> = Vec::new();
        
        let swap_id: SwapPair = SwapPair::MOTOKO_ICP;
        let active: bool = true;
        
        let token0 = Token { 
                    ticker: "MOTOKO".to_string(), 
                    ledger: "2tlvc-vqaaa-aaaah-adwxa-cai".to_string(), 
                    decimals: 8, 
                    standard: TokenStandard::ICRC1 
                };

        let token1 = Token { 
                    ticker: "ICP".to_string(), 
                    ledger: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(), 
                    decimals: 8,
                    standard: TokenStandard::ICRC2
                };

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::ICPSWAP, 
                canister_id: "rqvkb-tyaaa-aaaag-qdaiq-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });

        marketplaces.push(
            MarketplaceDetails { 
                marketplace: Marketplace::SONIC, 
                canister_id: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(), 
                active: true, 
                reverse_cross: false,
                unit_size: 1
            });
        
        let details = SwapPairDetails { 
            swap_id, 
            token0, 
            token1, 
            marketplaces, 
            active,
            last_quote: 1_u64,
        };

        s.borrow_mut().data.all_swaps.add_swap_pair(details)
    });

    // INIT INTERNAL QUOTES - sets first quote to allow quote sizing
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::ICP_ICP, quote: 1.0 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::ICP_USD, quote: 11.0 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::ICP_XDR, quote: 3.27 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::CKBTC_ICP, quote: 3800.0 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::CHAT_ICP, quote: 0.0808 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::OGY_ICP, quote: 0.0014 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::SNEED_ICP, quote: 23.1 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::SNS1_ICP, quote: 442.0 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::SONIC_ICP, quote: 0.048 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::MOD_ICP, quote: 0.0021 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::BOOM_ICP, quote: 0.0012 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::GHOST_ICP, quote: 0.000061 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::HOT_ICP, quote: 0.0016 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::CAT_ICP, quote: 0.0026 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::KINIC_ICP, quote: 0.2498 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::NUA_ICP, quote: 0.013 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::ICX_ICP, quote: 0.0067 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::TAL_ICP, quote: 0.09 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::CKETH_ICP, quote: 198.1 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::TRAX_ICP, quote: 0.002 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::GLDGOV_ICP, quote: 0.0057 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::NTN_ICP, quote: 1.22 });
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::EXE_ICP, quote: 0.0382 }); 
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::QUERIO_ICP, quote: 0.0236 }); 
        s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(InternalRateEntry { swap_pair: SwapPair::MOTOKO_ICP, quote: 49.06 }); 
    });
    
}