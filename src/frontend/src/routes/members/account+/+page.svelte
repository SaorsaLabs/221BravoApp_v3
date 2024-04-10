<script>
    import LayoutCombine from "../../../lib/componants/shared/layoutCombine.svelte";
	import Head from "../../../lib/componants/head/head.svelte";
	import HeadStats from "../../../lib/componants/head/headStats.svelte";
	import Foot from "../../../lib/componants/foot/foot.svelte";
    import {_slugData} from './+page';
    import { onMount } from "svelte";
    import { shortenString, processPromises, millisToDate, millisToTime } from '../../../lib/code/utils.js';
    import { Table, tableMapperValues, Paginator } from '@skeletonlabs/skeleton';
    import { getAllQuotes, getLatestICPprice } from '../../../lib/code/fetch/priceData.js';
    import { getNFTcount, nftGeekURLConstructor } from '../../../lib/code/fetch/nftData.js';
    import { addNamedAccounts } from '../../../lib/code/fetch/namedAccounts.js';
    import { searchAllTokens } from '../../../lib/code/fetch/searchAllTokens.js';
    import Loading from "../../../lib/componants/shared/loading.svelte";
    import { processMultiAccountTX } from '../../../lib/code/process/accountTx.js';
    import AccountTxTable from "../../../lib/componants/tables/accountTxTable.svelte";

    let urlAccountParam = _slugData.id;
    const formatter = new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD'
    });
    const formatNumber0dp = new Intl.NumberFormat('en-US', {
        maximumFractionDigits: 0,
    });
    const numFormater = new Intl.NumberFormat('en-US');
    let searchingInfoText = "Searching all token ledgers...";
    let tokenOverviewOutput;
    let i,k;
    let warnText;
    let showWarning = false;
    let searchComplete = false;
    let searchActive = false;
    let tokenOverview = []; // Result table data
    let transactionData = [];
    let rmWhitespace; // search input with whitespace removed.
    let accountFirstActive;
    let accountLastActive;
    let fmtTotalDollar;
    let totalSentCount;
    let totalRecCount;
    let totalCount;
    let tableDataFuture; 
    let activeFmDate;
    let activeFmTime;
    let lastActDate;
    let lastActTime;
    let nftCount;
    let nftGeekLink = "";
    let searchInput;
    let displayAC = "";
    let tokenOverviewLen = 0;
    let overviewPaginationSettings = {
                    page: 0,
                    limit: 5,
                    size: 0,
                    amounts: [5,10],
                };
    let userName;
    let globalName;

    onMount(()=>{
        if (urlAccountParam != null){
            searchInput = urlAccountParam;
            searchClick();
        }
    });

    async function searchClick(){
        clearClick();// clear arrays etc.
        searchComplete = false;
        searchActive = true;
        tableDataFuture = new Promise(async (resolve, reject) => {
            // check for blanks/ errors
            if (searchInput == "") {
                warnText = "Search cannot be empty!"
                showWarning = true;
            }
            else if (searchInput.length >= 150) {
                warnText = "Input too long!"
                showWarning = true;
            }
            else if ((searchInput.length < 9)) {
                warnText = "Invalid input!"
                showWarning = true; 
            }
            rmWhitespace = searchInput.replace(/\s/g, "");

            let futuresAR = [];
            futuresAR[0] = searchAllTokens(rmWhitespace);
            futuresAR[1] = getAllQuotes("USD");
            futuresAR[2] = getNFTcount(rmWhitespace);
            let bData = [{from_account: rmWhitespace, to_account: rmWhitespace}];
            futuresAR[3] = addNamedAccounts(bData);
            let futuresRes = await processPromises(futuresAR);
            let searchResult = futuresRes[0]; 
            let priceData = futuresRes[1];
            nftGeekLink = nftGeekURLConstructor(rmWhitespace);
            searchingInfoText = "Processing Results...";

            // catch nothing found 
            let searchResultLen = searchResult?.length ?? 0;
            let emptyCount = 0;
            for(i=0; i<searchResultLen; i++){
                if (searchResult[i].data.length == 0){
                    emptyCount++;
                }
            }
            if (emptyCount == searchResultLen){
                warnText = "No Results ☹️"
                showWarning = true; 
                resolve('Promise resolved!');
            }
  
            // format display
            let stLen = rmWhitespace.length ?? 0;
            userName = futuresRes[3][0]?.fromUserName ? futuresRes[3][0].fromUserName : "";
            globalName = futuresRes[3][0]?.fromGlobalName ? futuresRes[3][0].fromGlobalName : "";
            if (stLen > 70) {
                displayAC = shortenString(rmWhitespace, 40, 30);
            } else {
                displayAC = rmWhitespace;
            }

            // add value to each token
            let searchLen = searchResult.length ?? 0;
            let priceLen = priceData.length ?? 0;
            let matchTerm, ipcPrice; 
            for(i = 0; i< searchLen; i++){
                // catch ICP and fetch price
                if (searchResult[i].ticker == "ICP"){
                    ipcPrice = await getLatestICPprice();
                    searchResult[i].tokenPrice = ipcPrice;
                }
                // other tokens
                for(k=0; k<priceLen; k++){
                    matchTerm = `${searchResult[i].ticker}/`;
                    if (priceData[k].token_cross.includes(matchTerm)){
                        searchResult[i].tokenPrice = priceData[k].average_price;
                    }
                }
            }

            // Populate tokenOverview and Sort
            let sd;
            accountFirstActive = 9999999999999999999n;
            accountLastActive = 0n;
            totalSentCount = 0;
            totalRecCount = 0;
            totalCount = 0;
            nftCount = futuresRes[2];
            let accountTotalDollar = 0;
            let adjBalance;
            let adjPrice;
            let adjPriceRaw;
            let sum1, sum0;
            for(i = 0; i< searchLen; i++){
                if (searchResult[i]?.data.length > 0) {
                    // balances
                    sum0 = isNaN(Number(searchResult[i].data[0].overview.balance)/ Math.pow(10, searchResult[i].decimals)) ? 0 : Number(searchResult[i].data[0].overview.balance)/ Math.pow(10, searchResult[i].decimals);
                    adjBalance = sum0;
                    sum1 = isNaN(adjBalance*searchResult[i].tokenPrice) ? 0 : adjBalance*searchResult[i].tokenPrice;
                    accountTotalDollar += sum1
                    adjPrice = formatter.format(sum1);
                    adjPriceRaw = sum1;
                    // tx counts
                    totalSentCount += searchResult[i].data[0].overview.sent[0];
                    totalRecCount += searchResult[i].data[0].overview.received[0];
                    // first active
                    if (searchResult[i].data[0].overview.first_active < accountFirstActive) {
                        accountFirstActive = searchResult[i].data[0].overview.first_active;
                    }
                    if (searchResult[i].data[0].overview.last_active > accountLastActive) {
                        accountLastActive = searchResult[i].data[0].overview.last_active;
                    }
                    // update table data
                    sd = {name: searchResult[i].shortName, balance: adjBalance, dollar: adjPrice, rawDollar: adjPriceRaw, ticker: searchResult[i].ticker}
                    tokenOverview.push(sd);
                }//if
            }//for

            // sort Overview Table data
            tokenOverview.sort(function(a,b){return b.rawDollar - a.rawDollar});
            totalCount = totalSentCount + totalRecCount;
            fmtTotalDollar = formatter.format((accountTotalDollar));
            activeFmDate = millisToDate((Number(accountFirstActive)/1000000));
            activeFmTime = millisToTime((Number(accountFirstActive)/1000000));
            lastActDate = millisToDate((Number(accountLastActive)/1000000));
            lastActTime = millisToTime((Number(accountLastActive)/1000000));

            totalCount = numFormater.format(totalCount);
            totalSentCount = numFormater.format(totalSentCount);
            totalRecCount = numFormater.format(totalRecCount);
            tokenOverviewLen = tokenOverview.length ?? 0;

            overviewPaginationSettings = {
                    page: 0,
                    limit: 10,
                    size: tokenOverviewLen,
                    amounts: [5,10,20],
            };

            // PROCESS TRANSACTION DATA
            let tempTXS = [];
            let blockLen;
            for(i = 0; i< searchLen; i++){
                blockLen = searchResult[i].data[0]?.blocks.length ?? 0;
                if ( blockLen > 0) {
                    
                   tempTXS.push(processMultiAccountTX(searchResult[i].data[0].blocks, searchResult[i].searchedAC, searchResult[i].ticker, searchResult[i].decimals));
                }
            }
            // combine all txs
            let tempTXSLen = tempTXS.length ?? 0;
            let txsCount; 
            for(i=0; i<tempTXSLen; i++){
                txsCount = tempTXS[i].length ?? 0;
                for(k=0; k<txsCount; k++){
                    transactionData.push(tempTXS[i][k]);
                }
            }            
            // sort Transactions by time
            transactionData.sort(function(a,b){return Number(b.time) - Number(a.time)});
            // fix transaction numbers
            let newCount = 1;
            let transactionDataLen = transactionData.length ?? 0;
            for(i=0; i<transactionDataLen; i++){
                transactionData[i].count = newCount;
                newCount++;
            }

            searchActive = false;
            searchComplete = true;
            resolve('Promise resolved!');
        });
    }

    function clearClick(){
        showWarning = false;
        warnText = "";
        displayAC = "";
        tokenOverview = [];
        transactionData = [];

    }

    $: overviewPaginationSettings;
    $: tokenOverviewSliced = tokenOverview.slice(
		overviewPaginationSettings.page * overviewPaginationSettings.limit,
		overviewPaginationSettings.page * overviewPaginationSettings.limit + overviewPaginationSettings.limit
	);
    $: tokenOverviewOutput = tableMapperValues(tokenOverviewSliced, ['name', 'balance', 'dollar']);
