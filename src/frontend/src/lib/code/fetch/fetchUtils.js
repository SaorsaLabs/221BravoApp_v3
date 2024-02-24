import { getIdentity, icActor } from "./icAgent.js";
import { backendCanisterIDL } from "../IDL/backend.js";
import { backendCanisterID } from "../constants.js";
import { superIndexIDL } from "../IDL/superIndex.js";


export async function getDefaultAccountFromPrincipal(principal){
    const Frontend_ID = getIdentity();
    let actor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
    let res; 
    try {
        res = await actor.get_single_account(principal, 0);
    } catch (error) {
        res = "error"
    }
    return res; 
}

export async function testFetch(){
    const Frontend_ID = getIdentity();
    let actor = icActor("ly2wh-viaaa-aaaak-qckra-cai", superIndexIDL, Frontend_ID);
    let res = await actor.get_cycles_balance();
    return res;  
}