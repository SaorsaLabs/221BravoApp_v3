<script>
 import bell from "$lib/images/icons/bellLight.png";
 import bell2 from "$lib/images/icons/bellDark.png";
 import { onMount } from "svelte";
 import { getModalStore} from '@skeletonlabs/skeleton';
 import MemberServicesCombined from "../modal/memberServicesCombined.svelte";
 import { authStore } from '../../stores/authStore.js';

export let saveCross = "";
const modalStore = getModalStore();
let isDarkMode;
let loggedIn;

onMount(() => {
    // check login
    authStore.check();
    let x = authStore.read();
    loggedIn = x.data.loggedIn;

    const updateMode = () => {
                isDarkMode = document.documentElement.classList.contains('dark');
            };
            // Initial check
            updateMode();
            // Listen for class changes on the html element
            const observer = new MutationObserver(mutations => {
                mutations.forEach(mutation => {
                if (mutation.attributeName === "class") {
                    updateMode();
                }
                });
            });
            observer.observe(document.documentElement, { attributes: true });
            return () => {
                observer.disconnect();
            }    
    });


    async function handleAlertClick(){
      if (loggedIn == true || loggedIn == 'true'){
        const c = { ref: MemberServicesCombined };
        const modal = {
          type: 'component',
          component: c,
          title: 'Custom Form Component',
          body: '',
          meta: { modalType: "pricealerts", pageSelect: "newAlert", saveCross: saveCross}, 
        };
        modalStore.trigger(modal);
      } 
      if (loggedIn == false || loggedIn == 'false'){
        const c = { ref: MemberServicesCombined };
        const modal = {
          type: 'component',
          component: c,
          title: 'Custom Form Component',
          body: '',
          meta: { modalType: "notmember" }, 
        };
        modalStore.trigger(modal);
      }

    }
</script>

<button class="pl-1 pr-1 pt-1 bg-tertiary-500/50 rounded" on:click={()=>{ handleAlertClick() }}>
    <img src={isDarkMode ? bell : bell2} alt="alerts" width="20px"/>
</button>	
