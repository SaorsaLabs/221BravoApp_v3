use candid::CandidType;
use serde::Deserialize;
use crate::core::{stable_memory::STABLE_STATE, types::IDKey, utils::{canister_call, log}};
use super::types::{OHLCBucket, OHLC};



#[derive(CandidType, Deserialize, Clone, Default, Debug)]
pub struct PriceDataResult {
    cross: String, 
    active_from: u64,
    last_update: u64,
    pub m5: Vec<OHLC>,  
    pub m15: Vec<OHLC>, 
    pub h1: Vec<OHLC>,  
    pub d1: Vec<OHLC>, 
    pub w1: Vec<OHLC>   
}

pub fn import_token_history_impl(mut data: PriceDataResult){
    // get tree ref 
    let token_ref = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().directory_data.get_ref(data.cross.clone())
    });
    if token_ref == None { ic_cdk::trap("Token has not been added to OHLC Store yet!");}
    // update processing bucket
    let cross_idk = IDKey::from_string(data.cross).unwrap();
    let last_m5 =  data.m5.len();
    let last_m15 = data.m15.len();
    let last_h1 =  data.h1.len();
    let last_d1 =  data.d1.len();
    let last_w1 =  data.w1.len();
    let new_pb = OHLCBucket{
        cross: cross_idk,
        m5:  data.m5[last_m5-1].clone(),
        m15: data.m15[last_m15-1].clone(),
        h1:  data.h1[last_h1-1].clone(),
        d1:  data.d1[last_d1-1].clone(),
        w1:  data.w1[last_w1-1].clone()
    };
    STABLE_STATE.with(|s|{
        let mut binding = s.borrow_mut();
        binding.as_mut().unwrap().overwrite_processing_bucket(new_pb)
    });
    // update btree store
    data.m5.remove(last_m5-1);
    data.m15.remove(last_m15-1);
    data.h1.remove(last_h1-1);
    data.d1.remove(last_d1-1);
    data.w1.remove(last_w1-1);
    STABLE_STATE.with(|s|{
        let mut binding = s.borrow_mut();
        let state = binding.as_mut().unwrap();
        state.price_data_tree.import_m5_history(token_ref.unwrap(), data.m5);
        state.price_data_tree.import_m15_history(token_ref.unwrap(), data.m15);
        state.price_data_tree.import_h1_history(token_ref.unwrap(), data.h1);
        state.price_data_tree.import_d1_history(token_ref.unwrap(), data.d1);
        state.price_data_tree.import_w1_history(token_ref.unwrap(), data.w1);
    })
}

pub async fn fetch_token_history(){
    let all_crosses = STABLE_STATE.with(|s|{
        let binding = s.borrow();
        binding.as_ref().unwrap().get_all_crosses()
    });

    for cs in all_crosses {
        let call_result:Result<(Option<PriceDataResult>,), (ic_cdk::api::call::RejectionCode, String)> = 
        canister_call("lo4kk-kyaaa-aaaak-qcska-cai", "get_all_data", cs.clone(), None).await;
        match call_result {
            Ok(v) => {
                if let Some(data) = v.0 {
                    import_token_history_impl(data);
                    log(format!("{} data imported", cs));
                }
            },
            Err(e) => {
                log(format!("Import Error - {:?}, {}", e.0, e.1));
            }
        }
    }
}