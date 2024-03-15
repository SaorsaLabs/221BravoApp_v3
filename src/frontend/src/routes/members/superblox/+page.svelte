<script>
    import LayoutCombine from "../../../lib/componants/shared/layoutCombine.svelte";
	import Head from "../../../lib/componants/head/head.svelte";
	import HeadStats from "../../../lib/componants/head/headStats.svelte";
	import Foot from "../../../lib/componants/foot/foot.svelte";
    import { onMount } from 'svelte';
    import { fetchSuperBlox } from '../../../lib/code/fetch/blockData.js';
    import Loading from '../../../lib/componants/shared/loading.svelte';
    import SuperBloxTable from "../../../lib/componants/tables/superBloxTable.svelte";

	let resRecd = false;
    let promise;
    let blocks;

    async function fetchBlox(){
        resRecd = false;
        blocks = await fetchSuperBlox();
        resRecd = true;
    }

    onMount(() => {
		promise = fetchBlox();
    });
</script>

<svelte:head>
	<title>SuperBlox Explorer</title>
</svelte:head>

<LayoutCombine>
	<span slot="headStats">
		<HeadStats stats={"BLA BLA"}/>
	</span>
	<span slot="head">
		<Head/>
	</span>
	<span slot="body" class="pb-4">
        <div class="container mx-auto px-4 flex flex-wrap min-w-full md:min-w-0">
            <a href="/">Home </a>/ Blocks/ {"SuperBlox"}
        </div>
    
        <div class="container mx-auto px-4 py-5 flex flex-wrap min-w-full md:min-w-0 content-center justify-center items-center">
        {#if resRecd == false}
            <div class="p-3 min-h-screen">
                <Loading/>
                <p class="text-center pt-3">Fetching blocks from ALL ledgers...</p>
            </div>
        {:else}
            <SuperBloxTable sourceData={blocks} />
        {/if}
            
        </div>
	</span>

	<span slot="foot">
		<Foot/>
	</span>
</LayoutCombine>
