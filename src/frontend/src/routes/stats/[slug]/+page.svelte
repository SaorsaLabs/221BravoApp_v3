<script>
    import LayoutCombine from "../../../lib/componants/shared/layoutCombine.svelte";
	import Head from "../../../lib/componants/head/head.svelte";
	import HeadStats from "../../../lib/componants/head/headStats.svelte";
	import Foot from "../../../lib/componants/foot/foot.svelte";
    import {_slugData} from './+page';
    import BasicCandleChart from '../../../lib/componants/charts/basicCandleChart.svelte';
    import { onMount, onDestroy } from 'svelte';
	import Worker from '../../../service-worker.js?worker';
    import MintBurnDonutChart from '../../../lib/componants/charts/mintBurnDonutChart.svelte';
    import BarChart from '../../../lib/componants/charts/barChart.svelte';
    import TokenLogos from '../../../lib/componants/shared/tokenLogos.svelte';
    import { nanoToDate } from '../../../lib/code/utils.js';
    import Loading from '../../../lib/componants/shared/loading.svelte';
    import TopHolders from '../../../lib/componants/tables/topHolders.svelte';
    import TopMintBurnTx from "../../../lib/componants/tables/topMintBurnTx.svelte";
    import AlertButton from '../../../lib/componants/shared/alertButton.svelte';

    let token = _slugData.token;
	let syncWorker;
	let tmr;
	let resRecd = false;
    let resRecdChart = false;
    let resRecdOnce = false;
    let promise = callPromise();

    const formatter = new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD',
        // minimumFractionDigits: 3,
        // maximumFractionDigits: 8,
    });
    const formatNumber = new Intl.NumberFormat('en-US', {
        maximumFractionDigits: 0,
    });
    const formatNumber3dp = new Intl.NumberFormat('en-US', {
         maximumFractionDigits: 3,
    });

    // chart display mode
    let mbtToggle = "hr";
    let totToggle = "hr";
    let topHoldersDisplay = "ac";
    let topMbtDisplay = "tx";
    let topMbtTimeDisplay = "dy";
    
    // marketplace mode
    let mpCurrency = "USD";
    let mpLoaded = false;
    let mpDataAR = [];
    let mpDataLen;

    // statsData
    let chartTime = "h1";
    let tokenDecimals;
    let chartDecimals;
    let change24 = 0;
    let change7d = 0;
    let marketcap = 0;
    let supply = 0;
    let supplyRaw = 0;
    let uniqueACS = 0;
    let uniquePRS = 0;
    let hourlyMBT = [0,0,0];
    let hourlyMBTformat = [0,0,0];
    let dailyMBT = [0,0,0];
    let dailyMBTformat = [0,0,0];
    let hours = 0;
    let days = 0;
    let supplyChangeHR = 0;
    let supplyChangeDY = 0;
    let hourlyTOTvalues = [];
    let hourlyTOTlabels = [];
    let dailyTOTvalues = [];
    let dailyTOTlabels = [];
    let chartData = [];
    let topAcHolders = [];
    let topPrHolders = [];
    let hasPrincipalHolders = false;
    let teamWebsite = "";
    let tradeUrl = "";
    let dTopTX  = [];
    let dTopBurn, dTopMint;
    let hTopTX, hTopBurn, hTopMint;
    let cross;
    let isDarkMode = false;
	
	async function callPromise(){
		new Promise(async (resolve, reject) => {
		if (syncWorker){
			resRecd = false;
			syncWorker.postMessage({type: "fetch-stats-overview", data: {token: token}});
		}
		setTimeout(() => {
			// checking if message rec'd from worker
			if (resRecd == true) {
			resolve('Promise resolved!');
			}
		}, 500);
	});
	}

    async function callPromiseChart(){ 
        new Promise(async (resolve, reject) => {
		if (syncWorker){
			resRecdChart = false;
			syncWorker.postMessage({type: "fetch-stats-overview-charts", data: {token: token, timeframe: chartTime}});
		}
		setTimeout(() => {
			// checking if message rec'd from worker
			if (resRecdChart == true) {
			resolve('Promise resolved!');
			}
		}, 500);
	});
    }

    async function callPromiseMarketplaces(){ 
        new Promise(async (resolve, reject) => {
		if (syncWorker){
			mpLoaded = false;
			syncWorker.postMessage({type: "fetch-stats-overview-markets", data: {token: token, mode: mpCurrency}});
		}
		setTimeout(() => {
			// checking if message rec'd from worker
			if (mpLoaded == true) {
			resolve('Promise resolved!');
			}
		}, 500);
	});
    }

    async function changeMktCurrency(){
        mpLoaded = false;
        await callPromiseMarketplaces();
    }
	
	const loadWorker = async () => {
			syncWorker = new Worker()
			// handle return data
			syncWorker.onmessage = (e) => {
                if (e.data.result){
                    updateStats(e.data.result);
                    resRecd = true;
                    resRecdOnce = true;
                }
                if (e.data.chartResult){
                    updateChart(e.data.chartResult);
                    resRecdChart = true;
                }
                if (e.data.marketResult){
                    updateMarket(e.data.marketResult);
                    mpLoaded = true;
                }
			};
			// first call
			promise = await callPromise();
            await callPromiseChart();
            await callPromiseMarketplaces();
			// timer for follow up calls
			tmr = setInterval(callWorker, 120000); //120 secs
		};
	
	async function callWorker(){
		if (syncWorker){
			promise = callPromise();
            await callPromiseChart();
            await callPromiseMarketplaces();
		}
	}

    function updateStats(data){
        if(!data) return;
        // cross
        cross = data?.tokenData?.tradePair;
        // clear
        hourlyTOTvalues = [];
        hourlyTOTlabels = [];
        dailyTOTvalues = [];
        dailyTOTlabels = [];
        // process new data
        tokenDecimals = data.tokenData.decimals;
        chartDecimals = data.tokenData.chartDecimals;
        teamWebsite = data.tokenData.links[0].url;
        tradeUrl = data.tokenData.tradeURL;
        if (data?.priceChangeData) {
            change24 = Number(data.priceChangeData.change24).toFixed(2);
            change7d = Number(data.priceChangeData.change7d).toFixed(2);
            marketcap = formatter.format(data.priceChangeData.mcap);
            supply = formatNumber.format(data.priceChangeData.supply);
            supplyRaw = data.priceChangeData.supply;
            let previousSupplyHR = Number(data.priceChangeData.supply) - (hourlyMBT[0] - hourlyMBT[1]);
            supplyChangeHR = formatNumber3dp.format(((hourlyMBT[0] - hourlyMBT[1]) / previousSupplyHR) * 100);
            let previousSupplyDY = Number(data.priceChangeData.supply) - (dailyMBT[0] - dailyMBT[1]);
            supplyChangeDY = formatNumber3dp.format(((dailyMBT[0] - dailyMBT[1]) / previousSupplyDY) * 100);   
        }
        uniqueACS = formatNumber.format(data.totalHolders.total_accounts);
        uniquePRS = formatNumber.format(data.totalHolders.total_principals);
        // hourly mint/ burn/ transfer
        hourlyMBT[0] = Number(Number(data.hourlyData.mint_stats.total_value) / Math.pow(10, data.tokenData.decimals)).toFixed(3);
        hourlyMBT[1] = Number(Number(data.hourlyData.burn_stats.total_value) / Math.pow(10, data.tokenData.decimals)).toFixed(3);
        hourlyMBT[2] = Number(Number(data.hourlyData.transfer_stats.total_value) / Math.pow(10, data.tokenData.decimals)).toFixed(3);
        hourlyMBTformat[0] = formatNumber3dp.format(hourlyMBT[0]);
        hourlyMBTformat[1] = formatNumber3dp.format(hourlyMBT[1]);
        hourlyMBTformat[2] = formatNumber3dp.format(hourlyMBT[2]);

        // daily mint/ burn/ transfer
        dailyMBT[0] = Number(Number(data.dailyData.mint_stats.total_value) / Math.pow(10, data.tokenData.decimals)).toFixed(3);
        dailyMBT[1] = Number(Number(data.dailyData.burn_stats.total_value) / Math.pow(10, data.tokenData.decimals)).toFixed(3);
        dailyMBT[2] = Number(Number(data.dailyData.transfer_stats.total_value) / Math.pow(10, data.tokenData.decimals)).toFixed(3);
        dailyMBTformat[0] = formatNumber3dp.format(dailyMBT[0]);
        dailyMBTformat[1] = formatNumber3dp.format(dailyMBT[1]);
        dailyMBTformat[2] = formatNumber3dp.format(dailyMBT[2]);
     
        // top holders
        if (token != "ICP"){
            topAcHolders = data.topHolders?.accounts;
            topPrHolders = data.topHolders?.principals;
        } else {
            topAcHolders = data.topHolders?.accounts;
        }
        let topPrHolderLen = topPrHolders?.length ?? 0;
        if (topPrHolderLen > 0) {hasPrincipalHolders = true};
        hours = data.hourlyData.count_over_time.length ?? 0;
        days = data.dailyData.count_over_time.length ?? 0;
        
        let i, tm;
        for(i=hours-1; i>=0; i--){
            hourlyTOTvalues.push(Number(data.hourlyData.count_over_time[i].total_count));
            tm = nanoToDate(Number(data.hourlyData.count_over_time[i].start_time));
            hourlyTOTlabels.push(tm.shortTime);
        }
        for(i=days-1; i>=0; i--){
            dailyTOTvalues.push(Number(data.dailyData.count_over_time[i].total_count));
            tm = nanoToDate(Number(data.dailyData.count_over_time[i].start_time));
            dailyTOTlabels.push(tm.dateOnly);
        }
        // TOP TXS
        dTopTX = data.dailyData.top_transfers;
        dTopBurn = data.dailyData.top_burns;
        dTopMint = data.dailyData.top_mints;
        hTopTX = data.hourlyData.top_transfers;
        hTopBurn = data.hourlyData.top_burns;
        hTopMint = data.hourlyData.top_mints;
    }

    function updateChart(data){
        chartDecimals = data.chartDecimals;
        chartData = [];
        let i;
        let tm; 
        
        if (token != "ICP"){
            let dataLen = data.ohlc[0].length ?? 0;
            for(i=0; i<dataLen; i++){
            tm = nanoToDate(Number(data.ohlc[0][i].open_time));
            chartData.push({
                time: tm.UTCTimestampSecs, 
                open: Number(data.ohlc[0][i].open.usd_price), 
                high: Number(data.ohlc[0][i].high.usd_price),
                low: Number(data.ohlc[0][i].low.usd_price),
                close: Number(data.ohlc[0][i].close.usd_price)
            });
            }
            // reverse if needed. Should be oldest to newest.
            if (chartData[0].time > chartData[1].time){
                chartData.sort(function(a, b) {return Number(a.time) - Number(b.time);});
            }
        } else {
            let dataLen = data.ohlc.length ?? 0;
            for(i=0; i<dataLen; i++){
            tm = nanoToDate(Number(data.ohlc[i].open_time)*1000000000);
            chartData.push({
                time: tm.UTCTimestampSecs, 
                open: Number(data.ohlc[i].open.usd_price), 
                high: Number(data.ohlc[i].high.usd_price),
                low: Number(data.ohlc[i].low.usd_price),
                close: Number(data.ohlc[i].close.usd_price)
            });
            }
        }
    }

    function updateMarket(data){
        mpDataAR = data;
        mpDataLen = mpDataAR?.length ?? 0;
        mpLoaded = true;
    }

    
    onMount(() => {
        
    const updateMode = () => {
        resRecdChart = false;
        clearInterval(tmr);
        isDarkMode = document.documentElement.classList.contains('dark');
        loadWorker();
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

	onDestroy(() => {
		clearInterval(tmr);
	});

    $: chartDecimals = chartDecimals;
    $: chartData = chartData;
    $: isDarkMode;
</script>

<svelte:head>
	<title>{token} Stats</title>
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
            <a href="/">Home </a>/ {token}
        </div>
        <!-- First Section -->
        <div class="container mx-auto px-4 py-5 flex flex-wrap min-w-full md:min-w-0 content-center justify-center items-center">
        
            <!-- Chart/ Sidebar holder -->
            <div class="w-full flex flex-col lg:flex-row gap-2">
                    <!-- CHART -->
                    <div class="p-1 rounded lg:w-2/3 w-full">
                        <div class="w-full">
                            <table class="w-full">
                                <tr>
                                    <td><p>Token: {token}/USD<span class="ml-2"><AlertButton saveCross={cross}/></span></p></td>
                                    <td><p class="text-right pr-2">Timescales :: 
                                        {#if token != "ICP"}
                                            {#if chartTime == "m5"}
                                                <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => {chartTime = "m5"; callPromiseChart()}}>
                                                    M5    
                                                </button>
                                            {:else}
                                                <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {chartTime = "m5"; callPromiseChart()}}>
                                                    M5    
                                                </button>
                                            {/if}
                                        | 
                                        {/if}
                                        {#if chartTime == "m15"}
                                            <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => {chartTime = "m15"; callPromiseChart()}}>
                                                M15    
                                            </button>
                                        {:else}
                                            <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {chartTime = "m15"; callPromiseChart()}}>
                                                M15    
                                            </button>
                                        {/if}
                                        {#if token != "ICP"}
                                            | 
                                            {#if chartTime == "h1"}
                                                <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => {chartTime = "h1"; callPromiseChart()}}>
                                                    H1    
                                                </button>
                                            {:else}
                                                <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {chartTime = "h1"; callPromiseChart()}}>
                                                    H1    
                                                </button>
                                            {/if}
                                            | 
                                            {#if chartTime == "d1"}
                                                <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => {chartTime = "d1"; callPromiseChart()}}>
                                                    D1    
                                                </button>
                                            {:else}
                                                <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {chartTime = "d1"; callPromiseChart()}}>
                                                    D1    
                                                </button>
                                            {/if}
                                            | 
                                            {#if chartTime == "w1"}
                                                <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => {chartTime = "w1"; callPromiseChart()}}>
                                                    W1    
                                                </button>
                                            {:else}
                                                <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {chartTime = "w1"; callPromiseChart()}}>
                                                    W1    
                                                </button>
                                            {/if}
                                        {/if}
                                    </p></td>
                                </tr>
                            </table>
                        </div>
                        {#if resRecdChart == true}
                            <BasicCandleChart height={400} width={750} data={chartData} chartDecimals={chartDecimals} darkMode={isDarkMode}/>
                            <p class="text-xs">Created using <a href="https://github.com/tradingview/lightweight-charts" target="_blank">Lightweight-charts</a></p>
                        {:else}
                            <Loading/>
                        {/if}
                    </div>
        
                <!-- STATS SIDEBAR -->
                <div class=" w-full lg:w-1/3 p-1">
                    <p>{token} Stats :</p>
                    <div class="flex flex-col gap-4">
                        <!-- side 1 -->
                        <div class="flex-1">
                            <div class="flex flex-row sm:flex-col gap-4">
                                <div class="bg-tertiary-400/40 flex-1 rounded p-2">
                                    <table style="width: 100%;">
                                        <tr>
                                            <td>
                                                <p>24h Change: {#if change24 >= 0} 
                                                    <span class="text-success-600">{change24} %</span> 
                                                    {:else} <span class="text-warning-500">{change24} %</span>
                                                    {/if}</p>
                                                <p>7day Change: {#if change7d >= 0} 
                                                    <span class="text-success-600">{change7d} %</span> 
                                                    {:else} <span class="text-warning-500">{change7d} %</span>
                                                    {/if}</p>
                                            </td>
                                            <td>
                                                <TokenLogos token={token} width={"50px"} logoStyle={"flex justify-end p-1"}/>
                                            </td>
                                        </tr>
                                    </table>
                                    <p>Marketcap: {marketcap}</p>
                                    <p>Supply: {supply} {token}</p>
                                </div>
                                {#if token != "ICP"}
                                    <div class="bg-tertiary-400/40 flex-1 rounded p-2">
                                        {#if mpLoaded == true}
                                            <table class="w-full">
                                            <tr>
                                                <td>Marketplace</td>
                                                <td>Bid</td>
                                                <td>Ask</td>
                                                <td>Spread</td>
                                            </tr>
                                            {#each mpDataAR as mkt}
                                                <tr>
                                                    <td>{mkt.marketplace}</td>
                                                    <td>{mkt.bid}</td>
                                                    <td>{mkt.ask}</td>
                                                    <td>{mkt.spread}</td>
                                                </tr>
                                            {/each}
                                            </table>
                                        {:else}
                                            <div class="p-2">
                                                <Loading/>
                                                <p class="text-center">Fetching data..</p>
                                            </div>
                                        {/if}
                                        <p>
                                            Quote Currency: 
                                            {#if mpCurrency == "ICP"}
                                                <button class="bg-warning-600/80 p-1 rounded mt-1 ml-2" on:click={() => {mpCurrency="ICP"; changeMktCurrency();}}> ICP </button>
                                                <button class="bg-tertiary-500/50 p-1 rounded mt-1 ml-2" on:click={() => {mpCurrency="USD"; changeMktCurrency();}}> USD </button>
                                            {:else}
                                                <button class="bg-tertiary-500/50 p-1 rounded mt-1 ml-2" on:click={() => {mpCurrency="ICP"; changeMktCurrency();}}> ICP </button>
                                                <button class="bg-warning-600/80 p-1 rounded mt-1 ml-2" on:click={() => {mpCurrency="USD"; changeMktCurrency();}}> USD </button>
                                            {/if}
                                        </p>
                                    </div>
                                {/if}
                            </div> 
                        </div>
                        <!-- side 2 -->
                        <div class="flex-1">
                            <div class="flex flex-row md:flex-col gap-4">
                                <div class="bg-secondary-400/40 flex-1 rounded p-2">
                                    <p>Unique accounts: {uniqueACS}</p>
                                    {#if uniquePRS != "0"}<p>Unique principals: {uniquePRS}</p>{/if}

                                </div>
                                <div class="flex-1 bg-secondary-400/40 text-center p-2 rounded">
                                    <a href="/blocks/{token}">
                                        <button class="bg-warning-600/80 p-1 rounded mt-1"> {token} Block Explorer </button>
                                    </a>
                                    <a href="{tradeUrl}" target="_blank">
                                        <button class="bg-warning-600/80 p-1 rounded mt-1 ml-1 mr-1"> Marketplace </button>
                                    </a>
                                    <a href="{teamWebsite}" target="_blank">
                                        <button class="bg-warning-600/80 p-1 rounded mt-1"> Project Website </button>
                                    </a>
                                </div>
                            </div> 
                        </div>
                    </div>
                </div>
            </div>

            <!-- 2nd Section -->
            <div class="w-full flex flex-col lg:flex-row gap-2 mt-3">
                <div class="p-2 rounded lg:w-1/2 w-full bg-tertiary-400/40">
                    <div>
                        MINT/ BURN/ TRANSFER 
                        <span class="p-5"></span>
                        {#if mbtToggle == "hr"}
                            <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => {mbtToggle="hr"}}>
                                {hours}hr
                            </button>
                            <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {mbtToggle="dy"}}>
                                {days}days
                            </button>
                        {/if}
                        {#if mbtToggle == "dy"}
                            <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {mbtToggle="hr"}}>
                                {hours}hr
                            </button>
                            <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => {mbtToggle="dy"}}>
                                {days}days
                            </button>
                        {/if}
                    </div>
    
                    <div class="flex flex-row gap-2">
                        <div class="flex-1 rounded p-1">
                            {#if resRecd == true}
                                {#if mbtToggle == "hr"}
                                    <div width=500px class="mb-2">
                                        <MintBurnDonutChart dataArray={hourlyMBT} labelsArray={["Mint", "Burn", "Transfer"]} token={token}/>
                                    </div>
                                {/if}
                                {#if mbtToggle == "dy"}
                                <div width=500px class="mb-2">
                                    <MintBurnDonutChart dataArray={dailyMBT} labelsArray={["Mint", "Burn", "Transfer"]} token={token}/>
                                </div>
                            {/if}
                            {/if}
                        </div>
                        <div class="flex-1 rounded p-1">
                            {#if mbtToggle == "hr"}
                                <h3 class="h3 text-center">{hours}hr Stats</h3>
                                <hr class="pt-2">
                                <div class="ml-5">
                                    <p class="p-2">Mint: <span class="p-5"></span> {hourlyMBTformat[0]} </p>
                                    <p class="p-2">Burn: <span class="p-5"></span> {hourlyMBTformat[1]} </p>
                                    <p class="p-2">Transfer: <span class="p-2"></span> {hourlyMBTformat[2]} </p>
                                    <!-- <p class="p-3">Transfer <span class="p-3"></span> {hourlyMBT[2]} {token}</p> -->
                                </div>
                                <div class="pt-3">
                                    <p class="text-center text-warning-500">Change in Supply</p>
                                    <hr class="pt-2">
                                    <p class="text-center text-lg"> {#if supplyChangeHR > 0}+{/if}{supplyChangeHR}% Total {token}</p>
                                </div>
                            {/if}
                            {#if mbtToggle == "dy"}
                                <h3 class="h3 text-center">{days}days Stats</h3>
                                <hr class="pt-2">
                                <div class="ml-5">
                                    <p class="p-2">Mint: <span class="p-5"></span> {dailyMBTformat[0]} </p>
                                    <p class="p-2">Burn: <span class="p-5"></span> {dailyMBTformat[1]} </p>
                                    <p class="p-2">Transfer: <span class="p-2"></span> {dailyMBTformat[2]} </p>
                                    <!-- <p class="p-3">Transfer <span class="p-3"></span> {hourlyMBT[2]} {token}</p> -->
                                </div>
                                <div class="pt-3">
                                    <p class="text-center text-warning-500">Change in Supply</p>
                                    <hr class="pt-2">
                                    <p class="text-center text-lg"> {#if supplyChangeHR > 0}+{/if}{supplyChangeDY}% Total {token}</p>
                                </div>
                            {/if}
                        </div>
                    </div> 

                </div>
                <div class="p-2 rounded lg:w-1/2 w-full bg-tertiary-400/40">
                    <div>
                        TRANSACTION COUNT
                        <span class="p-5"></span>
                        {#if totToggle == "hr"}
                            <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => {totToggle="hr"}}>
                                {hours}hr
                            </button>
                            <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {totToggle="dy"}}>
                                {days}days
                            </button>
                        {/if}
                        {#if totToggle == "dy"}
                            <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {totToggle="hr"}}>
                                {hours}hr
                            </button>
                            <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => {totToggle="dy"}}>
                                {days}days
                            </button>
                        {/if}
                    </div>
                    {#if resRecd == true}
                        {#if totToggle == "hr"}
                            <div class="flex-1 mr-2">
                                <BarChart dataArray={hourlyTOTvalues} labelsArray={hourlyTOTlabels}/>
                            </div>
                        {/if}
                        {#if totToggle == "dy"}
                            <div class="flex-1 mr-2">
                                <BarChart dataArray={dailyTOTvalues} labelsArray={dailyTOTlabels}/>
                            </div>
                        {/if}
                    {/if}
                </div>
            </div>

            <!-- Largest Transactions -->
            <div class="w-full p-2 mt-3">
                {#if resRecdOnce == true}
                    <div class="w-full flex flex-row gap-2">
                        <div class="flex p-1"> <h3 class="h4 pl-2">Top Mint, Burn and Transfers</h3> </div>
                        <div class="flex p-1">
                            <!--     let topMbtDisplay = "tx";
                                     let topMbtTimeDisplay = "dy"; -->
                            {#if topMbtTimeDisplay == "dy"}
                                <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1 float" on:click={() => {topMbtTimeDisplay="dy"}}>
                                    {days}days
                                </button>
                                <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1 float" on:click={() => {topMbtTimeDisplay="hr"}}>
                                    {hours}hr
                                </button>
                            {:else if topMbtTimeDisplay == "hr"}
                                <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1 float" on:click={() => {topMbtTimeDisplay="dy"}}>
                                    {days}days
                                </button>
                                <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1 float" on:click={() => {topMbtTimeDisplay="hr"}}>
                                    {hours}hr
                                </button>
                            {/if}

                            <span class="p-2"> | </span>

                            {#if topMbtDisplay == "tx"}
                                <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1 float" on:click={() => {topMbtDisplay="tx"}}>
                                    Transfers
                                </button>
                                <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1 float" on:click={() => {topMbtDisplay="mt"}}>
                                    Mint
                                </button>
                                <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1 float" on:click={() => {topMbtDisplay="bn"}}>
                                    Burn
                                </button>
                            {:else if topMbtDisplay == "mt"}
                                <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1 float" on:click={() => {topMbtDisplay="tx"}}>
                                    Transfers
                                </button>
                                <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1 float" on:click={() => {topMbtDisplay="mt"}}>
                                    Mint
                                </button>
                                <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1 float" on:click={() => {topMbtDisplay="bn"}}>
                                    Burn
                                </button>
                            {:else if topMbtDisplay == "bn"}
                                <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1 float" on:click={() => {topMbtDisplay="tx"}}>
                                    Transfers
                                </button>
                                <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1 float" on:click={() => {topMbtDisplay="mt"}}>
                                    Mint
                                </button>
                                <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1 float" on:click={() => {topMbtDisplay="bn"}}>
                                    Burn
                                </button>
                            {/if}
                        </div>
                    </div>
                    <div class="w-full">
                        {#if topMbtTimeDisplay == "dy" && topMbtDisplay == "tx"}
                            <TopMintBurnTx sourceData={dTopTX} decimals={tokenDecimals} token={token} initNumItems={10}/>
                        {:else if topMbtTimeDisplay == "dy" && topMbtDisplay == "mt"}
                            <TopMintBurnTx sourceData={dTopMint} decimals={tokenDecimals} token={token} initNumItems={10}/>
                        {:else if topMbtTimeDisplay == "dy" && topMbtDisplay == "bn"}
                            <TopMintBurnTx sourceData={dTopBurn} decimals={tokenDecimals} token={token} initNumItems={10}/>
                        {:else if topMbtTimeDisplay == "hr" && topMbtDisplay == "tx"}
                            <TopMintBurnTx sourceData={hTopTX} decimals={tokenDecimals} token={token} initNumItems={10}/>
                        {:else if topMbtTimeDisplay == "hr" && topMbtDisplay == "mt"}
                            <TopMintBurnTx sourceData={hTopMint} decimals={tokenDecimals} token={token} initNumItems={10}/>
                        {:else if topMbtTimeDisplay == "hr" && topMbtDisplay == "bn"}
                            <TopMintBurnTx sourceData={hTopBurn} decimals={tokenDecimals} token={token} initNumItems={10}/>
                        {/if}

                    </div>
                {/if}
            </div>

            <!-- Top Holders -->
            {#if token != "ICP"}
                <div class="w-full p-2 rounded mt-3 ">
                    {#if resRecdOnce == true}
                        <div class="w-full flex flex-row gap-2">
                            <div class="flex p-1"> <h3 class="h4 pl-2">Top {token} Holders</h3> </div>
                            <div class="flex p-1">
                                {#if topHoldersDisplay == "ac"}
                                <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1 float" on:click={() => {topHoldersDisplay="ac"}}>
                                    Accounts
                                </button>
                                <!-- {#if hasPrincipalHolders == true}
                                    <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {topHoldersDisplay="pr"}}>
                                        Principals
                                    </button>
                                {/if} -->
                                {/if}
                                {#if topHoldersDisplay == "pr" && hasPrincipalHolders == true}
                                    <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {topHoldersDisplay="ac"}}>
                                        Accounts
                                    </button>
                                    <!-- <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => {topHoldersDisplay="pr"}}>
                                        Principals
                                    </button> -->
                                {/if}
                            </div>
                        </div>
                        <div class="w-full">
                            <TopHolders sourceData={topAcHolders} tokenSupply={supplyRaw} decimals={tokenDecimals} token={token}/>

                            <!-- removed until principal top holders fixed! 
                                {#if topHoldersDisplay == "ac"}
                                <TopHolders sourceData={topAcHolders} tokenSupply={supplyRaw} decimals={tokenDecimals} token={token}/>
                            {:else}
                                <TopHolders sourceData={topPrHolders} tokenSupply={supplyRaw} decimals={tokenDecimals} token={token}/>
                            {/if} -->
                        </div>
                    {/if}
                </div>
            {/if}

            <!-- Links -->
            <!-- <div class="w-full p-2 rounded flex mt-3 bg-tertiary-400/40">
                <p>LINKS ETC</p>
            </div> -->
        </div>
	</span>

	<span slot="foot">
		<Foot/>
	</span>
</LayoutCombine>


<style>

</style>