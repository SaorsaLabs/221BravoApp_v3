<script>
  import { onMount } from "svelte";
  import save1 from "$lib/images/icons/saveDark.png";
  import save2 from "$lib/images/icons/saveLight.png";
  import { getModalStore} from '@skeletonlabs/skeleton';
  import MemberServicesCombined from "../modal/memberServicesCombined.svelte";
  import { authStore } from '../../stores/authStore.js';

  export let accountToSave = undefined;

  let isDarkMode;

  // for popup
	const modalStore = getModalStore();
  let loggedIn;

  onMount(() => {
    // check login
    authStore.check();
		let x = authStore.read();
		loggedIn = x.data.loggedIn;

    // [][] for switching icons light/ dark [][]
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

    async function handleAddressClick(){
      if (loggedIn == true || loggedIn == 'true'){
        const c = { ref: MemberServicesCombined };
        const modal = {
          type: 'component',
          component: c,
          title: 'Custom Form Component',
          body: '',
          meta: { modalType: "addressbook", pageSelect: 1, saveAccount: accountToSave}, 
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
  
  <button on:click={() => handleAddressClick()}>
    <img src={isDarkMode ? save2 : save1} alt="Save Account" width="20px" style="margin-left:5px"/>
  </button>

  
  <style>
    button{
      cursor: pointer;
      background: none;
      padding: 0px;
      margin: 0px;
      border: 0px;
    }
  </style>