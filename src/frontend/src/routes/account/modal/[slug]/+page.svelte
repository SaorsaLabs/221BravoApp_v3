<script>
    import LayoutCombine from "../../../../lib/componants/shared/layoutCombine.svelte";
    import {_slugData} from './+page';
    import { onMount } from 'svelte';
    import Loading from '../../../../lib/componants/shared/loading.svelte';
    import { getFullFromID } from '../../../../lib/code/fetch/accountSearch.js';
    import { shortenString, nanoToDate } from '../../../../lib/code/utils.js';
    import { processAccountTX } from '../../../../lib/code/process/accountTx';
    import AccountTxTable from "../../../../lib/componants/tables/accountTxTable.svelte";
    import CopyButton from "../../../../lib/componants/shared/copyButton.svelte";
    import { addNamedAccounts } from '../../../../lib/code/fetch/namedAccounts.js';
    import SaveButton from "../../../../lib/componants/shared/saveButton.svelte";

    let token = _slugData.token;
    onMount(() => {
		loadData();
    });

    const formatNumber0dp = new Intl.NumberFormat('en-US', {
        maximumFractionDigits: 0,
    });

    let dataLoaded = false;
    let accountID = null;
    let resolved = null;
    let acIdFull = null;
    let warning = null;
    let decimalPower = 1;
    let decimals = 1;
    let sentTX = 0;
    let sentValue = 0;
    let recdTX = 0;
    let recdValue = 0;
    let totalTX = 0;
    let totalValue = 0;
    let fromConvert;
    let dateFrom = "";
    let timeFrom  = "";
    let toConvert;
    let dateTo = "";
    let timeTo = "";
    let txTableData = [];
    let searchedName;
    let displayName;

    async function loadData(){
        let idData = await getFullFromID(_slugData.id, token);
        if (idData.data == null || idData?.data.length == 0){
            warning = "Could not find any data!";
            if (_slugData.id?.length > 75){
                accountID = shortenString(_slugData.id,50,25);
            } else {
                accountID = _slugData.id;
            }
            resolved = (idData.resolved != null) ? shortenString(idData.resolved,40,10) : idData.resolved;
            dataLoaded = true;
            return;
        }
        acIdFull = (idData.resolved != null) ? idData.resolved : _slugData.id;
        decimals = idData.decimals;
        decimalPower = Math.pow(10,idData.decimals);
        resolved = (idData.resolved != null) ? shortenString(idData.resolved,40,10) : idData.resolved;
        if (_slugData.id?.length > 75){
                accountID = shortenString(_slugData.id,50,25);
            } else {
                accountID = _slugData.id;
        }

        // check for user/ global name for the searched account
        let blockData = [{from_account: acIdFull, to_account: acIdFull}];
        searchedName = await addNamedAccounts(blockData);
        if ( searchedName[0]?.fromGlobalName || searchedName[0]?.fromUserName ){
            if(searchedName[0]?.fromUserName){
                displayName = searchedName[0].fromUserName;
            } else {
                displayName = searchedName[0].fromGlobalName;
            }
        }

        sentTX = formatNumber0dp.format(idData.data[0]?.overview.sent[0]);
        sentValue = Number(idData.data[0]?.overview.sent[1])/decimalPower;
        recdTX = formatNumber0dp.format(idData.data[0].overview.received[0]);
        recdValue = Number(idData.data[0]?.overview.received[1])/decimalPower;
        totalTX = formatNumber0dp.format(idData.data[0]?.overview.sent[0] + idData.data[0]?.overview.received[0]);
        totalValue = Number(idData.data[0]?.overview.balance)/decimalPower;
        fromConvert = nanoToDate(Number(idData.data[0]?.overview.first_active));
        dateFrom = fromConvert.dateOnly;
        timeFrom = fromConvert.shortTime;
        toConvert = nanoToDate(Number(idData.data[0]?.overview.last_active));
        dateTo = toConvert.dateOnly;
        timeTo = toConvert.shortTime;

        // process transactions
        txTableData = processAccountTX(idData.data[0].blocks, acIdFull, token);
        dataLoaded = true;
    }

