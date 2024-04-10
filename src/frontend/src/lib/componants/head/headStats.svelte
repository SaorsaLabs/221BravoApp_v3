<script>
    import { LightSwitch } from '@skeletonlabs/skeleton';
    import { onMount, onDestroy } from 'svelte';
    import Worker from '../../../service-worker.js?worker';
	import {authStore, authTrigger} from '../../stores/authStore.js';
	import { cookieStore } from '../../stores/cookieStore.js';
	import { browser } from '$app/environment';
	import CookieToast from '../shared/cookieToast.svelte';
	import { getModalStore} from '@skeletonlabs/skeleton';
	import userProfileModal from '../../componants/modal/userProfile.svelte';
	import MemberServicesCombined from '../modal/memberServicesCombined.svelte';
	import { goto } from '$app/navigation';
	import adressbook from "$lib/images/icons/AddressBookLight.png";
	import bell from "$lib/images/icons/bellLight.png";

    let syncWorker;
    let promise = callPromise();
	let headStats = " | | ";
    let resRecd = false;
    let tmr = null;
	let tmr2 = null;
    let blockRate = 0;
	let icpPrice = 0;
	let icpNodes = 0;
	let numTokens = 0;
	let loggedIn = false;
	let showToast = false;

	// for popup
	const modalStore = getModalStore();

    onMount(() => {
		loadWorker();
		tmr2 = setInterval(checkLogin, 600000); // 10 minutes 
		checkLogin();
		checkCookieSettings();
    });

	async function checkCookieSettings(){
		if(browser){
			let cs = await cookieStore.check();
			if (cs == "cookieStore doesn't exist yet"){
				// Ask user their initial preference
				cookieStore.init();
				showToast = true;
			}
			else if (cs == true || cs == undefined){
				// need to re-verify their preference
				showToast = true;
			}
			//cookieStore.set(false, 0);
		}
	}

	function checkLogin(){
		authTrigger.subscribe(value =>{
			if(browser){
				if(value>=1) {
					authStore.check();
					let x = authStore.read();
					loggedIn = x.data.loggedIn;
					// for gating whole app -- closed testing
					// if(loggedIn == "false" || loggedIn == false) goto("/testing");
				}
				if(value == 0){
				// init store if needed
				authStore.init();
				authTrigger.update(n => n + 1);
				}
			}
    	});
	}

	async function callPromise(){
		new Promise(async (resolve, reject) => {
		if (syncWorker){
			resRecd = false;
			syncWorker.postMessage({type: "fetch-stats-head", data: {}});
		}
		setTimeout(() => {
			// checking if message rec'd from worker
			if (resRecd == true) {
			resolve('Promise resolved!');
			}
		}, 500);
	});
	}
	
	const loadWorker = async () => {
			syncWorker = new Worker()
			// handle return data
			syncWorker.onmessage = (e) => {
				resRecd = true;
				if (e.data.result){
				blockRate = Number(e.data.result.blockrate).toFixed(2);
				icpPrice = Number(e.data.result.icpPrice).toFixed(2);
				numTokens = Number(e.data.result.numTokens);
				icpNodes = e.data.result.nodes;
				}
			};
			// first call
			promise = await callPromise();
			// timer for follow up calls
			tmr = setInterval(updateStats, 120000); //120 secs 
		};
	
	async function updateStats(){
		if (syncWorker){
			promise = callPromise();
			await checkLogin();
		}
	}

	async function handleLogin(){
		const c = { ref: userProfileModal };
		const modal = {
			type: 'component',
			component: c,
			title: 'Custom Form Component',
			body: '',
			//response: (r) => console.log('response:', r)
		};
		modalStore.trigger(modal);
	}

	async function handleLogout(){
    let d = new Date();
    let time = (d.getTime()/1000); // current in secs.
		authStore.set(false,"x",time,"abc123");
		authTrigger.update(n => n + 1);
		loggedIn = false;
		goto("/");
	}

	async function handleAddressClick(){
		const c = { ref: MemberServicesCombined };
		const modal = {
			type: 'component',
			component: c,
			title: 'Custom Form Component',
			body: '',
			meta: { modalType: "addressbook", pageSelect: 0, saveAccount: undefined}, 
			//response: (r) => console.log('response:', r)
		};
		modalStore.trigger(modal);
	}

	async function handleAlertClick(){
		const c = { ref: MemberServicesCombined };
		const modal = {
			type: 'component',
			component: c,
			title: 'Custom Form Component',
			body: '',
			meta: { modalType: "pricealerts" },
			//response: (r) => console.log('response:', r)
		};
		modalStore.trigger(modal);
	}

    onDestroy(() => {
		clearInterval(tmr);
		clearInterval(tmr2);
		tmr2 = 0;
	});

	 $: loggedIn = loggedIn; 
	 $: icpPrice = icpPrice;
	 $: numTokens = numTokens;
	 $: headStats = `Coins: ${numTokens} | ICP $${icpPrice}`;

</script>

<header class="bg-surface-900 text-white border-b-2 border-primary-700/25">
    <div class="container mx-auto px-4 py-2 min-w-full md:min-w-0">
        
        <div class="gap-8 columns-2 align-middle">
            <div>
				<p class="text-sm">{headStats}</p>
			</div>
            <div class="flex justify-end h-full">
                <div class="flex justify-between gap-3">
					<!-- Alert Modal Button -->
					<!-- <button >
						.
					</button> -->
	
					<!-- Alert Button -->
					 <span>
						{#if loggedIn == "true" || loggedIn == true}
							<button class="pl-1 pr-1 pt-1 bg-tertiary-500/50 rounded" on:click={()=>{ handleAlertClick() }}>
								<img src={bell} alt="alerts" width="20px"/>
							</button>	
						{/if}
					</span> 

					<!-- Address Button -->
					 <span>
						 {#if loggedIn == "true" || loggedIn == true}
							<button class="pr-1 pl-1 pt-1 bg-tertiary-500/50 rounded" on:click={()=>{ handleAddressClick() }}>
								<img src={adressbook} alt="addressbook" width="20px"/>
							</button>	
						{/if}
					</span>
					<!-- Login Modal -->
					 <span>
						{#if loggedIn == "false" || loggedIn == false}
	
							 <button class="pl-2 pr-2 bg-tertiary-500/50 rounded" on:click={()=>{handleLogin()}}>
								Sign In
							</button>
						{:else}
							<button class="pl-2 pr-2 bg-tertiary-500/50 rounded" on:click={()=>{handleLogout()}}>
								Sign Out
							</button>
						{/if}
					</span>
                    <span><LightSwitch/></span>
                </div>
            </div>
        </div>
    </div>
</header>

{#if showToast == true}
	<!-- <CookieToast showToast={showToast}/> -->
{/if}

<style>

</style>