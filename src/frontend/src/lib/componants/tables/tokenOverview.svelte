<script>
    import { Table, tableMapperValues, Paginator } from '@skeletonlabs/skeleton';
    import { createSparkline } from '../../code/sparkLines2.js';
    import { parseTicker } from '../../code/utils.js';
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';

    export let sourceData;

    onMount(()=>{
        calcTableValues();
    });

    const formatter = new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD',
        // minimumFractionDigits: 3,
        // maximumFractionDigits: 8,
    });

    sourceData.sort(function(a, b) {return b.mcap - a.mcap;}); // sort by marketcap decending 
    let chartData = [];
    let sourceDataLen = sourceData.length ?? 0;
    let count = 1;
    let svg, pt, pr, mc, cng24, cng7, dir; 
    
    // process other tokens
    function calcTableValues(){
        for(let i = 0; i< sourceDataLen; i++){
            svg = createSparkline(sourceData[i].sparkline, 2, 100, 40, false);
            pt = parseTicker(sourceData[i].cross);
            pr = (sourceData[i].price > 0.1) ? formatter.format(sourceData[i].price) : `$${Number(sourceData[i].price).toFixed(6)}`;
            mc = formatter.format(sourceData[i].mcap);
            dir = (Number(sourceData[i].change24) >= 0) ? "<b class='text-green-500'>▲</b>" : "<b class='text-red-500'>▼</b>"; 
            cng24 = `${Number(sourceData[i].change24).toFixed(2)} %  ${dir}`;
            
            cng7 = `${Number(sourceData[i].change7d).toFixed(2)} %`;
            chartData.push({count, name: pt.base, price: pr, cng24, cng7, chart: svg, mcap: mc});
            count ++;
        }
    }

    function onSelected(meta){
        goto(`/stats/${meta.detail[0]}`); 
        //console.log("DATA :: ", sourceData[index]);
	}

    $: sourceData = sourceData;

    let headLabels = ['#', 'Token', 'Price', '24hr %', 'Market Cap <p class="text-xs">(Fully Diluted)<p/>', ''];
    let paginationSettings = {
            page: 0,
            limit: 25,
            size: chartData.length,
            amounts: [5,10, 25, 100],
        };
    
    $: sourceBodySliced = chartData.slice(
		paginationSettings.page * paginationSettings.limit,
		paginationSettings.page * paginationSettings.limit + paginationSettings.limit
	);
    $: sourceOutput = tableMapperValues(sourceBodySliced, ['count', 'name', 'price', 'cng24', 'mcap', 'chart']);


    function onPageChange(e) {
		console.log('Paginator - event:page', e.detail);
	}
	function onAmountChange(e) {
		console.log('Paginator - event:amount', e.detail);
	}

</script>


<div class="p-1 rounded w-full">
    <h3 class="h3 pb-3">Top Tokens</h3>
    <Table 
        source={
            {head: headLabels, 
            body: sourceOutput, 
            meta: tableMapperValues(sourceBodySliced, ['name'])}} 
        interactive={true} 
        on:selected={onSelected} 
        regionHeadCell="bg-tertiary-400/40 text-center"
        regionCell="text-center"
    />
    <div class="mt-2">
        <Paginator
            bind:settings={paginationSettings}
            showFirstLastButtons="{true}"
            showPreviousNextButtons="{true}"
        />
    </div>
</div>