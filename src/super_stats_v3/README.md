# Super Stats V3

The Super Stats canister is one of the core canister making up the backend of the 221Bravo App. This canister is stand alone and does not require the Index canister or TX Store to operate. The Stats canister fetches transactions directly from your chosen ICRC/ ICP ledger and creates the following stats  

-	Top Account Holders (ICRC/ ICP)
-	Top Principal Holders (ICRC only)
-	Total Holders (count of accounts/ principals)
-	Account Overview (Search)
-	Principal Overview (Search)
-	Daily/ Hourly Stats (Below)
-   -> Total transaction count
-	-> Total transaction value
-	-> Total transaction average
-	-> Total unique accounts
-	-> Total unique principals
-	-> Count/ Value of Burn, Mint, Transfer transactions
-	-> Count over time (Daily/ hourly)
-	-> Top (size) Mint/ Burn/ Transfer transactions


## Deploy/ Setup

The first step is to deploy a stats canister in the usual way, however it will require an argument giving the initial admin principal. This principal will initially be the only identity who can call the canister methods. Replace $(dfx identity get-principal) with a specific principal if required - eg '("2vxsx-fae")'

```bash
superIndexerCanister="YOUR CANISTER PRINCIPAL HERE"
dfx deploy ${superStatsCanister} --network ic --argument $(dfx identity get-principal) 
```

Once the canisters are deployed, the next step is to initialise the super stats canister setting the target ledger principal,  target ledger type and the stats timeframes. There are currently 3 types of supported version of the ICRC ledger: 

* DfinityIcp – The implementation of ICRC-1 that is used on the ICP ledger and OGY ledger.
* DfinityIcrc2 – The implementation of ICRC-1 and ICRC-2 used by SNS Projects and CK Tokens ledgers. 
* MemeIcrc – The implementation of ICRC-1 used by EXE, SNEED (old) and several other meme tokens. (NatLabs ICRC-1 canister). 

Your required ledger type can simply be chosen in the init argument below:


```bash
superIndexerCanister="YOUR CANISTER PRINCIPAL HERE"
indexType="DfinityIcrc2"
statsHours=24 # hourly data captures latest 24 hours.
statsDays=30 # daily data captures latest 30 days. (reduce this to 14 days for ICP ledger!)

dfx canister call ${superStatsCanister} --network ic init_target_ledger '( record {
                                                                        target_ledger = "'${tokenLedger}'";
                                                                        hourly_size = '$statsHours': nat8;
                                                                        daily_size = '$statsDays': nat8;
                                                                    }, variant { "'$indexType'" = null })'

```

If successful you should get the response “Target canister, tx store, fee and decimals set”. This indicates that the canister was able to register with the tx store and was able to communicate with the ledger canister. 
At this point you may want to add further cycles to the stats canister. 
Finally, you need to start a processing timer which will check the ledger for new transactions and process these into the index and stats. The only argument is the number of seconds the timer should wait between calls to the target ledger. 900 seconds (15 minutes) will likely suit your needs. 

```bash
superIndexerCanister="YOUR CANISTER PRINCIPAL HERE"
dfx canister call ${superStatsCanister} --network ic start_processing_timer '(900: nat64)'
``` 

The stats canister will then work through the history of the ledger in batches of 10,000 transactions and then continue to pull new transactions as available. 

You can check the canister logs to get an idea of the progress of the data-fetching with this argument `dfx canister call YOUR_CANISTER_ID_HERE –network ic get_all_logs`

## Available Methods
All available methods can be found in the API.rs files in ./core ./stats and ./timers