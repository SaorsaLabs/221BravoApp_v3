import { getIdentity, icActor } from "./icAgent";
import { backendCanisterIDL } from '../IDL/backend.js';
import { backendCanisterID } from '../constants.js';
import { getUniqueValues } from '../utils.js';
import { authStore } from "../../stores/authStore.js";

export async function getUserNamedAccounts(owner, checkAR){
    const Frontend_ID = getIdentity();
    let actor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
    let res = [];
    res = await actor.get_user_named_accounts(owner, checkAR); 
    return res;
}

export async function getPublicNamedAccounts(checkAR){
    const Frontend_ID = getIdentity();
    let actor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
    let res = [];
    res = await actor.get_public_named_accounts(checkAR); 
    return res;
}

// NOTE data = Vec<ProcessedTx> in rust. 
export async function addNamedAccounts(blockData){
    // get unique accounts
    let i;
    let dataLen = blockData?.length ?? 0;
    if (!blockData || dataLen == 0 ) return blockData;
    let allAccounts = [];
    for (i=0; i<dataLen; i++){
        allAccounts.push(blockData[i].from_account);
        allAccounts.push(blockData[i].to_account);
    }
    let unique = getUniqueValues(allAccounts);

    // lookup
    let globalAccounts = await getPublicNamedAccounts(unique);
    let userAccounts;
    let ls = authStore.read();
    if (ls.data.loggedIn == true) {
        userAccounts = await getUserNamedAccounts(ls.data.user, unique);
        // combine User
        let k;
        let uALen = userAccounts[0]?.length ?? 0; 
        for (i=0; i<uALen; i++){
            for (k=0; k<dataLen; k++){
                if(userAccounts[0][i][0] == blockData[k].from_account){
                    blockData[k].fromUserName = userAccounts[0][i][1];
                }
                // to ac global
                if(userAccounts[0][i][0] == blockData[k].to_account){
                    blockData[k].toUserName = userAccounts[0][i][1];
                }                
            }
        }
    }

    // combine global
    let m;
    let gALen = globalAccounts[0]?.length ?? 0;
    for (i=0; i<gALen; i++){
        for (m=0; m<dataLen; m++){
            // from ac global
            if(globalAccounts[0][i][0] == blockData[m].from_account){
                blockData[m].fromGlobalName = globalAccounts[0][i][1];
            }
            // to ac global
            if(globalAccounts[0][i][0] == blockData[m].to_account){
                blockData[m].toGlobalName = globalAccounts[0][i][1];
            }
        }
    }
    // return
    return blockData;
}
