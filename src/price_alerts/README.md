# 221Bravo - Price Alert Canister
The price alert canister work as part of a trio of canisters including The [Defi Oracle](https://github.com/SaorsaLabs/221BravoApp_v3/tree/main/src/defiOracle_mk2) and [Open Chat Bot Canister](https://github.com/SaorsaLabs/221BravoApp_v3/tree/main/src/ocBot_mk2). 

The alert canister is governed by a timer which initiates a call to the defi oracle to fetch the latest ICP token prices. The canister then checks these against user alerts (set by the 221Bravo Frontend). If any alerts are 'triggered' the alert canister calls the Open Chat Bot canister which then sends an alert message to the user's open chat account. 

The alert canister utilises stable memory to store the user/ price alert data. 

### Alert Struct
The InputAlert struct is used when sending and receiving data from the alert canister. 

```rust
pub struct InputAlert{
    pub id: u64, 
    pub user: String, 
    pub cross: String, 
    pub oc_id: String,
    pub price: f64,
    pub direction: u8 
}
```

When setting a price alert (via add_price_alert method) the following information must be supplied 
- user: This holds the hashed account relating to a specific 221Bravo User. 
- cross: The token cross as text eg 'CKBTC/ICP'
- oc_id: This is the Open Chat principal which will be messaged when the alert is triggered
- price: the price level at which the alert will trigger
- direction: 0 = down (crossed below price level) 1 = up (crossed above price level)

Note - id field is set by the canister based on the next available alert id. Input value can be set as 0 or any other valid u64.  

The add_price_alert method will return the alert ID associated with the input alert. 

### Canister Setup
The OC Bot canister (const OC_ALERT_BOT) can be set in constants.rs file in the core module.

1. Deploy the canister
```bash
dfx deploy price_alerts --network ic --argument $(dfx identity get-principal) 
```

2. Set the price oracle canister
```bash
dfx canister call price_alerts --network ic update_oracle_id '("YOUR_PRICE_ORACLE_CANISTER_ID")'
```

3. Start the processing timer (checks for new prices)
```bash
# Set timer to trigger every 300 seconds (5 minutes)

dfx canister call price_alerts --network ic start_alert_timer '(300: nat64)'
```