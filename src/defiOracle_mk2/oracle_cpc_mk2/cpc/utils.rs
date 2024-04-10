use candid::CandidType;
use oracle_shared_mk2::{
    shared_types::{OverviewV1, TokenOverview, ExchangeSnapshot}
};
use serde::{Serialize, Deserialize};
use crate::core::runtime::RUNTIME_STATE;

pub fn log(text: impl AsRef<str>){
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.add_log(text.as_ref().to_string())
    });
}

pub fn convert_quotes(input: Vec<TokenOverview>, output_currency: AltQuotes) -> Result<Vec<TokenOverview>, String> {
    let opt_cross = RUNTIME_STATE.with(|s|{
        match output_currency {
            AltQuotes::ICP => {
                Some(1_f64)
            },
            AltQuotes::USD => {
                s.borrow().data.internal_rates.get_single_rate(&String::from("ICP/USD"))
            }
            AltQuotes::XDR => {
                s.borrow().data.internal_rates.get_single_rate(&String::from("ICP/XDR"))
            }
        }
        
    });
    let cross_rate: f64;
    if let Some(rate) = opt_cross {
        cross_rate = rate;
    } else {
        RUNTIME_STATE.with(|s|{
            s.borrow_mut().stats.metrics.increment_total_errors()
        });
        return Err(String::from("Error: Could not retrive Cross Rate. (fn convert quotes)"));
    }
    
    let mut ret_vec: Vec<TokenOverview> = Vec::new();
    for tov in &input {
        match tov {
            TokenOverview::V1(data) => {
                let quote_ticker = extract_quote_ticker(&data.token_cross);
                if let Some(v) = quote_ticker {
                    match v {
                        "ICP" => { // quote is ICP
                            match output_currency {
                                AltQuotes::USD => {
                                    ret_vec.push(update_overview_v1(data.clone(), cross_rate));
                                },
                                AltQuotes::XDR => {
                                    ret_vec.push(update_overview_v1(data.clone(), cross_rate));
                                },
                                AltQuotes::ICP => { return Ok(input) }
                            }
                        },
                        _ => {
                            RUNTIME_STATE.with(|s|{
                                s.borrow_mut().stats.metrics.increment_total_errors()
                            });
                            return Err(String::from("Error: Only ICP quotes can be converted. (fn convert quotes)"));
                        },
                    }
                } else {
                    RUNTIME_STATE.with(|s|{
                        s.borrow_mut().stats.metrics.increment_total_errors()
                    });
                    return Err(String::from("Error: Could not extract ticker. (fn convert quotes)"));
                }
            },
            // V2 etc (if ever required)
        }
    }
    return Ok(ret_vec);
}

pub fn update_overview_v1(overview: OverviewV1, rate: f64) -> TokenOverview {
    let av_price = overview.average_price*rate;
    let mut ex_vec: Vec<ExchangeSnapshot> = Vec::new();
    for ex in overview.exchange_snapshots {
        let mut temp = ex.clone();
        temp.ask = (ex.ask as f64 *rate) as u128;
        temp.bid = (ex.bid as f64 *rate) as u128;
        temp.price = ex.price*rate;
        ex_vec.push(temp);
    }
    let ret = TokenOverview::V1(
        OverviewV1 {
        token_cross: overview.token_cross,
        snapshot_time: overview.snapshot_time,
        average_price: av_price,
        exchange_snapshots: ex_vec,
        cross_to_usd: rate
        }
    );

    return ret;
}

pub fn extract_quote_ticker(input: &String) -> Option<&str> {
    if let Some(index) = input.find('/') {
        // Check if the '/' is not the last character in the string
        if index < input.len() - 1 {
            return Some(&input[index + 1..]);
        }
    }
    None
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub enum AltQuotes{
    #[default]
    ICP,
    USD,
    XDR
}

// ARBS
// pub async fn check_for_arbs(token_overviews: Vec<OverviewV1>){
//     for ov in token_overviews {
//         let mut lowest_ask: u64 = u64::MAX;
//         let mut highest_bid: u64 = u64::MIN;
//         let mut buy_exchange:String = String::from("-");
//         let mut sell_exchange:String = String::from("-");

//         // get high/ lows
//         for snap in ov.exchange_snapshots {
//             if snap.ask < lowest_ask { 
//                 lowest_ask = snap.ask;
//                 buy_exchange = snap.exchange.to_string();
//             }
//             if snap.bid > highest_bid {
//                 highest_bid = snap.bid;
//                 sell_exchange = snap.exchange.to_string();
//             }
//         }

//         // can arb it? 
//         let arb_pcnt = ((highest_bid as f64 - lowest_ask as f64) / lowest_ask as f64) * 100.0;
//         if arb_pcnt >= MIN_ARB_PERCENT {
//             let time = ic_cdk::api::time();
//             let text = format!(
//                 "
//                 [][] -- ARB HIT -- [][] \n
//                 CROSS: {} \n
//                 Percent: {} % \n
//                 --------------- \n
//                 Market: {} \n
//                 ASK: {} \n
//                 --------------- \n
//                 Market: {} \n
//                 BID: {} \n
//                 --------------- \n
//                 TIME: {} \n
//                 ",
//                 ov.token_cross.to_string(),
//                 arb_pcnt,
//                 buy_exchange,
//                 lowest_ask,
//                 sell_exchange,
//                 highest_bid,
//                 time
//             );
//             oc_log(text).await; // send open chat message
//         }
//     }
// }