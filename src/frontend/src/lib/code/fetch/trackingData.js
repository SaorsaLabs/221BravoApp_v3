import { getIdentity, icActor } from "./icAgent";
import { trackingDataIDL } from '../IDL/trackingData.js';
import { getTokenData } from '../utils.js';

export async function fetchExchangeTrackingData(token){
    const Frontend_ID = getIdentity();
    let tokenData = getTokenData(token);
    let actor = icActor(tokenData.trackingCanister, trackingDataIDL, Frontend_ID);
    let data = await actor.get_outcome_data_from_package("EXPKG");
    return data;
}

export async function fetchDexTrackingData(token){
    const Frontend_ID = getIdentity();
    let tokenData = getTokenData(token);
    let actor = icActor(tokenData.trackingCanister, trackingDataIDL, Frontend_ID);
    let data = await actor.get_outcome_data_from_package("DEXPKG");
    return data;
}