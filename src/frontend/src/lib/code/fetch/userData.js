import { getIdentity, icActor } from '../fetch/icAgent.js';
import { backendCanisterIDL } from '../IDL/backend.js';
import { backendCanisterID } from '../constants.js';
import { priceAlertCanisterID } from '../constants.js';
import { priceAlertsIDL } from '../IDL/priceAlerts.js';
import { authStore } from '../../stores/authStore.js';

export async function getUserData(enc_ac){
    let ls;
    if (enc_ac) {
        const Frontend_ID = getIdentity();
        let backendActor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
        let userData = backendActor.get_user_data(enc_ac)
        return userData
    } else {
        ls = authStore.read();
        if (ls.data.loggedIn == true) {
            const Frontend_ID = getIdentity();
            let backendActor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
            let userData = backendActor.get_user_data(ls.data.user)
            return userData
        } else {
            return "Not Logged In";
        }
    }
}

export async function updateOCID(ocID){
    const Frontend_ID = getIdentity();
	let backendActor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
    let ls = authStore.read();
    if (ls.data.loggedIn == true) {
        let res = await backendActor.set_user_oc_id(ls.data.user, ocID);
        authStore.set(ls.data.loggedIn, ls.data.user, ls.data.authTime, "", ocID);
        return res;
    }
    return "Not Logged In";
}

export async function getUserAlerts(){
    const Frontend_ID = getIdentity();
	let backendActor = icActor(priceAlertCanisterID, priceAlertsIDL, Frontend_ID);
    let ls = authStore.read();
    if (ls.data.loggedIn == true) {
        let res = await backendActor.get_all_user_alerts(ls.data.user);
        return res;
    }
    return "Not Logged In";
}

export async function addUserAlert(cross, price, direction){
    const Frontend_ID = getIdentity();
	let backendActor = icActor(priceAlertCanisterID, priceAlertsIDL, Frontend_ID);
    let ls = authStore.read();
    if (ls.data.ocID == "abc123" || !ls.data.ocID) return "OC ID not set!";
    if (ls.data.loggedIn == true) {
        let alert = {
            id: 0,
            user: ls.data.user,
            cross: cross,
            oc_id: ls.data.ocID,
            price: price,
            direction: direction // 0 down, 1 up        
        };
        let res = await backendActor.add_price_alert(alert);

        if (res >= 0 ) return "Alert Added";
        else if (res == -1) return "Max Alerts Reached";
    }
    return "Not Logged In";
}

export async function removeUserAlert(cross, id){
    const Frontend_ID = getIdentity();
	let backendActor = icActor(priceAlertCanisterID, priceAlertsIDL, Frontend_ID);
    let ls = authStore.read();
    if (ls.data.loggedIn == true) {
        let alert = {
            id: id,
            user: ls.data.user,
            cross: cross,
            oc_id: "x",  // not required
            price: 0,    // not required
            direction: 0 // not required        
        };
        let res = await backendActor.remove_price_alert(alert);
        return res;
    }
    return "Not Logged In";
}

export async function addUserToken(){
    const Frontend_ID = getIdentity();
	let backendActor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
    let ls = authStore.read();
    if (ls.data.loggedIn == true) {
        let res = await backendActor.add_user_tokens(ls.data.user, 1);
        return res;
    }
    return "Not Logged In";
}