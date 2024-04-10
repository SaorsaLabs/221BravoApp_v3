
use oracle_shared_mk2::shared_types::StableCurrency;
use crate::core::runtime::RUNTIME_STATE;
pub fn log(text: impl AsRef<str>){
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.add_log(text.as_ref().to_string())
    });
}

pub fn get_trade_size_exs(swap_pair: &String, swap_type: &u8, reverse: &bool, base_decimals: &u32, quote_decimals: &u32, stable_size: &f64, stable_currency: &StableCurrency) -> Option<(u128, u128)> {
    let internal_rates = RUNTIME_STATE.with(|s|{
        s.borrow().data.internal_rates.clone()
    });

    match swap_type {
        0_u8 => { 
           // ICP_XDR & ICP_USD -- not used at this time
            return None;
        },
        1_u8 => { // Cross has ICP in it.
            let rate = internal_rates.get_single_rate(&swap_pair);
            match rate {
                Some(swap_rate) => {
                    // not reverse = ICP it the QUOTE
                    // get stable rate
                    let mut stable_cross: Option<f64> = None;
                    if stable_currency == &StableCurrency::XDR { stable_cross = internal_rates.get_single_rate(&String::from("ICP/XDR")) };
                    if stable_currency == &StableCurrency::USD { stable_cross = internal_rates.get_single_rate(&String::from("ICP/USD")) };
                    if stable_currency == &StableCurrency::ICP { stable_cross = Some(1.00); };
                    match stable_cross {
                        Some(cross) => { 
                            let icp_dcmls = 100000000_f64;
                            let full_trade_size = (icp_dcmls/cross)*stable_size;
                            let trade_base1 = full_trade_size/swap_rate;
                            let dec0_dec1_power = 10f64.powi(*base_decimals as i32 - *quote_decimals as i32);
                            let trade_base2 = trade_base1*dec0_dec1_power;
                            
                            if reverse == &false {
                                // catch undershoot
                                if trade_base2 > 0.0 && full_trade_size > 0.0 {
                                    return Some((trade_base2 as u128, full_trade_size as u128));
                                } else {
                                    return None;
                                }
                            } else {
                                 // catch undershoot
                                 if full_trade_size > 0.0 {
                                    return Some((full_trade_size as u128, trade_base2 as u128));
                                } else {
                                    return None;
                                }
                            }
                        }, 
                        None => { return None;}
                    }
                },
                None => { return None },
            }
        },
        _ => { return None; }
    }
}


