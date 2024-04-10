ohlcCanister="qzqml-yiaaa-aaaak-qcp4q-cai"
tokenCross="ICS/ICP"

dfx canister call ${ohlcCanister} --ic add_cross '("'$tokenCross'")'
