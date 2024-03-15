import { getIdentity, icActor } from "./icAgent.js";
import { getAllTokenData, combinePrincipalSubAccount, parsePrincipalSubAccountString, processPromises } from "../utils.js";
import { DEFAULT_SUBACCOUNT } from "../constants.js";
import { superIndexIDL } from '../IDL/superIndex.js';
import { getDefaultAccountFromPrincipal } from './fetchUtils.js';
import { getFullFromID } from './accountSearch.js';

export async function searchAllTokens(searchInput){
    let allTKNS = getAllTokenData();
    let tLen = allTKNS.length ?? 0;
    let i;
    let jobArray = [];
    for(i=0; i<tLen; i++){
        jobArray.push(getFullFromID(searchInput, allTKNS[i].ticker));
    }
    let res = await processPromises(jobArray);
    let resLen = res.length ?? 0;
    // add token shortnames
    for(i=0; i<resLen; i++){
        res[i].shortName = allTKNS[i].shortName;
        res[i].ticker = allTKNS[i].ticker;
        res[i].decimals = allTKNS[i].decimals; 
    }
    return res;
}