# 221Bravo Defi Oracle

The 221Bravo Price Oracle fetches real time prices, on-chain, from the largest DEXs in the ICP Ecosystem. The oracle implements a two step process for getting prices that more closely reflect the true buy/sell prices for each token – Firstly, the oracle fetches the price data for a fixed batch size (eg a $20 trade). This prevents distortions in the price caused by some tokens, like ckBTC, having a high dollar value while other tokens, like GHOST, have a very small dollar value per token. 

Secondly, the price oracle fetches both the buy price (bid) and sell price (ask) for each token. This is then averaged to get a single quote. The underlying bid/ ask and spread are also provided for each DEX users who want more detail. 

### Canister Relationships
There are 3 main parts to the Price Oracle family of canisters. The main canister is the ‘Central Processing Canister’ (CPC) which co-ordinates a group of ‘Market Interface Bots’ (MIB) who make queries to their assigned marketplace and then return these to the CPC. 
The CPC and MIB canister only provide live prices. 

In order to provide price history, a 3rd canister was crated – The OHLC (Open, high, low, close) Store periodically takes snapshots of price data from the CPC and stores these in buckets of 5 minute, 15 minute 1 hour, 1 day and 1 week bars. 

## Getting Live Quotes
There are two public methods on the CPC canister which provide a simple way to get price data on the tracked tokens. 
For a specific token, users can call the ‘get_quote_v1’ method. This method takes two arguments. The first argument is the token cross as text (example “CHAT/ICP”) and the 2nd argument is what currency the prices should be shown in. There are 3 variants for quote currency – ICP, USD and XDR. 

```bash
dfx canister call vq2tb-uiaaa-aaaak-qcqfa-cai --network ic get_quote_v1 '("CHAT/ICP", variant {"USD"})'
```

For all tokens, users can call ‘get_all_quotes_v1’ which will return price data for all tracked tokens. 

```bash
dfx canister call vq2tb-uiaaa-aaaak-qcqfa-cai --network get_all_quotes_v1 '(variant {"USD"})'
```

## Getting Historic Quotes
Users wishing to get more than the current quote can query the OHLC Store to get historic data including the currently forming bar. There are a number of methods you can use depending on what data you are needing. Historic Prices are given in ICP
You can get the available token crosses by querying ‘get_all_crosses’ 

```bash
dfx canister call lo4kk-kyaaa-aaaak-qcska-cai --network ic  get_all_crosses
```

For all price data for a certain token cross (example below is for CHAT/ICP)

```bash
dfx canister call lo4kk-kyaaa-aaaak-qcska-cai --network ic get_all_data '("CHAT/ICP")' 
```

You can get specific timeframes by calling one of the following methods 
* get_m5_data
* get_m15_data
* get_h1_data
* get_d1_data
* get_w1_data

Each of these methods takes two arguments. The first being the token cross as text and the second the maximum number of bars to fetch.

```bash
dfx canister call lo4kk-kyaaa-aaaak-qcska-cai --network ic get_m5_data '("CHAT/ICP", 50 :nat64)' 
```
