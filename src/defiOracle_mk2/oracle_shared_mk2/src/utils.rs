use candid::{Nat, Principal};
use num_traits::ToPrimitive;

use crate::shared_types::{ExchangeSnapshot, QuoteBucket, TokenOverview, InternalRateEntry, OverviewV1};

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
    
    // get all unique swap_ids
    let mut all_ids:Vec<String> = Vec::new();
    for qts in quotes.iter() {
        all_ids.push(qts.swap_pair.clone());
    }
    let unique_ids = get_unique_string_values(all_ids);
    
    // sort all quotes into unique quote buckets
    let mut all_buckets:Vec<QuoteBucket> = Vec::new();
    for uid in unique_ids {
        all_buckets.push(QuoteBucket{swap_pair: uid.clone(), snapshots: Vec::new()})
    }
    for qte in quotes.iter() {
        for bkt in all_buckets.iter_mut() {
            if &qte.swap_pair == &bkt.swap_pair && qte.spread_pct <= max_spread {
                bkt.snapshots.push(qte.clone())
            }
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
                snapshot_time: time_now.clone(),
                average_price: av_quote.clone(),
                exchange_snapshots: bkt.snapshots.clone(),
                cross_to_usd: icp_usd_quote
            }));

            op_internal_rates.push(InternalRateEntry { swap_pair: swp_pr, quote: av_quote, timestamp: time_now });
        }
    }

   return (op_snapshot_v1, op_internal_rates);
}


pub fn get_unique_string_values(vec: Vec<String>) -> Vec<String> {
    if vec.len() == 0 {return Vec::new()};
    
    let mut working_array: Vec<String> = vec.to_owned();
    let mut keepers: Vec<String> = Vec::new();
    working_array.sort();
    keepers.push(working_array[0].to_owned()); // 1st is always a keeeper
    for i in 1..working_array.len() {
        if working_array[i] != working_array[i-1] {
            keepers.push(working_array[i].to_owned());
        }
    }
    return keepers;
}