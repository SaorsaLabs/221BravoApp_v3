#!/bin/bash

# CROSS = "XXX/ICP"

oracleCanister="vq2tb-uiaaa-aaaak-qcqfa-cai"

# add to mibs?
# NOTE - MIB canisters should already be added/ authorised !! 
# leave as "" to ignore a specific MIB
addToMibs=true
mibICPSWAP="3hqkx-faaaa-aaaak-qcrgq-cai"
mibSONIC="3otbl-tiaaa-aaaak-qcrha-cai"
mibICDEX="qxevb-fyaaa-aaaak-qcqya-cai"

# New Swap Input
initQuote=0.239 # <== DONT FORGET THIS !!
# token 1
token1Ticker="KINIC"
token1Ledger="73mez-iiaaa-aaaaq-aaasq-cai"
token1decimals=8
# token 2
token2Ticker="ICP"
token2Ledger="ryjl3-tyaaa-aaaaa-aaaba-cai"
token2decimals=8
# swap type
swapType=1
tokenCross=${token1Ticker}'/'${token2Ticker}

# ICP SWAP
addICPSWAP=true # <== TRUE TO ADD
mkt1Canister="335nz-cyaaa-aaaag-qcdka-cai"
mkt1ReverseCross=false
mkt1UnitSize=1

# SONIC
addSONIC=true # <== TRUE TO ADD
mkt2Canister="3xwpq-ziaaa-aaaah-qcn4a-cai"
mkt2ReverseCross=false
mkt2UnitSize=1

# IC LIGHT
addICLIGHT=true # <== TRUE TO ADD
mkt3Canister="5p763-qaaaa-aaaar-qadka-cai"
mkt3ReverseCross=false
mkt3UnitSize=10000000

# -----------------------------------------
echo "[][] -------------------------------- [][]"
echo "        Starting '${tokenCross}'"
echo "[][] -------------------------------- [][]"

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

# Add ICP Swap if true
if [ "$addICPSWAP" = true ]; then
    dfx canister call ${oracleCanister} --network ic add_marketplace_to_swap '(
        "'${tokenCross}'", record {
            marketplace = variant { SONIC = null };
            canister_id = "'${mkt2Canister}'";
            active = true;
            reverse_cross = '$mkt2ReverseCross';
            unit_size =  '$mkt2UnitSize': nat64;
        } 
    )'
fi

# Add SONIC if true
if [ "$addSONIC" = true ]; then
    dfx canister call ${oracleCanister} --network ic add_marketplace_to_swap '(
        "'${tokenCross}'", record {
            marketplace =  variant { ICPSWAP = null };
            canister_id = "'${mkt1Canister}'";
            active = true;
            reverse_cross = '$mkt1ReverseCross';
            unit_size =  '$mkt1UnitSize': nat64;
        } 
    )'
fi

# Add IC LIGHT if true
if [ "$addICLIGHT" = true ]; then
    dfx canister call ${oracleCanister} --network ic add_marketplace_to_swap '(
        "'${tokenCross}'", record {
            marketplace =  variant { ICDEX = null };
            canister_id = "'${mkt3Canister}'";
            active = true;
            reverse_cross = '$mkt3ReverseCross';
            unit_size =  '$mkt3UnitSize': nat64;
        } 
    )'
fi

# ADD TO MIBS
if [ "$addToMibs" = true ]; then

    if [ "$mibICPSWAP" != "" ]; then
        dfx canister call ${oracleCanister} --network ic add_pair_to_mib_canister '("'${mibICPSWAP}'", "'${tokenCross}'")'
    fi

    if [ "$mibSONIC" != "" ]; then
        dfx canister call ${oracleCanister} --network ic add_pair_to_mib_canister '("'${mibSONIC}'", "'${tokenCross}'")'
    fi

    if [ "$mibICDEX" != "" ]; then
        dfx canister call ${oracleCanister} --network ic add_pair_to_mib_canister '("'${mibICDEX}'", "'${tokenCross}'")'
    fi

fi

