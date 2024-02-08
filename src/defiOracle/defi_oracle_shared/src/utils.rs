use candid::{Nat, Principal};
use num_traits::ToPrimitive;

use crate::shared_types::{ExchangeSnapshot, SwapPair, QuoteBucket, TokenOverview, InternalRateEntry, OverviewV1};

pub fn nat_to_u128(nat: Nat) -> Result<u128, String> {
    match nat.0.to_u128(){
         Some(v) => {
             return Ok(v);
         },
         None => {
             return Err("Not a valid u128".to_string())
         }
    }
 }
 
 pub fn nat_to_u64(nat: Nat) -> Result<u64, String> {
     match nat.0.to_u64(){
          Some(v) => {
              return Ok(v);
          },
          None => {
              return Err("Not a valid u64".to_string())
          }
     }
  }

pub fn round_number_to_f64(number: f64, decimal_places: u32) -> f64 {
    let multiplier = 10u32.pow(decimal_places);
    (number * multiplier as f64).round() / multiplier as f64
}

pub fn round_number_to_u64(number: f64, decimal_places: u32) -> u64 {
    let multiplier = 10u32.pow(decimal_places);
    let mt = (number * multiplier as f64).round() / multiplier as f64;
    if mt < 0_f64 { return 0_u64 } else {return mt as u64 } 
}

