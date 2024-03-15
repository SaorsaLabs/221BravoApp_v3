import { getIdentity, icActor } from "./icAgent";
import { priceAlertCanisterID } from '../constants.js';
import { authStore } from "../../stores/authStore.js";
import { priceAlertsIDL } from '../IDL/priceAlerts.js';

export async function getUserAlerts(){
    const Frontend_ID = getIdentity();
    let actor = icActor(priceAlertCanisterID, priceAlertsIDL, Frontend_ID);
    ls = authStore.read();
    if (ls.data.loggedIn == true) {
        // ls.data.user
        let alerts = await actor.get_all_user_alerts(ls.data.user);
    }
}

export async function addUserAlert(direction, ){
    const ocID = "abc123"; // fetch from authStore
    const Frontend_ID = getIdentity();
    let actor = icActor(priceAlertCanisterID, priceAlertsIDL, Frontend_ID);
    ls = authStore.read();
    if (ls.data.loggedIn == true) {
        let newAlert = {
            'id' : 0,
            'direction' : direction,
            'oc_id' : IDL.Text,
            'cross' : IDL.Text,
            'user' : IDL.Text,
            'price' : IDL.Float64,
          }
    }
}

