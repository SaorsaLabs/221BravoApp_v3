<script>
    import stoicLogo from '$lib/images/icons/Stoic.png';
	import plugLogo from '$lib/images/icons/Plug.png';
	import bitLogo from '$lib/images/icons/Bitfinity.png';
    import Loading from '../../componants/shared/loading.svelte';
	import { browser } from '$app/environment';
	import {stoicLogin, plugLogin, bitfinityLogin} from '../../code/auth/auth.js';
    import { createEventDispatcher } from 'svelte';
    import {authStore} from '../../stores/authStore.js';
    import { goto } from '$app/navigation';

    let intervalId;

    // return message to userProfile modal
    const dispatch = createEventDispatcher();
    function returnMessage(message) {
        dispatch('message', message);
        clearInterval(intervalId); 
    }

    // check for login (close modal)
    function checkLogin(){
        intervalId = setInterval(timerCallback, 500);
        // clear the timer after 10 seconds
        setTimeout(noMembership, 10000); 
    }
    let showLoginButtons = true;
    // timed out
    function noMembership(){
        authStore.check();
		let x = authStore.read();
	    if (x.data.loggedIn == false || x.data.loggedIn == 'false') {
            showLoginButtons = false;
            clearInterval(intervalId); 
            goto("/members");
            returnMessage("not-member");
        }
    }

    function timerCallback(){
        authStore.check();
		let x = authStore.read();
	    if (x.data.loggedIn == true || x.data.loggedIn == 'true') {
            returnMessage("logged-in");
        }
    }

    let stoicLoading = false;
	let plugLoading = false;
	let bitLoading = false;

	async function handleStoicClick(){
		if(browser){
			stoicLoading = true;
			await stoicLogin();
            checkLogin();
		}
	}
	async function handlePlugClick(){
		if(browser){
			plugLoading = true;
			await plugLogin();
            checkLogin();
		}
	}
	async function handleBitfinityClick(){
		if(browser){
			bitLoading = true;
			await bitfinityLogin();
            checkLogin();
		}
	}
</script>
{#if showLoginButtons == true}
<div class="flex"><p class="h3">Login</p></div>
    <div class="flex pb-4 content-center justify-center items-center">
        <table>
            <tr>
                <td><img src={stoicLogo} alt="Stoic Wallet Logo" width="54px"/></td>
                <td>
                    {#if stoicLoading == false}
                        <button class="pl-2 pr-2 bg-tertiary-500 rounded ml-4 h3" on:click={()=>{ handleStoicClick() }}>Stoic Login</button>
                    {:else}
                        <Loading/>
                    {/if}
                </td>
            </tr>
            <tr>
                <td><img src={bitLogo} alt="Bitfinity Wallet Logo" width="50px"/></td>
                <td>
                    {#if bitLoading == false || plugLoading == "false"}
                        <button class="pl-2 pr-2 bg-tertiary-500 rounded ml-4 h3" on:click={()=>{ handleBitfinityClick() }}>Bitfinity Login</button>
                    {:else}
                        <Loading/>
                    {/if}
                    
                </td>
            </tr>
            <tr>
                <td><img src={plugLogo} alt="Plug Wallet Logo" width="50px"/></td>
                <td>
                    {#if plugLoading == false || plugLoading == "false"}
                        <button class="pl-2 pr-2 bg-tertiary-500 rounded ml-4 h3" on:click={()=>{ handlePlugClick() }}>Plug Login</button>
                    {:else}
                        <Loading/>
                    {/if}
                </td>
            </tr>
        </table>
    </div>
{:else}
   <p class="h3"> No Membership Found 😟 </p>
   <p> </p>
{/if}