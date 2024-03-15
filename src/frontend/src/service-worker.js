import { backendCanisterID, icpLedgerCanister } from "./lib/code/constants.js";
import { getIdentity, icActor } from "./lib/code/fetch/icAgent.js";
import { backendCanisterIDL } from "./lib/code/IDL/backend.js";
import { processPromises, getTokenData, getAllTokenData } from "./lib/code/utils.js";
import { fetchStatsData, fetchICPSupply } from "./lib/code/fetch/statsData.js";
import { getOHLCdata, getICPOhlcData, getTokenQuote } from './lib/code/fetch/priceData.js';

onmessage = async (event) => {
    let workerResult;
    switch (event.data?.type) {
		case 'fetch-stats-home':
            let SH = await fetchStatsHome();
            workerResult = {'result': SH};
            postMessage(workerResult);
            return;
        case 'fetch-stats-head':
            let SHD = await fetchStatsHead();
            workerResult = {'result': SHD};
            postMessage(workerResult);
            return;
        case 'fetch-stats-overview':
            let tkn = event.data?.data.token;
            let SO = await fetchStatsOverview(tkn);
            workerResult = {'result': SO};
            postMessage(workerResult);
            return;
        case 'fetch-stats-overview-charts':
            let tkn2 = event.data?.data.token;
            let tfm = event.data?.data.timeframe;
            let SOC = await fetchStatsOverviewChart(tkn2, tfm);
            workerResult = {'chartResult': SOC};
           postMessage(workerResult);
            return;
        case 'fetch-stats-overview-markets':
            let tkn3 = event.data?.data.token;
            let mde = event.data?.data.mode;
            let MKT = await fetchMarketPrices(tkn3, mde);
            workerResult = {'marketResult': MKT}; 
            postMessage(workerResult);
            return;
	}
};

async function fetchStatsHead(){
    let futuresAR = [];
    futuresAR[0] = getICPOhlcData(300, 1);
    let allTkns = getAllTokenData();
    let allTknsLen = allTkns?.length ?? 0;
    let futureComplete = await processPromises(futuresAR);
    let resLen1 = futureComplete[0].length ?? 0;
    let price = 0;
    if(resLen1 == 1){
        price = futureComplete[0][0][3]
    }
    // Table Data
    let ret = {
        icpPrice: price,
        numTokens: allTknsLen,
    };

    return ret; 
}

async function fetchStatsHome(){
    const Frontend_ID = getIdentity();
    let actor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
    let futuresAR = [];
    futuresAR[0] = actor.get_top_tokens_data();
    futuresAR[1] = fetchICPSupply();
    futuresAR[2] = getICPOhlcData(900, 96);

    let futureComplete = await processPromises(futuresAR);
    let topTokens;
    let resLen1 = futureComplete[2].length ?? 0;

    if(resLen1 == 96){
        // Add ICP data 
        let cng7 = 0;
        let cng24 = ((futureComplete[2][95][3] - futureComplete[2][0][3]) / futureComplete[2][0][3]) *100;
        let mcap = (Number(futureComplete[1])/ Math.pow(10, 8)) * futureComplete[2][95][3];
        let sprk = [];
        // sparkline (reversed)
        for(let i = resLen1-1; i>=0; i--){
            sprk.push(futureComplete[2][i][3])
        }
        let icpData = [{
            change7d: cng7, 
            change24: cng24, 
            cross: "ICP/USD", 
            decimals: 8,
            price: futureComplete[2][95][3],
            ledger: icpLedgerCanister,
            mcap: mcap,
            sparkline: sprk
        }];
        topTokens = [...icpData, ...futureComplete[0]];
    } else {
        topTokens = futureComplete[0];
    }

    // Table Data
    let ret = {
        topTokens: topTokens
    };
    return ret;
}

async function fetchStatsOverview(token){
    let data = await fetchStatsData(token);
    return data;
}

async function fetchStatsOverviewChart(token, timeframe){
    let tokenData = getTokenData(token);
    if (tokenData == "Could not find a matching token") return {ohlc: [], tokenData: 4};
    if(token != "ICP"){
        let data = await getOHLCdata(token, timeframe);
        return {ohlc: data, chartDecimals: tokenData.chartDecimals};
    } else {
        let data = await getICPOhlcData(900, 192);
        let dLen = data?.length ?? 0;
        let ret = [];
        for(let i=0; i<dLen; i++){
            ret.push({
                close: {cross_price: 0, usd_price: data[i][3]},
                close_time: 0,
                open_time: data[i][4],
                open: {cross_price: 0, usd_price: data[i][0]},
                high: {cross_price: 0, usd_price: data[i][1]},
                low: {cross_price: 0, usd_price: data[i][2]},
                volume: 0
        });
        }
        return {ohlc: ret, chartDecimals: 8};
    }
}

async function fetchMarketPrices(token, mode){
    let tokenData = getTokenData(token);
    let tknQuote = await getTokenQuote(token, mode, tokenData.decimals);
    // format 
    let len = tknQuote?.length ?? 0;
    let OP = [];
    for(let i=0; i<len; i++){
        OP.push({
            marketplace: tknQuote[i].marketplace,
            bid: Number(tknQuote[i].bid).toFixed(tokenData.chartDecimals),
            ask: Number(tknQuote[i].ask).toFixed(tokenData.chartDecimals),
            spread: tknQuote[i].spread
        });
    }
    return OP;
}