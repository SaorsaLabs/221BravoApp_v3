import { Actor, HttpAgent } from '@dfinity/agent';
import fetch from 'isomorphic-fetch';
import { Secp256k1KeyIdentity } from '@dfinity/identity-secp256k1';
import dotenv from 'dotenv';

export function getIdentity() {
    dotenv.config();
	let key = process.env.VITE_FRONTEND_ID;
	let ar = key.split(' ');
	let ID = Secp256k1KeyIdentity.fromSecretKey(ar);
	return ID;
}

export async function icActor(canister, idlFactory, identity, local) {
    if (local == true){
        const canisterId = canister;
        const host = 'http://127.0.0.1:8000/';
        let agent;
        if (identity) {
            agent = new HttpAgent({ identity: identity, fetch, host: host });
        } else {
            agent = new HttpAgent({ fetch, host: host });
        }

        await agent.fetchRootKey();
    
        return Actor.createActor(idlFactory, {
            agent: agent,
            canisterId: canisterId
        });
    }

    if (local == false){
        const canisterId = canister;
        const host = 'https://ic0.app';
        let agent;
        if (identity) {
            agent = new HttpAgent({ identity: identity, fetch, host: host });
        } else {
            agent = new HttpAgent({ fetch, host: host });
        }
    
        return Actor.createActor(idlFactory, {
            agent: agent,
            canisterId: canisterId
        });
    }
}