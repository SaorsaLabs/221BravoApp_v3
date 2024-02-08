use defi_oracle_shared::shared_types::{TokenOverview, OverviewV1, SwapPair, ExchangeSnapshot, Marketplace};

use crate::cpc::constants::MAX_SPREAD_FOR_UPDATE;


pub fn vec_token_overview() -> Vec<TokenOverview> {
    let mut ret: Vec<TokenOverview> = Vec::new();

    // 0
    ret.push(TokenOverview::V1(
        OverviewV1{ 
            token_cross: SwapPair::CHAT_ICP.to_string(), 
            snapshot_time: 1672531200000, // 01/01/23 00:00
            average_price: 0.063, 
            exchange_snapshots: vec![
                ExchangeSnapshot{ 
                    snapshot_time: 1672531200000, 
                    swap_pair: SwapPair::CHAT_ICP, 
                    exchange: Marketplace::ICDEX, 
                    price: 0.063, 
                    bid: 6300000, 
                    ask: 6300000, 
                    spread_pct: 0.0, 
                    liquidity: (0,0) 
                },
                ExchangeSnapshot{ 
                    snapshot_time: 1672531200000, 
                    swap_pair: SwapPair::CHAT_ICP, 
                    exchange: Marketplace::ICPSWAP, 
                    price: 0.063, 
                    bid: 6300000, 
                    ask: 6300000, 
                    spread_pct: 0.0, 
                    liquidity: (0,0) 
                },
                ExchangeSnapshot{ 
                    snapshot_time: 1672531200000, 
                    swap_pair: SwapPair::CHAT_ICP, 
                    exchange: Marketplace::SONIC, 
                    price: 0.063, 
                    bid: 6300000, 
                    ask: 6300000, 
                    spread_pct: 0.0, 
                    liquidity: (0,0) 
                },
            ],
            cross_to_usd: 1.0
        }
    ));

    return ret;
}


pub fn vec_exchange_snapshots() -> Vec<ExchangeSnapshot> {
    let mut ret: Vec<ExchangeSnapshot> = Vec::new();

    // Test data
    // CKBTC - 3 quotes all within max spread (8258, 8257.5, 8257) AV = 8257.5
    // CHAT - 1 quote within max spread (0.05) AV = 0.05
    // SNS1 = 3 quotes, 2 within max spread (420.0 425.0) AV = 422.5
    
    ret.push(ExchangeSnapshot { 
        snapshot_time: 1672531200000, // 01/01/23 00:00
        swap_pair: SwapPair::CKBTC_ICP, 
        exchange: Marketplace::ICDEX, 
        price: 8258.0, 
        bid: 825800000000, 
        ask: 825800000000, 
        spread_pct: 0.0, 
        liquidity: (0,0)
    });

    ret.push(ExchangeSnapshot { 
        snapshot_time: 1672531200000, // 01/01/23 00:00
        swap_pair: SwapPair::CKBTC_ICP, 
        exchange: Marketplace::ICPSWAP, 
        price: 8257.5, 
        bid: 825750000000, 
        ask: 825750000000, 
        spread_pct: 0.0, 
        liquidity: (0,0)
    });

    ret.push(ExchangeSnapshot { 
        snapshot_time: 1672531200000, // 01/01/23 00:00
        swap_pair: SwapPair::CKBTC_ICP, 
        exchange: Marketplace::SONIC, 
        price: 8257.0, 
        bid: 825700000000, 
        ask: 825700000000, 
        spread_pct: 0.0, 
        liquidity: (0,0)
    });

    ret.push(ExchangeSnapshot { 
        snapshot_time: 1672531200000, // 01/01/23 00:00
        swap_pair: SwapPair::CHAT_ICP, 
        exchange: Marketplace::SONIC, 
        price: 0.05, 
        bid: 5000000, 
        ask: 5000000, 
        spread_pct: 0.0, 
        liquidity: (0,0)
    });

    ret.push(ExchangeSnapshot { 
        snapshot_time: 1672531200000, // 01/01/23 00:00
        swap_pair: SwapPair::SNS1_ICP, 
        exchange: Marketplace::ICDEX, 
        price: 420.0, 
        bid: 42000000000, 
        ask: 42000000000, 
        spread_pct: 0.0, 
        liquidity: (0,0)
    });

    ret.push(ExchangeSnapshot { 
        snapshot_time: 1672531200000, // 01/01/23 00:00
        swap_pair: SwapPair::SNS1_ICP, 
        exchange: Marketplace::ICPSWAP, 
        price: 425.0, 
        bid: 42500000000, 
        ask: 42500000000, 
        spread_pct: 0.0, 
        liquidity: (0,0)
    });

    ret.push(ExchangeSnapshot { 
        snapshot_time: 1672531200000, // 01/01/23 00:00
        swap_pair: SwapPair::SNS1_ICP, 
        exchange: Marketplace::SONIC, 
        price: 430.0, 
        bid: 64000000000, 
        ask: 43000000000, 
        spread_pct: MAX_SPREAD_FOR_UPDATE + 1.0, 
        liquidity: (0,0)
    });

    return ret;
}