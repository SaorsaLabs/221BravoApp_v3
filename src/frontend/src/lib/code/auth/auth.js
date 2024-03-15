import { backendCanisterID, genesisNFTCanister } from '../constants.js';
import { StoicIdentity } from './stoicAuth.js';
import { browser } from '$app/environment';
import { authStore, authTrigger } from '../../stores/authStore.js';
import { backendCanisterIDL } from '../IDL/backend.js';
import { genesisNftIDL } from '../IDL/genesisNFT.js';
import { getIdentity, icActor } from '../fetch/icAgent.js';
import { goto } from '$app/navigation';
import { getUserData } from '../fetch/userData.js';

// STOIC LOGIN
async function stoicLogin() {
	StoicIdentity.load().then(async (identity) => {
		identity = await StoicIdentity.connect();
		const P = identity.getPrincipal().toText();
		if (P.length > 0) {
			// Verify holder
			let isHolder = await checkHODLER(P); 

			if (isHolder.result == true || isHolder.result == 'true') {
				authTrigger.update((n) => n + 1);
				let d = new Date();
				let time = d.getTime() / 1000; // current in secs.
				let EA = await encryptID(isHolder.account);
				let userData = await getUserData(EA);
				let oc_id = "abc123";
				if (userData[0]?.user_oc_principal?.length > 0) {
					oc_id = userData[0].user_oc_principal[0];
				}
				authStore.set(true, EA, time, "", oc_id);
				StoicIdentity.disconnect();
				goto("/members/home");
				return true;
			} else {
				authTrigger.update((n) => -1);
				StoicIdentity.disconnect();
				return false;
			}
		} // if
	}); // stoic ID
}

async function plugLogin() {
	if (browser) {
		try {
			const publicKey = await window.ic.plug.requestConnect();
			const account = window.ic.plug.accountId;
			if (account.length > 0) {
				//Verify Holder
				let isHolder = await checkHODLER(account); 
				if (isHolder.result == true || isHolder.result == 'true') {
					authTrigger.update((n) => n + 1);
					let d = new Date();
					let time = d.getTime() / 1000; // current in secs.
					let EA = await encryptID(isHolder.account);
					let userData = await getUserData(EA);
					let oc_id = "abc123";
					if (userData?.user_oc_principal?.length > 0) {
						oc_id = userData.user_oc_principal[0];
					}
					authStore.set(true, EA, time, "", oc_id);
					goto("/members/home");
					return true;
				} else {
					// Verified = false
					authTrigger.update((n) => -1);
					return false;
				}
			} // if
		} catch (e) {
			console.log(e);
		}
	}
}

async function bitfinityLogin() {
	if (browser) {
		try {
			const publicKey = await window.ic.infinityWallet.requestConnect();

			const p1 = await window.ic.infinityWallet.getPrincipal();
			const principal = p1.toText();
			if (principal.length > 0) {
				// Verify holder
				let isHolder = await checkHODLER(principal); 

				if (isHolder.result == true || isHolder.result == 'true') {
					authTrigger.update((n) => n + 1);
					let d = new Date();
					let time = d.getTime() / 1000; // current in secs.
					let EA = await encryptID(isHolder.account);
					let userData = await getUserData(EA);
					let oc_id = "abc123";
					if (userData?.user_oc_principal?.length > 0) {
						oc_id = userData.user_oc_principal[0];
					}
					authStore.set(true, EA, time, "", oc_id);
					goto("/members/home");
					return true;
				} else {
					// Verified = false
					authTrigger.update((n) => -1);
					return false;
				}
			} // if
		} catch (e) {
			console.log(e);
		}
	}
}

// non-encrypted check
async function checkHODLER(ID){
	let res = false;
	let acc = "";
	let backendActor;
	const Frontend_ID = getIdentity();

	if (ID.includes("-")) {
		// get account from principal 
		backendActor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
		let account = await backendActor.get_single_account(ID, 0);
		
		// check if holder 
		const nftActor = icActor(genesisNFTCanister, genesisNftIDL, Frontend_ID);
		let hodlers = await nftActor.getRegistry();
		let holderLen = hodlers.length ?? 0;
		let i;
		for(i=0; i<holderLen; i++){
			if(hodlers[i][1] ==  account) { 
				res = true;
				acc = account;
			 }
		}
		// if holder check setup or get stats
		if (res == true){ 

			let accountData;
			backendActor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
			try {
				accountData = await backendActor.add_new_user(account);
			} catch (error) {
				console.log(error, "user : ", account);
			}
		}
	} else {
		// check if holder 
		const nftActor = icActor(genesisNFTCanister, genesisNftIDL, Frontend_ID);
		let hodlers = await nftActor.getRegistry();
		let holderLen = hodlers.length ?? 0;
		let i;
		for(i=0; i<holderLen; i++){
			if(hodlers[i][1] ==  ID) { 
				res = true;
				acc = ID;
			}
		}
		if (res == true){ 
			let accountData;
			backendActor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
			try {
				accountData = await backendActor.add_new_user(ID);
			} catch (error) {
				console.log(error," User : ", ID);
			}
		}
	}
	return {result: res, account:acc };
}

async function encryptID(ID){
	const Frontend_ID = getIdentity();
	let backendActor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
	let encryptedID = await backendActor.encrypt(ID);
	return encryptedID;
}

export { stoicLogin, plugLogin, bitfinityLogin};
