# Canister Templates - Guide

Modular_IC_Stable_Memory and Modular_IC_Stable_Structures are two templates used by 221Bravo App when building canister smart contracts. 

These templates have been built to be modular to make it easier to add features such as HTTPS Outcalls, timers or your own custom functions. Each template can use stable storage – either using the IC Stable Memory crate (https://docs.rs/ic-stable-memory/latest/ic_stable_memory/mem/index.html) or using IC Stable Structures (https://docs.rs/ic-stable-structures/latest/ic_stable_structures/)
Each template has a core module which has the minimum functionality required to create a canister with admin/ authorised users, event logging and cycles/ memory queries. 

## Deploying the canisters
When deploying the canister you will need to specify an admin principal (your dfx principal) who will having initial rights to call any gated methods. To do this you need to add 
`--argument ‘(“2vxsx-fae”)’`
To the end of the deploy argument (replace 2vxsx-fae with your own principal). For example
`dfx deploy example_canister –network ic –argument ‘(“xxxxx-xxx-xxxx”)’` 


## Core Functions
For ease, the available methods for each module are detailed in an API file in each module folder (for example src/core/api.rs) 

The core API methods are grouped by who is authorised to call them. Only Admin users can add/ remove admin or authorised privileges, read the canister logs, memory stats and set/ stop timers (if timer module is used)

Authorised users can be used to provide access to gated (non-admin) functions which you don’t want to be public. For example this is used on the 221Bravo Oracle canister to only allow the Central ‘Management’ canister to call the ‘worker’ canisters methods without allowing access to everyone. 

You can give everyone access to an authorised/ admin method by adding the anonymous principal (2vxsx-fae) to the list of admin/ authorised users. This can be useful if you want to only give access at certain times. 

To gate a custom function you have crated – simple add
`RUNTIME_STATE.with(|s| {
        s.borrow().data.check_admin(ic_cdk::caller().to_text());
    });`
At the start of the function associated with the method you want to gate.  Replace check_admin with check_authorised in the above script if you are gating for authorised level users.

For more examples, browse the v3 canisters in the main 221BravoApp_v3 repository.
