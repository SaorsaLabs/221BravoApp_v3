import { getIdentity, icActor } from "./icAgent";
import { statsCanisterIDL } from "../IDL/statsCanister.js";
import { backendCanisterIDL } from "../IDL/backend.js";
import { backendCanisterID, icpLedgerCanister, icpStatsCanister } from "../constants.js";
import { getTokenData, processPromises, parseTicker } from "../utils.js";
import { icpLedgerIDL } from "../IDL/icpLedger.js";
import { getICPOhlcData } from './priceData.js';


export async function fetchStatsData(token){

    const Frontend_ID = getIdentity();
    let tokenData = getTokenData(token);
    if (tokenData == "Could not find a matching token") return {};
    let tokenCross = `${token}/ICP`;
    
        let statsActor = icActor(tokenData.stats221B, statsCanisterIDL, Frontend_ID);
        let backendActor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
        let futuresAR = [];
        let priceChangeData;

        if (token != "ICP") {
            futuresAR[0] = backendActor.get_top_tokens_data(); // not ICP!
            futuresAR[1] = statsActor.get_total_holders();
            futuresAR[2] = statsActor.get_daily_stats();
            futuresAR[3] = statsActor.get_hourly_stats();
            futuresAR[4] = backendActor.get_top_holders(tokenCross); // not ICP!
        } else {
            // ICP DATA
            futuresAR[1] = statsActor.get_total_holders();
            futuresAR[2] = statsActor.get_daily_stats();
            futuresAR[3] = statsActor.get_hourly_stats();
           // futuresAR[4] = icpActor.get_top_account_holders(100);
            futuresAR[5] = fetchICPSupply();
            futuresAR[6] = getICPOhlcData(900, 96);
            futuresAR[7] = getICPOhlcData(86400, 7);
        }

        let futureComplete = await processPromises(futuresAR);
        // match top token data
        if (token == "ICP"){
            let c7 = ((Number(futureComplete[7][6][3]) - Number(futureComplete[7][0][3])) / Number(futureComplete[7][0][3])) * 100;
            let c24 = ((Number(futureComplete[6][95][3]) - Number(futureComplete[6][0][3])) / Number(futureComplete[6][0][3])) * 100;
            let sup = Number(futureComplete[5]) / Math.pow(10, 8);
            let mc = sup * Number(futureComplete[6][95][3]);
            priceChangeData = {
                change7d: c7,
                change24: c24,
                decimals: 8,
                mcap: mc,
                supply: sup
            }
            return {
                tokenData,
                priceChangeData,
                totalHolders: futureComplete[1],
                dailyData: futureComplete[2],
                hourlyData: futureComplete[3],
               // topHolders: futureComplete[4][0]
            };
        }
        if (token != "ICP") {
            let ttLen = futureComplete[0].length ?? 0;
            let dref;
            for (let i = 0; i< ttLen; i++){
                dref = parseTicker(futureComplete[0][i].cross);
                if (dref.base == token){
                    priceChangeData = futureComplete[0][i];
                }
            }
            return {
                tokenData,
                priceChangeData,
                totalHolders: futureComplete[1],
                dailyData: futureComplete[2],
                hourlyData: futureComplete[3],
                topHolders: futureComplete[4][0]
            };
        }
}

export async function fetchICPSupply(){
    const Frontend_ID = getIdentity();
    let actor = icActor(icpLedgerCanister, icpLedgerIDL, Frontend_ID);
    let res = await actor.icrc1_total_supply();
    return res;
}