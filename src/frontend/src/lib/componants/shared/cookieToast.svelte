<script>
import { getToastStore } from '@skeletonlabs/skeleton';
import { page } from '$app/stores';
import { cookieStore } from '../../stores/cookieStore.js';

export let showToast = false;

// current route
let loadedPage; 
const getPage = page.subscribe(value => {
    loadedPage = value.route.id;
  });

const toastStore = getToastStore();
const t = {
	message: 'We use cookies to help us improve 221Bravo.App <a href="/terms">(more info here)</a>. Click OK to accept or ‘X’ to reject cookies',
    autohide: false,
    action: {
		label: 'Ok',
		response: () => {
            let d = new Date();
		    let time = d.getTime() / 1000; // secs
            cookieStore.set(true, time);
        }
	},
};
let toastId;

if(showToast == true){
    // only show on the home page. 
    if (loadedPage == "/"){
        // set to false first
        let d = new Date();
		let time = d.getTime() / 1000; // secs
        cookieStore.set(false, time);
        toastId = toastStore.trigger(t);
    }
}

// close toast
//toastStore.close(toastId);
</script>
