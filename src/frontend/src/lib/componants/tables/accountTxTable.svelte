<script>
    import { Table, tableMapperValues, Paginator } from '@skeletonlabs/skeleton';
    import { shortenString, nanoToDate} from '../../code/utils.js';
    import { getModalStore} from '@skeletonlabs/skeleton';
    import AccountTxView from '../modal/accountTxView.svelte';

    export let sourceData;
    export let decimals;
    export let searchedAC;
    export let multiTokenView = false;

    const formatter = new Intl.NumberFormat('en-US', {maximumFractionDigits: 8}); 
    const modalStore = getModalStore(); // for popup
    let chartData = sourceData;
    let sourceDataLen = sourceData.length ?? 0;
    let decimalPower = (decimals == 0) ? 1 : Math.pow(10, decimals); // 0 is used for multi token tables
    let adjValue;
    let adjTime;
    let adjDir;
    let searchedName;
    let shortAcAdj;
    for(let i = 0; i< sourceDataLen; i++){
        chartData[i].shortAC = shortenString(chartData[i].linkedAC, 10, 6);
        if (multiTokenView == true) {
            adjValue = Number(chartData[i].value)/ decimalPower;
            chartData[i].valueAdj = `${formatter.format(adjValue)} <br> <span class="text-xs">${chartData[i].token}</span>`;
        } else {
            adjValue = Number(chartData[i].value)/ decimalPower;
            chartData[i].valueAdj = formatter.format(adjValue);
        }
        adjTime = nanoToDate(Number(chartData[i].time));
        chartData[i].adjTime = `${adjTime.dateOnly} <br/> ${adjTime.shortTime} UTC`;//adjTime.fullDateTime;
        if (chartData[i].direction == "in") {
            adjDir = `in <br> <span class="text-green-500 text-lg">⇨💰</span>`;
            if (chartData[i].fromUserName != null || chartData[i].fromGlobalName != null){
                if(chartData[i].fromUserName != null){
                    shortAcAdj = `${chartData[i].shortAC} <br> <span class="dark:text-primary-500 text-error-600">${chartData[i].fromUserName}</span>`;
                }else{
                    shortAcAdj = `${chartData[i].shortAC} <br> <span class="dark:text-primary-500 text-error-600">${chartData[i].fromGlobalName}</span>`;
                }
            } else { shortAcAdj = chartData[i].shortAC}
        } else {
            adjDir = `out <br> <span class="text-red-500 text-lg">⇦💰</span>`;
            if (chartData[i].toUserName != null || chartData[i].toGlobalName != null){
                if(chartData[i].toUserName != null){
                    shortAcAdj = `${chartData[i].shortAC} <br> <span class="dark:text-primary-500 text-error-600">${chartData[i].toUserName}</span>`;
                }else{
                    shortAcAdj = `${chartData[i].shortAC} <br> <span class="dark:text-primary-500 text-error-600">${chartData[i].toGlobalName}</span>`;
                }
            } else { shortAcAdj = chartData[i].shortAC}
        }
        chartData[i].shortAcAdj = shortAcAdj;
        chartData[i].adjDir = adjDir;
    }

    // DATA AVAILABLE FOR TABLE/ MODAL
    //         count,
    //         time: inputArray[i].tx_time,
    //         token,
    //         linkedAC,
    //         direction,
    //         value: inputArray[i].tx_value,
    //         type: inputArray[i].tx_type,
    //         fee: inputArray[i].tx_fee
    //         shortAC
    //         adjTime
    //         adjDir

    function onSelected(meta){
        let indexSelected = meta.detail[0]-1;
        const c = { ref: AccountTxView };
		const modal = {
			type: 'component',
			component: c,
			title: 'Custom Form Component',
			body: '',
            meta: { clickedData: sourceData[indexSelected], searchedAC },
			//response: (r) => console.log('response:', r)
		};
        modalStore.clear();
		modalStore.trigger(modal);
	}

    $: sourceData = sourceData;

    let headLabels = ['#', 'Time', 'Token', 'Account', 'Type', 'Direction', 'value'];
    let paginationSettings = {
            page: 0,
            limit: 25,
            size: chartData.length,
            amounts: [10, 25, 50, 100],
        };
    
    $: sourceBodySliced = chartData.slice(
		paginationSettings.page * paginationSettings.limit,
		paginationSettings.page * paginationSettings.limit + paginationSettings.limit
	);
    $: sourceOutput = tableMapperValues(sourceBodySliced, ['count', 'adjTime', 'token', 'shortAcAdj', 'type', 'adjDir', 'valueAdj']);


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
            meta: tableMapperValues(sourceBodySliced, ['count'])}} 
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