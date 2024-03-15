import { getIdentity, icActor } from "./icAgent";
import { priceOracleIDL } from "../IDL/priceOracle";
import { priceStoreMk2IDL } from "../IDL/priceStoreMk2.js";
import { priceOracleCanisterID, priceStoreMk2CanisterID } from '../constants.js';


export async function getAllQuotes(mode){
    const Frontend_ID = getIdentity();
    let priceOpt;
    if (mode == "ICP") {
        priceOpt = {'ICP': null};
    } else if (mode == "USD") {
        priceOpt = {'USD': null};
    }
    let actor = icActor(priceOracleCanisterID, priceOracleIDL, Frontend_ID);
    let res; 
    try {
        res = await actor.get_all_quotes_v1(priceOpt) 
    } catch (error) {
        res = "error"
        console.log(error);
    }
    return res;
}

export async function getTokenQuote(token, mode, decimals){
    const Frontend_ID = getIdentity();
    let priceOpt;
    if (mode == "ICP") {
        priceOpt = {'ICP': null};
    } else if (mode == "USD") {
        priceOpt = {'USD': null};
    }

    let cross = `${token}/ICP`;
    let actor = icActor(priceOracleCanisterID, priceOracleIDL, Frontend_ID);
    let res; 
    try {
        res = await actor.get_quote_v1(cross, priceOpt); 
    } catch (error) {
        res = "error"
        console.log(error);
    }

    // process results
    // let avPrice = res[0]?.average_price ?? 0.0; not required
    let mpLen = res[0]?.exchange_snapshots?.length ?? 0;
    let mpOP = [];
    let mp;
    let decPower = Math.pow(10, decimals);
    let bid, ask, spread;
    for(let i=0; i<mpLen; i++){
        mp = Object.keys(res[0].exchange_snapshots[i].exchange);
        bid = Number(res[0].exchange_snapshots[i].bid)/decPower;
        ask = Number(res[0].exchange_snapshots[i].ask)/decPower;
        spread = `${Number(res[0].exchange_snapshots[i].spread_pct).toFixed(2)} %`;
        mpOP.push({
            marketplace: mp[0], bid, ask, spread
        });
    }
    return mpOP;
}

export async function getOHLCdata(token, timeframe){
    if (!token || !timeframe) return [];
    const Frontend_ID = getIdentity();
    let actor = icActor(priceStoreMk2CanisterID, priceStoreMk2IDL, Frontend_ID);
    let res;
    let tkn = `${token}/ICP`; 
    try {
        if (timeframe == "m5") {
            res = await actor.get_m5_data(tkn, 2500);
        }
        if (timeframe == "m15") {
            res = await actor.get_m15_data(tkn, 1500);
        }
        if (timeframe == "h1") {
            res = await actor.get_h1_data(tkn, 1000);
        }
        if (timeframe == "d1") {
            res = await actor.get_d1_data(tkn, 1000);
        }
        if (timeframe == "w1") {
            res = await actor.get_w1_data(tkn, 500);
        }
    } catch (error) {
        res = "error"
        console.log(error);
    }
    return res;
}

export async function getLatestICPprice(){
    try {
        let settings = { method: "Get" };
        let host = "api.coinbase.com";
        let url = `https://${host}/v2/prices/icp-usd/buy`;
        const response = await fetch(url, settings);
        const d = await response.json();
        let ret = d.data.amount;
        return ret;
    } catch (error) {
        return 0;
    }
}

export async function getICPOhlcData(barSizeSecs, barsToFetch) {
    const granularity = barSizeSecs; 
    const bars = barsToFetch; 
    const end = new Date(); 
    const start = new Date(end.getTime() - (bars * granularity * 1000)); 
    const productID = 'ICP-USD'; 
    const url = `https://api.pro.coinbase.com/products/${productID}/candles?start=${start.toISOString()}&end=${end.toISOString()}&granularity=${granularity}`;
    try {
      const response = await fetch(url);
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const data = await response.json();
      // data   
      // [1705396500, 12.78, 12.8, 12.794, 12.78, 515.7754]
      // time, low, high, open, close 
      let retData = [];
      // re-format data
      let dataLen = data.length ?? 0;
      for(let i=0; i<dataLen; i++){
        retData.push([data[i][3], data[i][2], data[i][1], data[i][4], data[i][0]]); // open, high, low, close, time
      }
      return retData.reverse(); // oldest to newest.
    } catch (error) {
      console.error('Error fetching data:', error);
    }
  }
  


