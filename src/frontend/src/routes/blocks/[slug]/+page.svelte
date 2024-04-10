<script>
    import LayoutCombine from "../../../lib/componants/shared/layoutCombine.svelte";
	import Head from "../../../lib/componants/head/head.svelte";
	import HeadStats from "../../../lib/componants/head/headStats.svelte";
	import Foot from "../../../lib/componants/foot/foot.svelte";
    import {_slugData} from './+page';
    import { onMount, onDestroy } from 'svelte';
    import { fetchLatestBlocks, fetchCustomBlocks } from '../../../lib/code/fetch/blockData.js';
    import TokenLogos from '../../../lib/componants/shared/tokenLogos.svelte';
    import Loading from '../../../lib/componants/shared/loading.svelte';
    import BlockTxTable from "../../../lib/componants/tables/blockTxTable.svelte";
    import tokenList from '../../../lib/staticData/tokens.json';
    import Fuse from 'fuse.js';
    import { goto } from "$app/navigation";
    import { MAX_BLOCK_SEARCH } from '../../../lib/code/constants.js';

    let token = _slugData.token;
    let tokenInput;
    let startBlock;
    let endBlock;
	let resRecd = false;
    let customRange = false;
    let slugRange = false;
    let blockData = [];
    let decimals;

    let latestBlocks;
    let promise;
    let isError = false;
    let errorText = "";

    function validateInput(startBlock, endBlock){
        let sb = Number(startBlock);
        let eb = Number(endBlock);
        if (sb > eb) return "Start Block cannot be higher than End Block";
        else if ((eb - sb) > MAX_BLOCK_SEARCH ) return "Max fetch size is 10,000 blocks";
        else if (sb < 0 || sb < 0) return "Start Block/ End Block must be grater than zero";
        else if (/^[0-9]+$/.test(startBlock) == false || /^[0-9]+$/.test(endBlock) == false) return "Search input must be a number";
        else {
            return "OK"
        }
    }

    async function fetchData(){
        resRecd = false;
        isError = false;
        errorText = "";
        
        // check for slug data
        if (_slugData.startBlock != null && _slugData.endBlock != null){
            slugRange = true;
            customRange = true;
        }

        // no user input + no slug input.. fetch latest blocks
        if (customRange == false && slugRange == false){
            latestBlocks = await fetchLatestBlocks(token, 10000);
            blockData = latestBlocks.blocks;
            let blockLen = blockData?.length ?? 0;
            if(blockLen != 0) {
                startBlock = blockData[blockLen-1].block;
                endBlock = blockData[0].block;
            }
            decimals = latestBlocks.tokenData.decimals;
            tokenInput = token;
        } 
        // custom block range 
        else { 
            if (slugRange == true) {
                // input via slug
                tokenInput = token;
                startBlock = _slugData.startBlock;
                endBlock = _slugData.endBlock;
                let vi = validateInput(startBlock, endBlock);
                if(vi == "OK"){
                    latestBlocks = await fetchCustomBlocks(token, startBlock, endBlock);
                    blockData = latestBlocks.blocks;
                    decimals = latestBlocks.tokenData.decimals;
                    tokenInput = token;
                } else {
                    isError = true;
                    errorText = vi;
                }
            }
            else {
                // input via user
                let vi = validateInput(startBlock, endBlock);
                if(vi == "OK"){
                    latestBlocks = await fetchCustomBlocks(token, startBlock, endBlock);
                    blockData = latestBlocks.blocks;
                    decimals = latestBlocks.tokenData.decimals;
                    tokenInput = token;
                } else {
                    isError = true;
                    errorText = vi;
                }
            }
        }
        resRecd = true;
    }

    // FUSE Search
    let searchResults;
    const fuseOptions = {
        isCaseSensitive: false,
        keys: [
            "ticker",
            "shortName"
        ]
    };
    const fuse = new Fuse(tokenList, fuseOptions);

    function handleInputChange(){
        if(tokenInput?.length != undefined){
            searchResults = fuse.search(tokenInput);
            // max 5 results
            if (searchResults?.length > 5) {
                searchResults = searchResults.slice(0, 5);
            }
        }
        if(tokenInput?.length <= 1){
            searchResults = [];
        }
    }

    async function tokenSelect(tokenSelected){
        customRange = false;
        slugRange = false;
        _slugData.startBlock = null;
        _slugData.endBlock = null;
        searchResults = [];
        tokenInput = tokenSelected;
        token = tokenSelected;
        goto(`./${tokenSelected}`);
        fetchData();
    }

    async function customRangeSearch(){
        customRange = true;
        slugRange = false;
        fetchData();
    }

    onMount(() => {
		promise = fetchData()
    });

	onDestroy(() => {
	
	});

    $: blockData;