</script>

<LayoutCombine>

	<span slot="body" class="pb-4">
        <!-- First Section -->
        <div class="container mx-auto px-4 py-5 flex flex-wrap min-w-full md:min-w-0 content-center justify-center items-center">
            <!-- Warning text -->
            {#if warning != null}
                <div>
                    <p class="text-xl text-warning-500">{warning}</p>
                </div>
            {/if}

            <!-- Overview -->
            <div class="w-full">
                {#if dataLoaded == false}
                    <div class="p-3 min-h-screen">
                        <Loading/>
                        <p class="text-center pt-3">Searching {token} ledger...</p>
                    </div>
                {/if}
                {#if dataLoaded == true}
                {#if resolved == null}
                    <p class="pb-1">
                        Searched: {accountID} <CopyButton text={acIdFull}/> <SaveButton accountToSave={acIdFull}/>
                        {#if displayName != null}{@html "<br>"} <span class="text-primary-500">{displayName}</span>{/if}
                    </p>
                {:else}
                    <p class="pb-1">Resolved to: {resolved} <CopyButton text={acIdFull}/> <SaveButton accountToSave={acIdFull}/>
                        {#if displayName != null}{@html "<br>"} <span class="text-primary-500">{displayName}</span>{/if}
                    </p>
                {/if}
            {/if}
            </div>
            
                <div class="flex flex-col lg:flex-row gap-4 w-full pb-3">
                    {#if dataLoaded == true}
                    <div class="flex flex-row gap-4 w-full lg:w-3/5">
                        <div class="p-1 bg-surface-400/40 flex-1 w-1/5 rounded">
                            <p class="pl-1 text-primary-500 text-lg">Sent</p>
                                <p class="pl-1 text-lg">{sentValue} {token}</p>
                                <p class="pl-1 text">{sentTX} txs</p>
                        </div>
                        <div class="p-1 bg-surface-400/40 flex-1 w-1/5 rounded">
                            <p class="pl-1 text-primary-500 text-lg">Received</p>
                                <p class="pl-1 text-lg">{recdValue} {token}</p>
                                <p class="pl-1 text">{recdTX} txs</p>
                        </div>
                        <div class="p-1 bg-primary-400/40 flex-1 w-1/5 rounded">
                            <p class="pl-1 text-primary-500 text-lg">Balance</p>
                                <p class="pl-1 text-lg">{totalValue} {token}</p>
                                <p class="pl-1 text">{totalTX} txs</p>
                        </div>
                    </div>
                    <div class="flex flex-row gap-4 w-full lg:w-2/5">
                        <div class="p-1 bg-surface-400/40 flex-1 w-full lg:w-1/5 rounded">
                            <p class="pl-1 text-primary-500 text-lg">First Active</p>
                                <p class="pl-1 text-lg">{dateFrom}</p>
                                <p class="pl-1 text">{timeFrom} UTC</p>
                        </div>
                        <div class="p-1 bg-surface-400/40 flex-1 w-full lg:w-1/5 rounded">
                            <p class="pl-1 text-primary-500 text-lg">Last Active</p>
                                <p class="pl-1 text-lg">{dateTo}</p>
                                <p class="pl-1 text">{timeTo} UTC</p>
                        </div>
                    </div>
                    {/if}
                </div>
            <!-- Transactions -->
            {#if dataLoaded == true}
            <div class="w-full p-2 rounded mt-5"> 
                <div><p class="h4">Transactions:</p></div>  
                <div><AccountTxTable sourceData={txTableData} decimals={decimals} searchedAC={acIdFull}/></div>
            </div>
            {/if}
        </div>
	</span>
</LayoutCombine>


<style>

</style>