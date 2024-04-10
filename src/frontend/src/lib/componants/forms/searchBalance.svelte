<script>
    import SearchIcon from '$lib/images/icons/search.png';
	import HelpIcon from '$lib/images/icons/help.png';
    import HelpIconLight from '$lib/images/icons/helpLight.png';
    import searchHelpModal from '../modal/searchHelp.svelte';
    import AccountView from '../modal/accountView.svelte';
    import { getModalStore} from '@skeletonlabs/skeleton';
    import { Table, tableMapperValues, Paginator } from '@skeletonlabs/skeleton';
    import { onMount } from 'svelte';
    import { getOverviewDataAllTokens } from '../../code/fetch/overviewAllTokens.js';
    import Loading from '../../componants/shared/loading.svelte';
    import { shortenString, processPromises, millisToDate, millisToTime } from '../../code/utils';
    import { getAllQuotes, getLatestICPprice } from '../../code/fetch/priceData.js';
    import { getNFTcount, nftGeekURLConstructor } from '../../code/fetch/nftData.js';
    import { addNamedAccounts } from '../../code/fetch/namedAccounts.js';
    import SaveButton from '../shared/saveButton.svelte';
    import CopyButton from '../shared/copyButton.svelte';

    // for popup
    const modalStore = getModalStore();

    // Light/ Dark switch help icon
    let isDarkMode = false;
    let isSearchActive = false;
    let showWarning = false;
    let warnText = "";

    onMount(() => {
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

    // ON SEARCH CLICK
    let sourceData = []; // Result table data
    let rmWhitespace; // search input with whitespace removed.
    let accountFirstActive;
    let accountLastActive;
    let fmtTotalDollar;
    let fmtSentDollar;
    let fmtRecDollar;
    let totalSentCount;
    let totalRecCount;
    let totalCount;
    let totalSentValue;
    let totalRecValue;
    let tableDataFuture; 
    let activeFmDate;
    let activeFmTime;
    let lastActDate;
    let lastActTime;
    let nftCount;
    let nftGeekLink = "";
    let searchInput = "";
    let displayAC = "";
    let sourceDataLen = 0;
    let paginationSettings = {
                    page: 0,
                    limit: 5,
                    size: 0,
                    amounts: [5,10],
                };
    let userName;
    let globalName;

    const formatter = new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD'
    });
    const numFormater = new Intl.NumberFormat('en-US');

    async function searchClick(){
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
            isSearchActive = true;
            rmWhitespace = searchInput.replace(/\s/g, "");

            let futuresAR = [];
            futuresAR[0] = getOverviewDataAllTokens(rmWhitespace);
            futuresAR[1] = getAllQuotes("USD");
            futuresAR[2] = getNFTcount(rmWhitespace);
            let bData = [{from_account: rmWhitespace, to_account: rmWhitespace}];
            futuresAR[3] = addNamedAccounts(bData);
            let futuresRes = await processPromises(futuresAR);
            let searchResult = futuresRes[0]; 
            let priceData = futuresRes[1];
            nftGeekLink = nftGeekURLConstructor(rmWhitespace);
            // catch nothing found 
            if (searchResult.data.length == 0){
                warnText = "No Results"
                showWarning = true; 
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
            let i,k;
            let searchLen = searchResult.data.length ?? 0;
            let priceLen = priceData.length ?? 0;
            let matchTerm, ipcPrice; 
            for(i = 0; i< searchLen; i++){
                // catch ICP and fetch price
                if (searchResult.data[i].ticker == "ICP"){
                    ipcPrice = await getLatestICPprice();
                    searchResult.data[i].tokenPrice = ipcPrice;
                }
                // other tokens
                for(k=0; k<priceLen; k++){
                    matchTerm = `${searchResult.data[i].ticker}/`;
                    if (priceData[k]?.token_cross?.includes(matchTerm)){
                        searchResult.data[i].tokenPrice = priceData[k].average_price;
                    }
                }
            }

            // Populate sourceData and Sort
            let sd;
            accountFirstActive = 9999999999999999999n;
            accountLastActive = 0n;
            totalSentCount = 0;
            totalRecCount = 0;
            totalSentValue = 0;
            totalRecValue = 0;
            totalCount = 0;
            nftCount = 0;
            nftCount = futuresRes[2];
            let accountTotalDollar = 0;
            let accountSentDollar = 0; 
            let accountRecDollar = 0; 
            let adjBalance;
            let adjSentBalance;
            let adjRecBalance;
            let adjPrice;
            let adjPriceRaw;
            let sum1, sum2, sum3;
            let sum1A, sum2A, sum3A;
            let sum4, sum4A;
            let sum5, sum5A;
            let sum6, sum6A;
            for(i = 0; i< searchLen; i++){
                // balances
                sum1 = Number(searchResult.data[i].balance)/ Math.pow(10, searchResult.data[i].decimals);
                sum1A =isNaN(sum1) ? 0 : sum1;
                adjBalance = sum1A;
                sum2 = Number(searchResult.data[i].sent[1])/ Math.pow(10, searchResult.data[i].decimals);
                sum2A = isNaN(sum2) ? 0 : sum2;
                adjSentBalance = sum2A;
                sum3 = Number(searchResult.data[i].received[1])/ Math.pow(10, searchResult.data[i].decimals);
                sum3A = isNaN(sum3) ? 0 : sum3;
                adjRecBalance = sum3A;
                sum4 = adjBalance*searchResult.data[i].tokenPrice;
                sum4A = isNaN(sum4) ? 0 : sum4;
                accountTotalDollar += sum4A;
                sum5 = adjSentBalance*searchResult.data[i].tokenPrice;
                sum5A = isNaN(sum5) ? 0 : sum5;
                accountSentDollar += sum5A;
                sum6 = adjRecBalance*searchResult.data[i].tokenPrice;
                sum6A = isNaN(sum6) ? 0 : sum6;
                accountRecDollar += sum6A;
                adjPrice = formatter.format(sum4A);
                adjPriceRaw = sum4A;
                // tx counts
                totalSentCount += searchResult.data[i].sent[0];
                totalRecCount += searchResult.data[i].received[0];
                // first active
                if (searchResult.data[i].first_active < accountFirstActive) {
                    accountFirstActive = searchResult.data[i].first_active;
                }
                if (searchResult.data[i].last_active > accountLastActive) {
                    accountLastActive = searchResult.data[i].last_active;
                }
                // update table data
                sd = {name: searchResult.data[i].shortName, balance: adjBalance, dollar: adjPrice, rawDollar: adjPriceRaw, ticker: searchResult.data[i].ticker}
                sourceData.push(sd);
            }

            // sort Table data
            sourceData.sort(function(a,b){return b.rawDollar - a.rawDollar});

            totalCount = totalSentCount + totalRecCount;
            fmtTotalDollar = formatter.format((accountTotalDollar));
            fmtSentDollar = formatter.format((accountSentDollar));
            fmtRecDollar = formatter.format((accountRecDollar));
            activeFmDate = millisToDate((Number(accountFirstActive)/1000000));
            activeFmTime = millisToTime((Number(accountFirstActive)/1000000));
            lastActDate = millisToDate((Number(accountLastActive)/1000000));
            lastActTime = millisToTime((Number(accountLastActive)/1000000));

            totalCount = numFormater.format(totalCount);
            totalSentCount = numFormater.format(totalSentCount);
            totalRecCount = numFormater.format(totalRecCount);
            sourceDataLen = sourceData.length ?? 0;
            // 
            paginationSettings = {
                    page: 0,
                    limit: 5,
                    size: sourceDataLen,
                    amounts: [5,10],
                };
            resolve('Promise resolved!');
    
        });
    }

    function clearClick(){
        isSearchActive = false;
        searchInput = "";
        showWarning = false;
        warnText = "";
        displayAC = "";
        sourceData = [];
    }
    // HELP CLICK 
	function helpClick(){
		const c = { ref: searchHelpModal };
		const modal = {
			type: 'component',
			component: c,
			title: 'Custom Form Component',
			body: '',
			//response: (r) => console.log('response:', r)
		};
		modalStore.trigger(modal);
	}

    function accountView(token){
		const c = { ref: AccountView };
		const modal = {
			type: 'component',
			component: c,
			title: 'Custom Form Component',
			body: '',
            meta: { token, id: rmWhitespace },
			//response: (r) => console.log('response:', r)
		};
        modalStore.clear();
		modalStore.trigger(modal);
    }

    function onSelected(meta){
        accountView(meta.detail[0]);
	}

    $: paginationSettings = paginationSettings;
    $: sourceBodySliced = sourceData.slice(
		paginationSettings.page * paginationSettings.limit,
		paginationSettings.page * paginationSettings.limit + paginationSettings.limit
	);
    $: sourceOutput = tableMapperValues(sourceBodySliced, ['name', 'balance', 'dollar']);
    $: tickerOutput = tableMapperValues(sourceBodySliced, ['ticker']);

    function onPageChange(e) {
		//console.log('Paginator - event:page', e.detail);
	}
	function onAmountChange(e) {
		//console.log('Paginator - event:amount', e.detail);
	}
