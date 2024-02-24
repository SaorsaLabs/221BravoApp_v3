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

// export async function searchAllTokens1(searchInput){
//     const Frontend_ID = getIdentity();
//     let allTKNS = getAllTokenData();
//     let tLen = allTKNS.length ?? 0;
//     let i;
//     let returnData = {};
//     let searchAR = [];
//     let tokenList = [];
//     if (searchInput == "" || searchInput == null || searchInput == undefined) return returnData;
//     // sort input
//     if (searchInput.includes(".") && searchInput.includes("-")){
//         // icrc account
//         let parsedAc = parsePrincipalSubAccountString(searchInput);
//         // if default sub account include icp-og searches
//         if (parsedAc.subaccount == DEFAULT_SUBACCOUNT && parsedAc.principal != '') {
//             let primaryAc = await getDefaultAccountFromPrincipal(parsedAc.principal);
//             for(i=0; i<tLen; i++){
//                 if(allTKNS[i].standard.includes("icp-og")){
//                     if (primaryAc != "error") {
//                         searchAR.push({canister: allTKNS[i].index221B, input: primaryAc});
//                         tokenList.push({shortName: allTKNS[i].shortName, ticker: allTKNS[i].ticker, decimals: allTKNS[i].decimals});
//                     }
//                 }
//             }
//         }
//         // add all icrc accounts
//         for(i=0; i<tLen; i++){
//             if(allTKNS[i].standard.includes("icrc")){
//                 searchAR.push({canister: allTKNS[i].index221B, input: searchInput});
//                 tokenList.push({shortName: allTKNS[i].shortName, ticker: allTKNS[i].ticker, decimals: allTKNS[i].decimals});
//             }
//         }
//     } else if (searchInput.includes("-")) {
//         // principal only
//         let primaryAc = await getDefaultAccountFromPrincipal(searchInput);
//         let icrcAc = combinePrincipalSubAccount(searchInput, DEFAULT_SUBACCOUNT);
            
//         for(i=0; i<tLen; i++){
//             if(allTKNS[i].standard.includes("icp-og")){
//                 if (primaryAc != "error") {
//                     searchAR.push({canister: allTKNS[i].index221B, input: primaryAc});
//                     tokenList.push({shortName: allTKNS[i].shortName, ticker: allTKNS[i].ticker, decimals: allTKNS[i].decimals});
//                 }
//             }
//             if(allTKNS[i].standard.includes("icrc")){
//                 searchAR.push({canister: allTKNS[i].index221B, input: icrcAc});
//                 tokenList.push({shortName: allTKNS[i].shortName, ticker: allTKNS[i].ticker, decimals: allTKNS[i].decimals});
//             }
//         }
//     } else {
//         // ICP OG format
//         for(i=0; i<tLen; i++){
//             if(allTKNS[i].standard.includes("icp-og")){
//                 searchAR.push({canister: allTKNS[i].index221B, input: searchInput});
//                 tokenList.push({shortName: allTKNS[i].shortName, ticker: allTKNS[i].ticker, decimals: allTKNS[i].decimals});
//             }
//         }
//     }

//     // Build futures array
//     let jobArray = [];
//     let searchArLen = searchAR.length ?? 0;
//     for(i=0; i<searchArLen; i++){
//         let actor = icActor(searchAR[i].canister, superIndexIDL, Frontend_ID);
//         jobArray.push(actor.get_full_from_id(searchAR[i].input))
//     }
//     // process futures
//     let res = await processPromises(jobArray);
//     let resLen = res.length ?? 0;
//     // add token shortnames
//     for(i=0; i<resLen; i++){
//         if (res[i].length > 0) {
//             res[i][0].shortName = tokenList[i].shortName;
//             res[i][0].ticker = tokenList[i].ticker;
//             res[i][0].decimals = tokenList[i].decimals;
//         }
//     }
//     // filter 0 len
//     let finalOP = [];
//     for(i=0; i<resLen; i++){
//         if (res[i].length > 0) finalOP.push(res[i][0]);
//     }
//     returnData = {data: finalOP};
//     return returnData;
// }
