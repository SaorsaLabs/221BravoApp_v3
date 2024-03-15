<script>
	import LayoutCombine from "../lib/componants/shared/layoutCombine.svelte";
	import Head from "../lib/componants/head/head.svelte";
	import HeadStats from "../lib/componants/head/headStats.svelte";
	import Foot from "../lib/componants/foot/foot.svelte";
	import SearchBackground from   '$lib/images/Search_Background.webp'; 
	import SearchBalance from "../lib/componants/forms/searchBalance.svelte";
    import { onMount, onDestroy } from 'svelte';
	import Worker from '../service-worker.js?worker'
  	import TokenOverview from "../lib/componants/tables/tokenOverview.svelte";
  	import Loading from "../lib/componants/shared/loading.svelte";

	let syncWorker;
	let tmr;
	let tableData = [];
	let resRecd = false; 

	let promise = callPromise();

	async function callPromise(){
		new Promise(async (resolve, reject) => {
		if (syncWorker){
			resRecd = false;
			syncWorker.postMessage({type: "fetch-stats-home", data: {}});
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
				tableData = e.data.result.topTokens;
			};
			// first call
			promise = await callPromise();
			// timer for follow up calls
			tmr = setInterval(updateStats, 120000); //120 secs 
		};
	
	async function updateStats(){
		if (syncWorker){
			promise = callPromise();
			//syncWorker.postMessage({type: "fetch-stats-home", data: {}});
		}
	}

	$: tableData = tableData;
    onMount(() => {
		//alert("Price data is offline due to maintenance. Account and block search should be unaffected");
		loadWorker()
    });

	onDestroy(() => {
		clearInterval(tmr);
	});
</script>

<svelte:head>
	<title>Home - 221Bravo App</title>
</svelte:head>

<LayoutCombine>
	<span slot="headStats">
		<HeadStats/>
	</span>
	<span slot="head">
		<Head/>
	</span>
	<span slot="body" class="pb-4"> 
		
		<!-- SEARCH BOX -->
		<div class="container mx-auto px-4 py-5 flex flex-wrap min-w-full md:min-w-0 min-h-80 content-center justify-center items-center">
			<!-- background image -->
			<div class="w-full h-1 text-center -my-5">
				<span class="inline-block"><img src={SearchBackground} alt='Background' width="550px"/></span>
			</div>
			<SearchBalance/>
		</div>

		<!-- TOKENS OVERVIEW TABLE -->
		<div class="container mx-auto px-4 py-5 flex flex-wrap min-w-full md:min-w-0 content-center justify-center items-center">
			{#if resRecd == false}
				<Loading/>
			{:else}
				<TokenOverview sourceData={tableData}/>
			{/if}
		</div>
	</span>

	<span slot="foot">
		<Foot/>
	</span>
</LayoutCombine>
