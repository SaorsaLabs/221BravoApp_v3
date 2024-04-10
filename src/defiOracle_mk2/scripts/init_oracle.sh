#!/bin/bash

oracleCanister="vq2tb-uiaaa-aaaak-qcqfa-cai"

# stop alert canister and OHLC store
# Re-install all canisters
# INIT MIBS
# INIT SWAPS

# add authorised on oracle CPC
dfx canister call ${oracleCanister} --network ic add_authorised '("ztewi-mzfkq-w57f2-xtl6i-kacap-n2gg6-dxyzu-p3oql-aikxf-rsivy-aqe")'
dfx canister call ${oracleCanister} --network ic add_authorised '("2vxsx-fae")'
dfx canister call ${oracleCanister} --network ic add_authorised '("qzqml-yiaaa-aaaak-qcp4q-cai")'
dfx canister call ${oracleCanister} --network ic add_authorised '("rjp2d-wiaaa-aaaak-qdbna-cai")'

# add mibs
dfx canister call ${oracleCanister} --network ic add_mib_canister '("qxevb-fyaaa-aaaak-qcqya-cai", "IC LIGHT 1", variant {"ICDEX"} )'
dfx canister call ${oracleCanister} --network ic add_mib_canister '("3hqkx-faaaa-aaaak-qcrgq-cai", "ICP SWAP 1", variant {"ICPSWAP"} )'
dfx canister call ${oracleCanister} --network ic add_mib_canister '("3otbl-tiaaa-aaaak-qcrha-cai", "SONIC", variant {"SONIC"} )'
dfx canister call ${oracleCanister} --network ic add_mib_canister '("j7dmq-lqaaa-aaaak-qc6iq-cai", "ICP SWAP 2", variant {"ICPSWAP"} )'
dfx canister call ${oracleCanister} --network ic add_mib_canister '("jrbby-qaaaa-aaaak-qc6jq-cai", "SONIC 2", variant {"SONIC"} )'
dfx canister call ${oracleCanister} --network ic add_mib_canister '("a77je-3iaaa-aaaak-qcyfq-cai", "IC LIGHT 2", variant {"ICDEX"} )'

