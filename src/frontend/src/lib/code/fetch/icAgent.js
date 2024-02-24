import { Actor, HttpAgent } from '@dfinity/agent';
import fetch from 'isomorphic-fetch';
import { Secp256k1KeyIdentity } from '@dfinity/identity-secp256k1';

export function getIdentity() {
	let key = import.meta.env.VITE_FRONTEND_ID;
	let ar = key.split(' ');
	let ID = Secp256k1KeyIdentity.fromSecretKey(ar);
	return ID;
}

export function icActor(canister, idlFactory, identity) {
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
