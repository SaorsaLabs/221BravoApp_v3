<script>
    import { Table, tableMapperValues, Paginator } from '@skeletonlabs/skeleton';
    import { shortenString} from '../../code/utils.js';
    import { getModalStore} from '@skeletonlabs/skeleton';
    import { onMount } from 'svelte';
    import AccountView from '../modal/accountView.svelte';
    import { addNamedAccounts } from '../../code/fetch/namedAccounts.js';

    export let sourceData;
    export let tokenSupply;
    export let decimals;
    export let token;

    onMount(()=>{fetchNames()});

    const modalStore = getModalStore(); // for popup
    const formatter = new Intl.NumberFormat('en-US', {
         maximumFractionDigits: 2,
    });
    let paginationSettings = {
            page: 0,
            limit: 10,
            size: 0,
            amounts: [10, 25, 50],
        };
    let chartData = [];
    let sourceDataLen = sourceData?.length ?? 0;
    let count = 1;
    let ac, bal, pct, pct2, adjBal;
    let supplyAdj;
    let loadComplete = false;

    // check for account names
    async function fetchNames(){
        let allACS = [];
        for(let i=0; i< sourceDataLen; i++){
            allACS.push({from_account: sourceData[i].holder, to_account: sourceData[i].holder});
        }
        let searchedName = await addNamedAccounts(allACS);

        for(let i=0; i< sourceDataLen; i++){
            if ( searchedName[i]?.fromGlobalName || searchedName[i]?.fromUserName ){
                if(searchedName[i]?.fromUserName){
                    sourceData[i].displayName = searchedName[i].fromUserName;
                } else {
                    sourceData[i].displayName = searchedName[i].fromGlobalName;
                }
            }
        }

        let ac2;
        for(let i = 0; i< sourceDataLen; i++){
            adjBal =Number(sourceData[i].data.balance)/ Math.pow(10, decimals);
            supplyAdj = Number(tokenSupply) /Math.pow(10, decimals);
            bal = formatter.format(adjBal);
            pct = formatter.format((adjBal / tokenSupply)*100);
            pct2 = `${pct} %`;
            if (sourceData[i].displayName){
                ac2 = shortenString(sourceData[i].holder, 18, 12);
                ac = `${ac2} <br> <span class="dark:text-primary-500 text-error-600">${sourceData[i].displayName}</span>`
            } else {
                ac = shortenString(sourceData[i].holder, 18, 12);
            }
            chartData.push({count, account: ac, balance: bal, pcnt: pct2, fullac: sourceData[i].holder});
            count ++;
        }

        paginationSettings = {
            page: 0,
            limit: 10,
            size: chartData.length,
            amounts: [10, 25, 50],
        };

        loadComplete = true;
    }

    

    function onSelected(meta){
        const c = { ref: AccountView };
		const modal = {
			type: 'component',
			component: c,
			title: 'Custom Form Component',
			body: '',
            meta: { token, id: meta.detail[0] },
			//response: (r) => console.log('response:', r)
		};
        modalStore.clear();
		modalStore.trigger(modal);
	}

    $: sourceData = sourceData;

    let headLabels = ['#', 'Account', 'Balance', '% supply'];
    
    $: sourceBodySliced = chartData.slice(
		paginationSettings.page * paginationSettings.limit,
		paginationSettings.page * paginationSettings.limit + paginationSettings.limit
	);
    $: sourceOutput = tableMapperValues(sourceBodySliced, ['count', 'account', 'balance', 'pcnt']);


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
                meta: tableMapperValues(sourceBodySliced, ['fullac'])}} 
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