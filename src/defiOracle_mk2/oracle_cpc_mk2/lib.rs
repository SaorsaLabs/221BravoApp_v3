mod core;
mod cpc;
mod timers;
mod http;
mod test_data;


#[cfg(test)]
mod tests {
    use oracle_shared_mk2::{shared_types::{TokenOverview, InternalRateEntry, ExchangeSnapshot}, utils::process_mib_quotes};

    use crate::{
        cpc::{
            utils::{extract_quote_ticker, convert_quotes, AltQuotes}, constants::MAX_SPREAD_FOR_UPDATE
        },
        test_data::{vec_token_overview, vec_exchange_snapshots}, 
        core::runtime::RUNTIME_STATE
    };

    #[test]
    fn test_extract_quote_ticker() {
        let ticker1 = "CKBTC/ICP".to_string();
        let res1 = extract_quote_ticker(&ticker1);
        let ticker2 = "ICP/USD".to_string();
        let res2 = extract_quote_ticker(&ticker2);
        let ticker3 = "ICP_USD".to_string();
        let res3 = extract_quote_ticker(&ticker3);
        assert_eq!(res1, Some("ICP"));
        assert_eq!(res2, Some("USD"));
        assert_eq!(res3, None);
    }

    #[test]
    fn test_convert_quotes(){
        // get test data
        let test_data = vec_token_overview();
        // set USD quote 
        RUNTIME_STATE.with(|s|{
            let qte = InternalRateEntry{ swap_pair: String::from("ICP/USD"), quote: 1.5, timestamp: 0};
            s.borrow_mut().data.internal_rates.add_swap_pair_to_vec(qte);
        });
        // convert 
        let c1 = convert_quotes(test_data, AltQuotes::USD).unwrap();
        let c1_data;
        match &c1[0] {
            TokenOverview::V1(v) => { c1_data = v.to_owned() }
        }
        // check
        assert_eq!(c1_data.average_price, 0.0945_f64); // 0.063 * 1.5 = 0.0945
        assert_eq!(c1_data.exchange_snapshots[0].price, 0.0945_f64);
        assert_eq!(c1_data.exchange_snapshots[1].price, 0.0945_f64);
        assert_eq!(c1_data.exchange_snapshots[2].price, 0.0945_f64);
    }

    #[test]
    fn test_process_mib_quotes(){
        // NOTE - Have to comment out ic_cdk::api::time in process_mib_quotes to get test to run! 

        let data: Vec<ExchangeSnapshot> = vec_exchange_snapshots();
        let processed: (Vec<TokenOverview>, Vec<InternalRateEntry>) = process_mib_quotes(data, MAX_SPREAD_FOR_UPDATE, 1.0);
        let overviews: Vec<TokenOverview> = processed.0;
        let ov1 = overviews[0].get_v1_data().unwrap();
        let ov2 = overviews[1].get_v1_data().unwrap();
        let ov3 = overviews[2].get_v1_data().unwrap();
 
        assert_eq!(overviews.len(), 3);
        assert_eq!(ov1.average_price, 0.05);
        assert_eq!(ov1.exchange_snapshots.len(), 1);
        assert_eq!(ov2.average_price, 8257.5);
        assert_eq!(ov2.exchange_snapshots.len(), 3);
        assert_eq!(ov3.average_price, 422.5);
        assert_eq!(ov3.exchange_snapshots.len(), 2);
    }
    
    
}