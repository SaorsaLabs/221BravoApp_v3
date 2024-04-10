#!/bin/bash
#oracleCanister="52jup-gaaaa-aaaak-qcwpa-cai"
oracleCanister="vq2tb-uiaaa-aaaak-qcqfa-cai"

 dfx canister call oracle_cpc_mk2 --network ic update_pair_status '("ICS/ICP", null, true: bool)'


# dfx canister call ${oracleCanister} --network ic update_pair_status '("CKBTC/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("CHAT/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("OGY/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("SNEED/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("SNS1/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("SONIC/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("MOD/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("BOOM/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("GHOST/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("HOT/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("CAT/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("KINIC/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("NUA/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("ICX/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("CKETH/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("TRAX/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("GLDGOV/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("NTN/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("QUERIO/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("MOTOKO/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("ICL/ICP", null, true: bool)'
# dfx canister call ${oracleCanister} --network ic update_pair_status '("ELNA/ICP", null, true: bool)'