pub fn process_mib_quotes(quotes: Vec<ExchangeSnapshot>, max_spread: f64, icp_usd_quote: f64) -> (Vec<TokenOverview>, Vec<InternalRateEntry>) {
    // NOTE: The input order of quotes does not always = the output order of TokenOverview! 

    let mut all_buckets:Vec<QuoteBucket> = Vec::new();
    // init sub-buckets
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::CHAT_ICP ,snapshots: Vec::new() }); // CHAT_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::CKBTC_ICP, snapshots: Vec::new() }); // CKBTC_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::OGY_ICP, snapshots: Vec::new() }); // OGY_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::SNEED_ICP, snapshots: Vec::new() }); // SNEED_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::SNS1_ICP, snapshots: Vec::new() }); // SNS1_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::SONIC_ICP, snapshots: Vec::new() }); // SONIC_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::MOD_ICP, snapshots: Vec::new() }); // MOD_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::BOOM_ICP, snapshots: Vec::new() }); // BOOM_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::GHOST_ICP, snapshots: Vec::new() }); // GHOST_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::HOT_ICP, snapshots: Vec::new() }); // HOT_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::CAT_ICP, snapshots: Vec::new() }); // CAT_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::KINIC_ICP, snapshots: Vec::new() }); // KINIC_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::NUA_ICP, snapshots: Vec::new() }); // NUA_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::ICX_ICP, snapshots: Vec::new() }); // ICX_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::TAL_ICP, snapshots: Vec::new() }); // TAL_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::CKETH_ICP, snapshots: Vec::new() }); // CKETH_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::TRAX_ICP, snapshots: Vec::new() }); // TRAX_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::GLDGOV_ICP, snapshots: Vec::new() }); // GLDGOV_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::NTN_ICP, snapshots: Vec::new() }); // NTN_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::EXE_ICP, snapshots: Vec::new() }); // EXE_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::QUERIO_ICP, snapshots: Vec::new() }); // QUERIO_ICP
    all_buckets.push(QuoteBucket{ swap_pair: SwapPair::MOTOKO_ICP, snapshots: Vec::new() }); // MOTOKO_ICP

    // sort into buckets
    for qt in quotes{
        match qt.swap_pair {
            SwapPair::CHAT_ICP  =>  { if qt.spread_pct <= max_spread  { all_buckets[0].snapshots.push(qt.clone()) }},
            SwapPair::CKBTC_ICP =>  { if qt.spread_pct <= max_spread  { all_buckets[1].snapshots.push(qt.clone()) }},
            SwapPair::OGY_ICP =>    { if qt.spread_pct <= max_spread  { all_buckets[2].snapshots.push(qt.clone()) }},
            SwapPair::SNEED_ICP =>  { if qt.spread_pct <= max_spread  { all_buckets[3].snapshots.push(qt.clone()) }},
            SwapPair::SNS1_ICP =>   { if qt.spread_pct <= max_spread  { all_buckets[4].snapshots.push(qt.clone()) }},
            SwapPair::SONIC_ICP =>  { if qt.spread_pct <= max_spread  { all_buckets[5].snapshots.push(qt.clone()) }},
            SwapPair::MOD_ICP =>    { if qt.spread_pct <= max_spread  { all_buckets[6].snapshots.push(qt.clone()) }},
            SwapPair::BOOM_ICP =>   { if qt.spread_pct <= max_spread  { all_buckets[7].snapshots.push(qt.clone()) }},
            SwapPair::GHOST_ICP =>  { if qt.spread_pct <= max_spread  { all_buckets[8].snapshots.push(qt.clone()) }},
            SwapPair::HOT_ICP =>    { if qt.spread_pct <= max_spread  { all_buckets[9].snapshots.push(qt.clone()) }},
            SwapPair::CAT_ICP =>    { if qt.spread_pct <= max_spread  { all_buckets[10].snapshots.push(qt.clone()) }},
            SwapPair::KINIC_ICP =>  { if qt.spread_pct <= max_spread  { all_buckets[11].snapshots.push(qt.clone()) }},
            SwapPair::NUA_ICP =>    { if qt.spread_pct <= max_spread  { all_buckets[12].snapshots.push(qt.clone()) }},
            SwapPair::ICX_ICP =>    { if qt.spread_pct <= max_spread  { all_buckets[13].snapshots.push(qt.clone()) }},
            SwapPair::TAL_ICP =>    { if qt.spread_pct <= max_spread  { all_buckets[14].snapshots.push(qt.clone()) }},
            SwapPair::CKETH_ICP =>  { if qt.spread_pct <= max_spread  { all_buckets[15].snapshots.push(qt.clone()) }},
            SwapPair::TRAX_ICP =>   { if qt.spread_pct <= max_spread  { all_buckets[16].snapshots.push(qt.clone()) }},
            SwapPair::GLDGOV_ICP => { if qt.spread_pct <= max_spread  { all_buckets[17].snapshots.push(qt.clone()) }},
            SwapPair::NTN_ICP =>    { if qt.spread_pct <= max_spread  { all_buckets[18].snapshots.push(qt.clone()) }},
            SwapPair::EXE_ICP =>    { if qt.spread_pct <= max_spread  { all_buckets[19].snapshots.push(qt.clone()) }},
            SwapPair::QUERIO_ICP =>    { if qt.spread_pct <= max_spread  { all_buckets[20].snapshots.push(qt.clone()) }},
            SwapPair::MOTOKO_ICP =>    { if qt.spread_pct <= max_spread  { all_buckets[21].snapshots.push(qt.clone()) }},
            _ => {}
        }
    }

    let mut op_snapshot_v1:  Vec<TokenOverview> = Vec::new();
    let mut op_internal_rates: Vec<InternalRateEntry> = Vec::new();

    // process each bucket 
    for bkt in all_buckets {

        let mut sum_quotes: f64 = 0.0;
        let quote_len = &bkt.snapshots.len();

        if quote_len > &0 {
            let time_now = ic_cdk::api::time();

            for qt in &bkt.snapshots {
                sum_quotes += qt.price.clone();
            }

            let av_quote = sum_quotes / quote_len.clone() as f64;
            let swp_pr = bkt.swap_pair;
            
            op_snapshot_v1.push(TokenOverview::V1( OverviewV1{
                token_cross: swp_pr.clone().to_string(), 
                snapshot_time: time_now,
                average_price: av_quote.clone(),
                exchange_snapshots: bkt.snapshots.clone(),
                cross_to_usd: icp_usd_quote
            }));

            op_internal_rates.push(InternalRateEntry { swap_pair: swp_pr, quote: av_quote });
        }
    }

   return (op_snapshot_v1, op_internal_rates);
}
