<script>
    import { Table, tableMapperValues, Paginator } from '@skeletonlabs/skeleton';
    import { shortenString, nanoToDate} from '../../code/utils.js';
    import { getModalStore} from '@skeletonlabs/skeleton';
    import BlockTxView from '../modal/blockTxView.svelte';
    import { addNamedAccounts } from '../../code/fetch/namedAccounts.js';
    import { onMount } from 'svelte';

    export let sourceData;
    export let decimals;
    export let token;
    export let initNumItems = 25;

    onMount(()=>{fetchNames()});

    const formatter = new Intl.NumberFormat('en-US'); 
    const modalStore = getModalStore(); // for popup
    let tableData = sourceData;
    let tableDataLen = tableData.length ?? 0;
    let decimalPower = Math.pow(10, decimals);
    let adjValue;
    let adjTime;
    let index = 0;
    let loadComplete = false;

    async function fetchNames(){
        let allACS = [];
        for(let i=0; i< tableDataLen; i++){
            allACS.push({from_account: tableData[i].from_account, to_account: tableData[i].to_account});
        }
        let searchedName = await addNamedAccounts(allACS);
        for(let i=0; i< tableDataLen; i++){
            if ( searchedName[i]?.fromGlobalName || searchedName[i]?.fromUserName ){
                if(searchedName[i]?.fromUserName){
                    tableData[i].displayFromName = searchedName[i].fromUserName;
                    tableData[i].fromUserName = searchedName[i].fromUserName;
                } else {
                    tableData[i].displayFromName = searchedName[i].fromGlobalName;
                    tableData[i].fromGlobalName = searchedName[i].fromGlobalName;
                }
            }
            if ( searchedName[i]?.toGlobalName || searchedName[i]?.toUserName ){
                if(searchedName[i]?.toUserName){
                    tableData[i].displayToName = searchedName[i].toUserName;
                    tableData[i].toUserName = searchedName[i].toUserName;
                } else {
                    tableData[i].displayToName = searchedName[i].toGlobalName;
                    tableData[i].toGlobalName = searchedName[i].toGlobalName;
                }
            }
        }

        for(let i = 0; i< tableDataLen; i++){
            tableData[i].index = index;
            tableData[i].token = token;
            tableData[i].shortFrom = shortenString(tableData[i].from_account, 10, 6);
            tableData[i].shortTo = shortenString(tableData[i].to_account, 10, 6);
            adjValue = Number(tableData[i].tx_value)/ decimalPower;
            tableData[i].valueAdj = formatter.format(adjValue);
            adjTime = nanoToDate(Number(tableData[i].tx_time));
            tableData[i].adjTime = `${adjTime.dateOnly} <br/> ${adjTime.shortTime} UTC`;

            // From Name
            if (tableData[i].displayFromName){
                tableData[i].shortFromAdj = `${tableData[i].shortFrom} <br> <span class="dark:text-primary-500 text-error-600">${tableData[i].displayFromName}</span>`
            } else {
                tableData[i].shortFromAdj = tableData[i].shortFrom;
            }

            // To Name
            if (tableData[i].displayToName){
                tableData[i].shortToAdj = `${tableData[i].shortTo} <br> <span class="dark:text-primary-500 text-error-600">${tableData[i].displayToName}</span>`
            } else {
                tableData[i].shortToAdj = tableData[i].shortTo;
            }

            index++;
        }
        
        loadComplete = true;
    }

    // DATA AVAILABLE FOR TABLE/ MODAL
    // count,
    // block
    // from_account
    // hash
    // spender
    // to_account
    // tx_fee
    // tx_time
    // tx_type
    // tx_value
    // adjTime
    // adjDir
    // shortFrom
    // shortTo

    function onSelected(meta){
        let indexSelected = meta.detail[0];
        const c = { ref: BlockTxView };
		const modal = {
			type: 'component',
			component: c,
			title: 'Custom Form Component',
			body: '',
            meta: { clickedData: tableData[indexSelected], },
			//response: (r) => console.log('response:', r)
		};
        modalStore.clear();
		modalStore.trigger(modal);
	}

    $: tableData = tableData;
    $: sourceOutput = tableMapperValues(sourceBodySliced, ['valueAdj', 'tx_type', 'adjTime', 'shortFromAdj', 'shortToAdj',  'block']);
    
    let headLabels = [`value <br/> (${token})`, 'Type', 'Time', 'From', 'To', 'Block']; 
    let paginationSettings = {
            page: 0,
            limit: initNumItems,
            size: tableData.length,
            amounts: [10, 25, 50, 100],
        };
    
    $: sourceBodySliced = tableData.slice(
		paginationSettings.page * paginationSettings.limit,
		paginationSettings.page * paginationSettings.limit + paginationSettings.limit
	);
    

    function onPageChange(e) {
		//console.log('Paginator - event:page', e.detail);
	}
	function onAmountChange(e) {
		//console.log('Paginator - event:amount', e.detail);
	}

</script>


<div class="p-1 rounded w-full">
    {#if loadComplete == true}
        <Table 
            source={
                {head: headLabels, 
                body: sourceOutput, 
                meta: tableMapperValues(sourceBodySliced, ['index'])}} 
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
    {/if}
</div>