</script>

<svelte:head>
	<title>Account+ Multi-Token</title>
</svelte:head>

<LayoutCombine>
	<span slot="headStats">
		<HeadStats/>
	</span>
	<span slot="head">
		<Head/>
	</span>
	<span slot="body" class="pb-4">
        <div class="container mx-auto px-4 flex flex-wrap min-w-full md:min-w-0">
            <a href="/">Home </a>/ <a href="/members/home">Members</a>/ Account+
        </div>
        <!-- First Section -->
        <div class="container mx-auto px-4 py-5 flex flex-wrap min-w-full md:min-w-0 content-center justify-center items-center">

            <!-- search bar -->
            <div class="w-full dark:bg-primary-500/40 bg-primary-700 rounded p-1 mb-4 flex content-center justify-center items-center"> 
                <input class="input pl-2 w-10/12" title="Token (Ticker or name)" type="text" placeholder="Search Account/ Principal/ ICRC Account" bind:value={searchInput}/>
                <button class="bg-primary-500/50 rounded pl-2 pr-2 ml-2 " on:click={() => {searchClick()}}>Search</button>  
            </div>

            <!-- token overviews -->
            <div class="w-full pb-3"> 
                {#if searchComplete == true}
                    <!-- Overview Stats -->
                    <div class="flex flex-col lg:flex-row gap-4 w-full">
                        <div class="flex flex-row gap-4 w-full lg:w-3/5">
                            <div class="p-1 bg-surface-400/40 flex-1 w-1/5 rounded">
                                <p class="pl-1 dark:text-primary-500 text-error-600 text-lg">Sent</p>
                                    <p class="pl-1 text">{totalSentCount} txs</p>
                            </div>
                            <div class="p-1 bg-surface-400/40 flex-1 w-1/5 rounded">
                                <p class="pl-1 dark:text-primary-500 text-error-600 text-lg">Received</p>
                                    <p class="pl-1 text">{totalRecCount} txs</p>
                            </div>
                            <div class="p-1 bg-primary-400/40 flex-1 w-1/5 rounded">
                                <p class="pl-1 dark:text-primary-500 text-error-600 text-lg">Balance</p>
                                    <p class="pl-1 text-lg">{fmtTotalDollar}</p>
                                    <p class="pl-1 text">{totalCount} txs</p>
                            </div>
                        </div>
                        <div class="flex flex-row gap-4 w-full lg:w-2/5">
                            <div class="p-1 bg-surface-400/40 flex-1 w-full lg:w-1/5 rounded">
                                <p class="pl-1 dark:text-primary-500 text-error-600 text-lg">First Active</p>
                                    <p class="pl-1 text-lg">{activeFmDate}</p>
                                    <p class="pl-1 text">{activeFmTime}</p>
                            </div>
                            <div class="p-1 bg-surface-400/40 flex-1 w-full lg:w-1/5 rounded">
                                <p class="pl-1 dark:text-primary-500 text-error-600 text-lg">Last Active</p>
                                    <p class="pl-1 text-lg">{lastActDate}</p>
                                    <p class="pl-1 text">{lastActTime}</p>
                            </div>
                        </div>
                    </div>
        
                    <!-- Token Balance Table -->
                    <div class="w-full pt-4">
                        <Table 
                        source={
                            {head: ['Token', 'Balance', 'Value $'], 
                            body: tokenOverviewOutput }} 
                        interactive={false} 
                        regionHeadCell="bg-tertiary-400/40"
                        />
                        <div class="mt-2">
                            <Paginator
                                bind:settings={overviewPaginationSettings}
                                showFirstLastButtons="{true}"
                                showPreviousNextButtons="{true}"
                            />
                        </div>
                    </div>

                    <!-- all transactions Table -->
                    <div class="w-full pt-4">
                        <div><p class="h4">Transactions:</p></div>  
                        <div><AccountTxTable sourceData={transactionData} decimals={0} searchedAC={searchInput} multiTokenView={true}/></div>
                    </div>

                <!-- Init State/ Searching Active -->
                {:else}
                <div class="w-full min-h-96">
                    {#if searchActive == false}
                        <div class="w-full pt-3">
                            See all SNS/ CK/ Top Token transactions in one place!
                        </div>
                    {:else}
                        <Loading/>
                        <p class="text-center pt-3">{searchingInfoText}</p>
                    {/if}
                </div>
                {/if}
            </div>
        </div>
	</span>

	<span slot="foot">
		<Foot/>
	</span>
</LayoutCombine>