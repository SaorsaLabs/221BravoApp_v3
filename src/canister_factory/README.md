# 221Bravo Canister Factory
The canister factory supports the automation of canister creation by allowing a user to upload a number of canister wasms to the factory. Users can then call the canister factory to create canister(s) with a specified wasm.

The 221Bravo App currently uses the canister factory to simplify the process of creating the new index canisters required to add a token to the app. The canister factory is work-in-progress and further features will be added. 

The canister factory was inspired/ supported by code in the awesome [Juno Build](https://github.com/junobuild) repo. 

### Deploying the canister
```bash
dfx deploy canister_factory --network ic --argument $(dfx identity get-principal) 
```

### Uploading wasm
The easiest way to upload a wasm to the canister factory is to use the upload_wasm.mjs script. This can be found in the scripts folder. 

1. To create the wasm file you need to deploy the canister locally.
```bash
dfx start --background --clean

dfx deploy YOUR_CANISTER_NAME

dfx stop
```
Once the canister has been deployed you can find the wasm file in .dfx/local/canisters/YOUR_CANISTER_NAME/

2. Once the wasm file has been created you can then modify the upload_wasm.mjs file to ensure the correct file is uploaded. 
```javascript
// Arguments 
// 1. the principal of your canister_factory,
// 2. The canister/ wasm name in .dfx/local/canisters/
// 3. The name the wasm will be known by in the canister factory
// 4. The version (this can be anything)
// 5. true = calls the local network, false = calls IC mainnet        

// Line 117
uploadWasmToFactory("xxxx-xxxxx-xxx-xxx", "Example_Canister", "CANISTER123", "0.1.1", false);
```

3. Once the wasm is uploaded, you can then deploy a new canister and install the wasm. NOTE - make sure the canister factory has enough cycles. When creating the canister the canister factory will be added as a controller. You can add another controller by changing (const SAORSA_ADMIN) in canister_factory/constants.rs
```bash 
dfx canister call canister_factory --network ic create_new_cansister_with_wasm '("YOUR_CNSTR_NAME")' 
# The name must be the same as the name you gave the wasm during upload. To get a list of all available wasms use - 
dfx canister call canister_factory --network ic get_all_wasms
```