</script>

<svelte:head>
	<title>Block Explorer</title>
</svelte:head>

<LayoutCombine>
	<span slot="headStats">
		<HeadStats stats={token}/>
	</span>
	<span slot="head">
		<Head/>
	</span>
	<span slot="body" class="pb-4">
        <div class="container mx-auto px-4 flex flex-wrap min-w-full md:min-w-0">
            <a href="/">Home </a>/ Blocks/ {token}
        </div>
        <!-- First Section -->
        <div class="container mx-auto px-4 py-5 flex flex-wrap min-w-full md:min-w-0 content-center justify-center items-center">
        {#if resRecd == false}
            <div class="p-3 min-h-screen">
                <Loading/>
                <p class="text-center pt-3">Fetching blocks from {token} ledger...</p>
            </div>
        {:else}
            <!-- search bar -->
            <div class="w-full dark:bg-primary-500/40 bg-primary-700 rounded p-1 mb-4">
                <div class="flex flex-col lg:flex-row">
                    <div class="flex-1">
                        <div class="flex-row pl-1">
                            <span class="flex-1"> Start Block: </span>
                            <span class="flex-1"> 
                                <input class="input pl-2 flex-1 w-2/3 ml-3" title="Start Block" type="text" placeholder="Start Block" bind:value={startBlock}/>
                            </span>
                        </div>
                    </div>
                    <div class="flex-1">
                        <div class="flex-row">
                            <span class="flex-1"> End Block: </span>
                            <span class="flex-1"> 
                                <input class="input pl-2 flex-1 w-2/3 ml-3" title="End Block" type="text" placeholder="End Block" bind:value={endBlock}/>
                            </span>
                        </div>
                    </div>
                    <div class="flex-1">
                        <div class="min-width-3/6">

                            <div class="flex-row">
                                <span class="flex-1">Token: </span>
                                <input class="input pl-2 flex-1 w-2/3" title="Token (Ticker or name)" type="text" placeholder="Token Name" on:keyup={() => {handleInputChange()}} bind:value={tokenInput}/>
                                <button class="flex-1 bg-primary-500/50 rounded pl-2 pr-2 ml-2" on:click={() => {customRangeSearch()}}>Search</button>
                            </div>
                                
                            
                            {#if searchResults?.length > 0}
                            <!-- Token input bar search results -->
                            <nav class="list-nav">
                                <ul>
                                    {#each searchResults as res}
                                        <li>
                                            <button class="w-full" on:click={()=>{ tokenSelect(res.item.ticker)}}>
                                                <span class="badge bg-primary-500" style="width:40px"><TokenLogos token={res.item.ticker}/></span>
                                                <span class="flex-auto">{res.item.shortName}</span>
                                            </button>
                                        </li>
                                    {/each}
                                </ul>
                            </nav>
                            {/if}
                            
                            <!-- Output/ warning -->
                            <!-- {#if showWarning == true}
                                <div class="min-width-3/6">
                                    <p class="text-warning-500">{warnText}</p>
                                </div>
                            {/if} -->
                        </div>
                    </div>
                </div>
            </div>

            {#if customRange == false}     
                <div class="w-full flex justify-left text-left">
                    <p class="h4 pl-1 flex">Latest {latestBlocks.tokenData.shortName} blocks </p>
                    {#if token != "ICP"} 
                        <div class="flex pl-3" style="width:40px"><TokenLogos token={token}/></div>
                        {:else}
                        <div class="flex pl-3 pt-2" style="width:40px"><TokenLogos token={token}/></div>
                    {/if} 
                </div>
            {:else}
                <div class="w-full justify-left text-left"><p class="h4 pl-1">{latestBlocks.tokenData.shortName} Ledger Blocks: {startBlock} to {endBlock} </p></div>
            {/if}
            <BlockTxTable sourceData={blockData} decimals={decimals} token={token}/>
        {/if}
            
        </div>
	</span>

	<span slot="foot">
		<Foot/>
	</span>
</LayoutCombine>


<style>

</style>