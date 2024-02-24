import { getIdentity, icActor } from "./icAgent";
import { superIndexIDL } from '../IDL/superIndex.js';
import { DEFAULT_SUBACCOUNT } from "../constants.js";
import { combinePrincipalSubAccount, getTokenData, parsePrincipalSubAccountString } from "../utils.js";
import { getDefaultAccountFromPrincipal } from './fetchUtils.js';
import { addNamedAccounts } from './namedAccounts.js';

export async function getFullFromID(id, token){
    if(!id || !token) return {data: null, resolved: null, decimals: 0};
    let tokenData = getTokenData(token);
    if (tokenData == "Could not find a matching token") return {data: null, resolved: null, decimals: 0};
    const Frontend_ID = getIdentity();
    let actor = icActor(tokenData.index221B, superIndexIDL, Frontend_ID);

    if (tokenData.standard.includes("icrc")){
        // has sub account
        if(id.includes("-") && id.includes(".")){
            let res = await actor.get_full_from_id(id);
            if(res.length > 0){
                let add = await addNamedAccounts(res[0].blocks);
                res[0].blocks = add;
            }
            return {data: res, resolved: null, decimals: tokenData.decimals, searchedAC: id};
        }
        // principal only
        else if(id.includes("-")){
            let defSA = DEFAULT_SUBACCOUNT;
            let combined = combinePrincipalSubAccount(id, defSA);
            let res = await actor.get_full_from_id(combined);
            if(res.length > 0){
                let add = await addNamedAccounts(res[0].blocks);
                res[0].blocks = add;
            }
            return {data: res, resolved: combined, decimals: tokenData.decimals, searchedAC: combined};
        }
        else {
            // unsupported 
            return {data: [], resolved: null, decimals: null};
        }
    }
    else if (tokenData.standard == "icp-og"){
        if(id.includes("-") && !id.includes(".")){
            // principal
            let account = await getDefaultAccountFromPrincipal(id);
            let res = await actor.get_full_from_id(account);
            if(res.length > 0){
                let add = await addNamedAccounts(res[0].blocks);
                res[0].blocks = add;
            }
            return {data: res, resolved: account, decimals: tokenData.decimals, searchedAC: account};
        } 
        else if (id.includes(".")) {
            // if default icrc - ok 
            if (id.includes(DEFAULT_SUBACCOUNT)){
                let parsed = parsePrincipalSubAccountString(id);
                // principal
                let account = await getDefaultAccountFromPrincipal(parsed.principal);
                let res = await actor.get_full_from_id(account);
                if(res.length > 0){
                    let add = await addNamedAccounts(res[0].blocks);
                    res[0].blocks = add;
                }
                return {data: res, resolved: account, decimals: tokenData.decimals, searchedAC: account};
            }
            // else unsupported icrc account
            return {data: [], resolved: null, decimals: null};
        } else {
            // ICP OG style account
            let res = await actor.get_full_from_id(id);
            if(res.length > 0){
                let add = await addNamedAccounts(res[0].blocks);
                res[0].blocks = add;
            }
            return {data: res, resolved: null, decimals: tokenData.decimals, searchedAC: id};
        }
    }
}