</script>

<!-- INPUT SECTION -->
<div class="w-full h-52" id="holder">
    <div class="max-w-3xl mx-auto pt-4">
        <h2 class="h2 pt-12">Search Accounts</h2>
        <div class="float">
            <p class="pb-2 text-secondary-400 dark:text-secondary-400">Top tokens on the Internet Computer 
                <button class="float-right pr-1"  on:click={() => {helpClick()}}>
                    <img src={isDarkMode ? HelpIconLight : HelpIcon} alt="Help Logo" width="20px"/>
                </button>
            </p>
        </div>
        <!-- Search Bar -->
        <div class="min-width-3/6">
            <div class="float input-group input-group-divider grid-cols-[auto_1fr_auto] text-lg">
                <div class="input-group-shim"><img src={SearchIcon} alt='Background' width="20px"/></div>
                <input type="search pl-2" placeholder=" Search by Principal/ Account/ Principal.Sub-Account"  bind:value={searchInput}/>
                {#if isSearchActive == false}	
                    <button class="variant-filled-secondary" on:click={()=>{searchClick()}}>Submit</button>
                {:else}
                    <button class="variant-filled-warning" on:click={()=>{clearClick()}}>Clear</button>
                {/if}
            </div>
            <!-- Output/ warning -->
            {#if showWarning == true}
                <div class="min-width-3/6">
                    <p class="text-warning-500">{warnText}</p>
                </div>
            {/if}
        </div>
    </div>
</div>

<!-- RESULTS -->
{#if isSearchActive == true && showWarning == false}
    {#await tableDataFuture}
        <div class="p-2"> <Loading/> </div>
    {:then}
        <div class="w-full flex flex-col lg:flex-row gap-2">
            <!-- BALANCES TABLE -->
            <div class="p-1 rounded lg:w-2/3 w-full">
                <p>Account: {#if userName != "" || globalName != ""}
                                {#if userName != ""}
                                    {displayAC} <CopyButton text={rmWhitespace}/> {@html "<br>"} <span class="text-primary-500">{userName}</span>
                                {:else}
                                    {displayAC} <CopyButton text={rmWhitespace}/> {@html "<br>"} <span class="text-primary-500">{globalName}</span>
                                {/if}
                            {:else}
                                {displayAC} <CopyButton text={rmWhitespace}/>
                            {/if}
                    
                </p>
                <Table 
                    source={
                        {head: ['Token', 'Balance', 'Value $'], 
                        body: sourceOutput, 
                        meta: tickerOutput}} 
                    interactive={true} 
                    on:selected={onSelected} 
                    regionHeadCell="bg-tertiary-400/40"
                />
                <div class="mt-2">
                    <Paginator
                        bind:settings={paginationSettings}
                        showFirstLastButtons="{true}"
                        showPreviousNextButtons="{true}"
                    />
                </div>
            </div>

            <!-- STATS SIDEBAR -->
            <div class="lg:w-1/3 w-full p-1">
                <p>Account Stats: </p>
                <div class="flex flex-col gap-4">
                    <!-- side 1 -->
                    <div class="flex-1">
                        <div class="flex flex-row md:flex-col gap-4">
                            <div class="bg-tertiary-400/40 flex-1 rounded p-1">
                                <!-- <p class="p-1 text-warning-500">Name: Unknown Account</p> -->
                                <!-- side 1 inner 1 -->
                                <div class="flex flex-row md:flex-col gap-1">
                                    <div class="flex-1 rounded p-1 text-xl">
                                        <span class="text-warning-500">Total Balance:</span> {fmtTotalDollar}
                                    </div>
                                    <div class="flex-1 rounded p-1">
                                        Total Transactions: {totalCount}
                                    </div>
                                </div>
                            </div>
                            <div class="bg-tertiary-400/40 flex-1 rounded p-1">
                                  <!-- side 1 inner 2-->
                                  <div class="flex flex-row md:flex-col gap-1">
                                    <div class="flex-1 rounded p-1">
                                        <p class="p-1">NFTs: {nftCount}</p>
                                        <a href={nftGeekLink} target="_blank">
                                            <button type="button" class="btn btn-sm variant-filled pt-1">Explore</button>
                                        </a>
                                    </div>
                                    <div class="flex-1 rounded p-1">
                                        <p class="text-xs"><a href="https://nftgeek.app" target="_blank">Powered by NFT Geek</a></p>
                                    </div>
                                </div>
                            </div>
                        </div> 
                    </div>
                    <!-- side 2 -->
                    <div class="flex-1">
                        <div class="flex flex-row md:flex-col gap-4">
                            <div class="bg-secondary-400/40 flex-1 rounded p-1">
                                <!-- side 2 inner 1 -->
                                <div class="flex flex-row md:flex-col gap-1">
                                    <div class="flex-1 rounded p-1">
                                        <p>Active From: {activeFmDate}</p>
                                        <p>{activeFmTime}</p>
                                    </div>
                                    <div class="flex-1 rounded p-1">
                                        <p>Last Active: {lastActDate}</p>
                                        <p>{lastActTime}</p>
                                    </div>
                                </div>
                            </div>
                            <div class="bg-secondary-400/40 flex-1 rounded p-1">
                                   <!-- side 2 inner 2 -->
                                   <div class="flex flex-row md:flex-col gap-1">
                                    <div class="flex-1 rounded p-1">
                                        <p>Total Sent: {fmtSentDollar}</p>
                                        <p>Sent txs: {totalSentCount}</p>
                                    </div>
                                    <div class="flex-1 rounded p-1">
                                        <p>Total Rec'd: {fmtRecDollar}</p>
                                        <p>Rec'd txs: {totalRecCount}</p>
                                    </div>
                                </div>
                            </div>
                        </div> 
                    </div>
                    <!-- side 3
                    <div class="flex-1">
                        <div class="flex flex-row md:flex-col gap-4">
                            <div class="bg-success-400/40 flex-1 rounded p-1">SUB 3/1</div>
                        </div> 
                    </div> -->
                </div>
            </div>
        </div>

        <div class="w-full flex flex-col lg:flex-row pt-4">
            <div class="border-t-4 border-secondary-300/50 dark:border-secondary-700/50 min-w-full p-5 border-dotted"></div>
        </div>

    {/await}
{/if}



