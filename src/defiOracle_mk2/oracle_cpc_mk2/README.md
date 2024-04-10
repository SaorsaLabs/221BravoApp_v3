# 221Bravo Defi Oracle - Central Processing Canister
The Central Processing Canister acts as the central hub of the Defi Price Oracle. Below are the main API methods for adding and managing pairs. 

### Input settings/ Canister setup
Quote Size - To ensure that quoted prices reflect the liquidity of a swap pair, our defi oracle fetches prices based on a set trade value. During the init of the canister this trade value is set to $10. This trade value can be changed by using the following method. Note variants are ICP, USD or XDR :

```bash
# replace ORACLE_CANISTER_ID with the principal id of your oracle canister
dfx canister call ORACLE_CANISTER_ID --network ic set_quote_trade_size '(20: float64, variant {"USD"})'
```

Link MIB Canisters to CPC - At least one MIB canister should be deployed for each marketplace that you want to 'fetch' quotes from. More than one MIB can be used for the same marketplace - this can be useful if you are fetching a large number of quotes. For example if you want to get quotes for 40 different pairs, you can have the one MIB canister fetch the first 20 quotes and another MIB fetch then next 20 quotes. 

Currently there are 3 ICP Marketplaces that the MIB canisters can interface with - Sonic, ICP Swap and IC Dex (ICLight.io) 

```bash
# Available variants variant {"ICDEX"} variant {"SONIC"} variant {"ICPSWAP"}
# replace ORACLE_CANISTER_ID with the principal id of your oracle canister
dfx canister call ORACLE_CANISTER_ID --network ic add_mib_canister '("your-mib-canister-id", "Name-your-mib-here", variant {"ICDEX"} )'
```

### Adding Token Pairs
Note - The current version of price oracle only supports ICP token pairs, either XXX/ICP or ICP/XXX.

Adding token pairs to the price oracle is a four step process: 

1. Add the token to the Price Oracle (CPC). This is done via the 'add_swap_to_oracle' method on the CPC canister.

```bash
# Example using CKBTC/ICP
oracleCanister="your-canister-id-here"
initQuote=4101.25 # The oracle needs an init price to start the price fetching process. 
# token 1
token1Ticker="CKBTC"
token1Ledger="mxzaz-hqaaa-aaaar-qaada-cai"
token1decimals=8
# token 2
token2Ticker="ICP"
token2Ledger="ryjl3-tyaaa-aaaaa-aaaba-cai"
token2decimals=8
# swap type
swapType=1 # At the moment this is always 1. Future non-ICP crosses etc will add other swap types. 
tokenCross=${token1Ticker}'/'${token2Ticker}

# Method call
dfx canister call ${oracleCanister} --network ic add_swap_to_oracle '(
    record {
    swap_id = "'${tokenCross}'";
    token0 =  record { 
        ticker = "'${token1Ticker}'";
        ledger = "'${token1Ledger}'";
        decimals = '$token1decimals': nat32;
    };
    token1 = record { 
        ticker = "'${token2Ticker}'";
        ledger = "'${token2Ledger}'";
        decimals = '$token2decimals': nat32;
    };
    swap_type = '$swapType': nat8;
    init_quote = '$initQuote': float64;
    }
)'
```

2. Add marketplaces to the swap record. This could be any or all of the compatible marketplaces - SONIC, ICP Swap, IC Dex. The reverse cross input allows the oracle to deal with marketplaces with different base/ quote pairs. For example CKBTC/ICP (not reversed) or ICP/CKBTC (reversed = true). The unit size and details of which token is the 'base' can be found by looking at the associated marketplace swap canister. EG - IC DEX canister for CKBTC/ICP can be seen [HERE](https://dashboard.internetcomputer.org/canister/5u2c6-kyaaa-aaaar-qadiq-cai) (call method - 'info')

```bash
mkt1Canister="xmiu5-jqaaa-aaaag-qbz7q-cai"
mkt1ReverseCross=false # XXX/ICP = false, ICP/XXX = true
mkt1UnitSize=1 # this is only used on IC DEX, for Sonic and ICP Swap this value will not be counted. 
oracleCanister="your-canister-id-here"

dfx canister call ${oracleCanister} --network ic add_marketplace_to_swap '(
    "'${tokenCross}'", record {
        marketplace = variant { SONIC = null };
        canister_id = "'${mkt2Canister}'";
        active = true;
        reverse_cross = '$mkt2ReverseCross';
        unit_size =  '$mkt2UnitSize': nat64;
    } 
)'
```

3. Send the token and marketplace record to the your MIB canisters. For example, if you have added SONIC and ICP Swap as marketplaces on the token record, then you will need to send the token record to your SONIC and ICP Swap MIBS. 

```bash
oracleCanister="your-canister-id-here"
mibCanisterID="your-mib-canister-id"
tokenCross="CKBTC/ICP"
 dfx canister call ${oracleCanister} --network ic add_pair_to_mib_canister '("'${mibCanisterID}'", "'${tokenCross}'")'
```

4. At this stage you have added the token and marketplace record to the CPC and MIB canisters - however all new tokens are set as not-active. This allows you to check the setup before going live. Once you are happy all details are correct, you need to change the status of the token cross. 

```bash
oracleCanister="your-canister-id-here"
tokenCross="CKBTC/ICP"
isActive=true

 dfx canister call ${oracleCanister} --network ic update_pair_status '("'${tokenCross}'", null, '${isActive}': bool)'
```

### Timers
The Defi Oracle is designed to automatically fetch prices every X number of seconds. To do this the CPC canister smart contract has timer functions which are controlled by the following timer methods. The MIBS do not need timers as they are called by the CPC canister. 

```bash
# start the timer - CPC canister with a main quotes timer of 60 seconds and 900 seconds for Stable Quotes fetching. 
dfx canister call ${oracleCanister} --network start_quotes_timer '(60: nat64, 900: nat64)'
```

### Price History
The price oracle canister only provides 'live' prices. If you have setup a [OHLC Store Canister](https://github.com/SaorsaLabs/221BravoApp_v3/tree/main/src/ohlcStore_mk2) you should add your newly added price cross to the OHLC store. 

```bash
ohlcCanister="your-ohlc-canister-id"
tokenCross="XXX/ICP" # your new token cross

dfx canister call ${ohlcCanister} --ic add_cross '("'$tokenCross'")'
```