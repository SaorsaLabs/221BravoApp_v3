import { backendCanisterID } from '../constants.js'
import { getIdentity, icActor } from './icAgent.js';
import { backendCanisterIDL } from '../IDL/backend.js';
import { authStore, authTrigger } from '../../stores/authStore.js';

export async function saveAccount(account, name) {
    let res;

    // trim if larger than 200 characters
    let ac, nm;
    if (account.length > 200) {
        ac = account.substring(0, 200);
    } else {
        ac = account;
    }
    if (name.length > 200) {
        nm = name.substring(0, 200);
    } else {
        nm = name;
    }

    // get user 
    let usr = authStore.read();
    // save details
    if (usr.data.loggedIn == true){
        const Frontend_ID = getIdentity();
		const backendActor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
		let save_res = await backendActor.add_user_named_accounts(usr.data.user, ac, nm);
        res = save_res;
    } else {
        res = "Not Logged In!"
    }
    return res;
}

export async function getAllAccounts() {
    const Frontend_ID = getIdentity();
    let usr = authStore.read();
    let actor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
    let res = await actor.get_all_user_named_accounts(usr.data.user); 
    return res;
}

export async function deleteAccount(account){
    const Frontend_ID = getIdentity();
    let usr = authStore.read();
    let actor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
    let res = await actor.remove_user_named_account(usr.data.user, account); 
    if (res == "Account removed from directory") {
        authTrigger.update((n) => n + 1);
    }
    return res;
}