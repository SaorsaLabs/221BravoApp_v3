import { getIdentity, icActor } from "./icAgent";
import { superIndexIDL } from '../IDL/superIndex.js';
import { getTokenData, getAllTokenData, processPromises } from '../utils.js';
import { addNamedAccounts } from '../fetch/namedAccounts.js';

export async function fetchLatestBlocks(token, blockLen){
    const Frontend_ID = getIdentity();
    let tokenData = getTokenData(token);
    if (tokenData == "Could not find a matching token") return {blocks: [], tokenData: {}};

    let actor = icActor(tokenData.index221B, superIndexIDL, Frontend_ID);
    let blocks = await actor.get_latest_transactions(blockLen);
    let blocksAdj = await addNamedAccounts(blocks);
    return {blocks: blocksAdj, tokenData: tokenData};
}

export async function fetchCustomBlocks(token, startBlock, endBlock){
    const Frontend_ID = getIdentity();
    let tokenData = getTokenData(token);
    if (tokenData == "Could not find a matching token") return {blocks: [], tokenData: {}};

    let blocksRequired = [];
    let count = startBlock;
    for(let i=startBlock; i<=endBlock; i++){
        blocksRequired.push(Number(count));
        count++;
    }
    blocksRequired.reverse();
    let actor = icActor(tokenData.index221B, superIndexIDL, Frontend_ID);
    let blocks = await actor.get_multiple_tx(blocksRequired);
    let blocksAdj = await addNamedAccounts(blocks);
    return {blocks: blocksAdj, tokenData: tokenData};
}

export async function fetchSuperBlox(){
    const Frontend_ID = getIdentity();
    let tokenData = getAllTokenData();
    let tDLen = tokenData?.length ?? 0;
    let i;
    let blocks;
    let allBlocks = [];
    let tokenDataAR = [];
    let futures = [];
    let fCount = 0;
    let firstICPTime;
    // Fetch data
    for(i=0; i<tDLen; i++){
        if(tokenData[i].ticker == "ICP"){
            let actor = icActor(tokenData[i].index221B, superIndexIDL, Frontend_ID);
            futures[fCount] = actor.get_latest_transactions(10000);
            tokenDataAR.push(tokenData[i]);
        } else {
            let actor = icActor(tokenData[i].index221B, superIndexIDL, Frontend_ID);
            futures[fCount] = actor.get_latest_transactions(1000);
            tokenDataAR.push(tokenData[i]);
        }
        fCount++;
    }

    //process data
    let resolvedPromises = await processPromises(futures);
    let rPLen = resolvedPromises?.length ?? 0;
    for(i=0; i<rPLen; i++){
        let promLen = resolvedPromises[i]?.length ?? 0;
        for(let k=0; k<promLen; k++){
            resolvedPromises[i][k].token = tokenDataAR[i].ticker;
            resolvedPromises[i][k].decimals = tokenDataAR[i].decimals;
            resolvedPromises[i][k].tx_time = Number(resolvedPromises[i][k].tx_time);
            allBlocks.push(resolvedPromises[i][k]);
        }
        if(tokenDataAR[i].ticker == "ICP") firstICPTime = resolvedPromises[i][promLen-1].tx_time;
    }
    let retBlox = [];
    allBlocks.sort((a, b) => b.tx_time - a.tx_time); 
    let aBLen = allBlocks.length ?? 0;
    for(i=0;i<aBLen; i++){
        retBlox.push(allBlocks[i]);
        if(allBlocks[i].tx_time <= firstICPTime) break;
    }
    let bloxIncludingNames = await addNamedAccounts(retBlox);
    return bloxIncludingNames;
}
