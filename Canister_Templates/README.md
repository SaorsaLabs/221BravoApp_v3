# Canister Templates - Guide

Modular_IC_Stable_Memory and Modular_IC_Stable_Structures are two templates uses by 221Bravo App when building canister smart contracts. 

These templates have been built to be modular to make it easier to add features such as HTTPS Outcalls, timers or your own custom functions. Each template can use stable storage – either using the IC Stable Memory crate (https://docs.rs/ic-stable-memory/latest/ic_stable_memory/mem/index.html) or using IC Stable Structures (https://docs.rs/ic-stable-structures/latest/ic_stable_structures/)
Each template has a core module which has the minimum functionality required to create a canister with admin/ authorised users, event logging and cycles/ memory queries. 

## Deploying the canisters
When deploying the canister you will need to specify an admin principal (your dfx principal) who will having initial rights to call any gated methods. To do this you need to add 
`--argument ‘(“2vxsx-fae”)’`
To the end of the deploy argument (replace 2vxsx-fae with your own principal). For example
`dfx deploy example_canister –network ic –argument ‘(“xxxxx-xxx-xxxx”)’` 


## Core Functions
