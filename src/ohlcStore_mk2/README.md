# 221Bravo - OHLC Store (Open, High, Low, Close)
The OHLC Store canister was created to store token price history. The OHLC does this by periodically fetching quotes from the [Defi Oracle Canister](https://github.com/SaorsaLabs/221BravoApp_v3/tree/main/src/defiOracle_mk2) and using these quotes to form 'bars/ candlestick' data on several timeframes.  

## deploy
```bash
dfx deploy ohlc_store_mk2 --network ic --argument $(dfx identity get-principal)
```

## Getting Historic Quotes
Users wishing to get more than the current quote can query the OHLC Store to get historic data including the currently forming bar. There are a number of methods you can use depending on what data you are needing. Historic Prices are given as the cross price (eg CKBTC/ICP) and USD price (eg CKBTC/USD). 

You can get the available token crosses by querying ‘get_all_crosses’ 

```bash
dfx canister call YOUR_OHLC_CANISTER_ID --network ic  get_all_crosses
```

For all price data for a certain token cross (example below is for CHAT/ICP)

```bash
dfx canister call YOUR_OHLC_CANISTER_ID --network ic get_all_data '("CHAT/ICP")' 
```

You can get specific timeframes by calling one of the following methods 
* get_m5_data
* get_m15_data
* get_h1_data
* get_d1_data
* get_w1_data

Each of these methods takes two arguments. The first being the token cross as text and the second the maximum number of bars to fetch.

```bash
dfx canister call YOUR_OHLC_CANISTER_ID --network ic get_m5_data '("CHAT/ICP", 50 :nat64)' 
```
### Adding new crosses
Once you have setup a new cross on your defi oracle [See Here](https://github.com/SaorsaLabs/221BravoApp_v3/tree/main/src/defiOracle_mk2/oracle_cpc_mk2) you can then add that cross to the OHLC Canister. NOTE - Any new cross will start from 00:00 UTC to ensure that bars align with other crosses. For example - if you add the token at 18:00 UTC the OHLC canister will not save any fetched quotes for the first six hours (until 00:00 UTC)  

```bash
# EG adding CHAT/ICP to the OHLC Store. Note - this cross should match the cross on the Oracle CPC exactly

dfx canister call YOUR_OHLC_CANISTER_ID --network ic add_cross '("CHAT/ICP")' 

# Removing the cross from the OHLC Store. WARNING - This will wipe all data for the cross! 

dfx canister call YOUR_OHLC_CANISTER_ID --network ic remove_cross '("CHAT/ICP")' 
```

### Timers
The OHLC Canister designed to automatically fetch prices every X number of seconds. We would recommend  that this is set to 60 seconds as it balances the requirement for accurate data with the cost of update call to fetch the data. The timer APIs can be called as follows:

```bash
# start the timer with an interval of 60 seconds

dfx canister call ${oracleCanister} --network start_quotes_timer '(60: nat64, "CANISTER_ID_OF_YOUR_CPC_CANISTER")'

# stop the timer 

dfx canister call ${oracleCanister} --network stop_all_timers
```