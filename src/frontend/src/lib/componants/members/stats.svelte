<script>
import { onMount, onDestroy } from 'svelte';
import { fetchExchangeTrackingData, fetchDexTrackingData } from '../../code/fetch/trackingData.js';
import { nanoToDate } from '../../code/utils.js';
import LineChart from '../charts/lineChart.svelte';
import BarChart from '../charts/barChart.svelte';


let isDarkMode = false;
let valuesAR1 = [];
let labelsAR1 = [];
let dexV1 = [];
let dexV2 = [];
let dexV3 = [];
let dexLabels = [];
let changeV1 = [];
let changeV2 = [];
let changeV3 = [];

onMount(()=>{
    //fetchData();
    // [][] for switching icons light/ dark [][]
    const updateMode = () => {
        valuesAR1 = [];
        labelsAR1 = [];
        dexV1 = [];
        dexV2 = [];
        dexV3 = [];
        dexLabels = [];
        changeV1 = [];
        changeV2 = [];
        changeV3 = [];
        showChart = false;
        isDarkMode = document.documentElement.classList.contains('dark');
        fetchData();
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



let tm;
let showChart = false;
let showDexBarChart = false;
async function fetchData(){
    // EXCHANGES
    let data = await fetchExchangeTrackingData("ICP");
    let dataLen = data?.length ?? 0;
    let snapLen, k; 
    let sum;
    for(let i=0; i<dataLen; i+=3){ // +=3 to make 12 hourly
        snapLen = data[i].ExchangeCollection.exchange_snapshots.length ?? 0;
        sum = 0;
        for(k=0; k<snapLen; k++){
            sum += Number(data[i].ExchangeCollection.exchange_snapshots[k].balance)/100000000;
        }
        valuesAR1.push(sum);
        tm = nanoToDate(Number(data[i].ExchangeCollection.snapshot_time));
        labelsAR1.push(tm.dateOnly);
    }
    // DEXES
    let dex = await fetchDexTrackingData("ICP");
    let dexLen = dex?.length ?? 0;
    let tm2;
    let v1, v2, v3;
    
    for(let m = 0; m<dexLen; m+=6){ // +6 to make it daily
        v1 = Number(dex[m].ExchangeCollection.exchange_snapshots[0].balance/100000000n);
        v2 = Number(dex[m].ExchangeCollection.exchange_snapshots[1].balance/100000000n);
        v3 = Number(dex[m].ExchangeCollection.exchange_snapshots[2].balance/100000000n);
        dexV1.push(v1);
        dexV2.push(v2);
        dexV3.push(v3);
        tm2 = nanoToDate(Number(dex[m].ExchangeCollection.snapshot_time));
        dexLabels.push(tm2.dateOnly);
    }
    let resLen = dexV1.length;
    for(let m = 1; m<resLen; m++){
        changeV1[m] = dexV1[m] - dexV1[m-1];
        changeV2[m] = dexV2[m] - dexV2[m-1];
        changeV3[m] = dexV3[m] - dexV3[m-1];
    }
    // remove 1st bar
    changeV1.shift();
    changeV2.shift();
    changeV3.shift();
    dexLabels.shift();
    showChart = true;
}

$: isDarkMode;
</script>

<div class="flex flex-col xl:flex-row gap-4 pr-2">
    <div class="bg-primary-400/40 flex-1 rounded p-1 pl-2">
        Total Exchange Balance (ICP)
        {#if showChart == true}
            <LineChart dataArray4={valuesAR1} labelsArray={labelsAR1} dataset4Title="All Exchanges" darkMode={isDarkMode}/>
        {/if}
    </div>
    <div class="bg-primary-400/40 flex-1 rounded p-1 pl-2">
        {#if showChart == true}
            {#if showDexBarChart == true}

            Dex Balance Change (ICP) 
                <span class="pl-2">
                    {#if showDexBarChart == true}
                        <!-- <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => {showDexBarChart = true}}>Bar Chart</button> -->
                        <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {showDexBarChart = false}}>Line Chart</button>
                    {:else}
                        <!-- <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {showDexBarChart = true}}>Bar Chart</button> -->
                        <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => {showDexBarChart = false}}>Line Chart</button>
                    {/if}
                </span>
                <BarChart 
                    dataArray={changeV1}
                    dataArray2={changeV2} 
                    dataArray3={changeV3} 
                    labelsArray={dexLabels}
                    datasetTitle="Helix"
                    dataset2Title="ICP Swap"
                    dataset3Title="Sonic"
                    showTitle={true}
                    darkMode={isDarkMode}
                />  
            {:else}
            Dex Total Balances (ICP)
                <span class="pl-2">
                    {#if showDexBarChart == true}
                        <!-- <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => {showDexBarChart = true}}>Bar Chart</button> -->
                        <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {showDexBarChart = false}}>Line Chart</button>
                    {:else}
                        <!-- <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {showDexBarChart = true}}>Bar Chart</button> -->
                        <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => {showDexBarChart = false}}>Line Chart</button>
                    {/if}
                </span>
                <LineChart 
                    dataArray1={dexV1}
                    dataArray2={dexV2}  
                    dataArray3={dexV3}
                    dataset1Title="Helix"
                    dataset2Title="ICP Swap"
                    dataset3Title="Sonic"
                    labelsArray={dexLabels}
                    darkMode={isDarkMode}
                />
            {/if}
        {/if}
    </div>
</div>