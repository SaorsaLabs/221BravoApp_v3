<script>
    import { Table, tableMapperValues, Paginator } from '@skeletonlabs/skeleton';
    import { shortenString, nanoToDate} from '../../code/utils.js';
    import { getModalStore} from '@skeletonlabs/skeleton';
    import BlockTxView from '../modal/blockTxView.svelte';

    export let sourceData;
    export let initNumItems = 50;

    const formatter = new Intl.NumberFormat('en-US'); 
    const modalStore = getModalStore(); // for popup
    let tableData = sourceData;
    let sourceDataLen = sourceData.length ?? 0;
    let decimalPower;
    let adjValue;
    let adjTime;
    let index = 0;
    let fromAcAdj;
    let toAcAdj;
    for(let i = 0; i< sourceDataLen; i++){
        tableData[i].index = index;
        tableData[i].shortFrom = shortenString(tableData[i].from_account, 10, 6);
        tableData[i].shortTo = shortenString(tableData[i].to_account, 10, 6);
        decimalPower = Math.pow(10, tableData[i].decimals);
        adjValue = Number(tableData[i].tx_value)/ decimalPower;
        tableData[i].valueAdj = adjValue; //formatter.format(
        adjTime = nanoToDate(Number(tableData[i].tx_time));
        tableData[i].adjTime = `${adjTime.dateOnly} <br/> ${adjTime.shortTime} UTC`;

        // from 
        if (tableData[i].fromUserName != null || tableData[i].fromGlobalName != null){
            if(tableData[i].fromUserName != null){
                fromAcAdj = `${tableData[i].shortFrom} <br> <span class="dark:text-primary-500 text-error-600">${tableData[i].fromUserName}</span>`;
            }else{
                fromAcAdj = `${tableData[i].shortFrom} <br> <span class="dark:text-primary-500 text-error-600">${tableData[i].fromGlobalName}</span>`;
            }
        } else { fromAcAdj = tableData[i].shortFrom}

        // to 
        if (tableData[i].toUserName != null || tableData[i].toGlobalName != null){
            if(tableData[i].toUserName != null){
                toAcAdj = `${tableData[i].shortTo} <br> <span class="dark:text-primary-500 text-error-600">${tableData[i].toUserName}</span>`;
            }else{
                toAcAdj = `${tableData[i].shortTo} <br> <span class="dark:text-primary-500 text-error-600">${tableData[i].toGlobalName}</span>`;
            }
        } else { toAcAdj = tableData[i].shortTo}

        tableData[i].fromAcAdj = fromAcAdj;
        tableData[i].toAcAdj = toAcAdj;
        tableData[i].blockAdj =  `${tableData[i].block} <br> <span class="dark:text-primary-500 text-error-600">${tableData[i].token}</span>`
        index++;
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
            meta: { clickedData: sourceData[indexSelected], },
			//response: (r) => console.log('response:', r)
		};
        modalStore.clear();
		modalStore.trigger(modal);
	}

    $: sourceData = sourceData;

    let headLabels = ['Block', 'Time', 'From', 'To', 'Type', `value`];
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
    $: sourceOutput = tableMapperValues(sourceBodySliced, ['blockAdj', 'adjTime', 'fromAcAdj', 'toAcAdj', 'tx_type', 'valueAdj']);

    function onPageChange(e) {
		//console.log('Paginator - event:page', e.detail);
	}
	function onAmountChange(e) {
		//console.log('Paginator - event:amount', e.detail);
	}

</script>


<div class="p-1 rounded w-full">
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
</div>