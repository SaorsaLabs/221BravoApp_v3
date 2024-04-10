<script>
    import { Table, tableMapperValues, Paginator } from '@skeletonlabs/skeleton';
    import { getModalStore} from '@skeletonlabs/skeleton';
    import CopyButton from '../shared/copyButton.svelte';
    import { onMount } from 'svelte';
    import { createEventDispatcher} from 'svelte';
    import delete1 from "$lib/images/icons/deleteDark.png";
    import delete2 from "$lib/images/icons/deleteLight.png";
    import Loading from '../shared/loading.svelte';

    export let sourceData;
    export let initNumItems = 5;

    const modalStore = getModalStore(); // for popup
    let tableData = sourceData;
    let sourceDataLen = sourceData.length ?? 0;
    let rowSelected = false;
    let selectedData;
    let isDarkMode;
    let showDeleteLoader = false;

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

    // return message to addressBook.svelte
    const dispatch = createEventDispatcher();
    function returnMessage(message) {
        dispatch('message', message);
    }

    // DATA AVAILABLE FOR TABLE/ MODAL
    // count,
    // account
    // name
    // shortAccount

    function onSelected(meta){
        let indexSelected = meta.detail[0]; 
        selectedData = sourceData[indexSelected];
        rowSelected = true;
	}

    $: sourceData;
    $: tableData = sourceData;

    let headLabels = ['#', 'Account', 'Name'];
    let paginationSettings = {
            page: 0,
            limit: initNumItems,
            size: tableData.length,
            amounts: [5],
        };
    
    $: sourceBodySliced = tableData.slice(
		paginationSettings.page * paginationSettings.limit,
		paginationSettings.page * paginationSettings.limit + paginationSettings.limit
	);
    $: sourceOutput = tableMapperValues(sourceBodySliced, ['count', 'shortAccount', 'name']);

    // function onPageChange(e) {
	// 	//console.log('Paginator - event:page', e.detail);
	// }
	// function onAmountChange(e) {
	// 	//console.log('Paginator - event:amount', e.detail);
	// }

</script>

<div class="p-1 rounded w-full">
    <div class="pb-3">
        {#if rowSelected == true}
            <p class="dark:text-warning-500 text-error-600">SELECTED ROW:</p>
            <table class="w-full bg-warning-600/80 bg-rounded">
                <tr>
                    <td class="text-center">{selectedData.shortAccount}</td>
                    <td class="text-center">{selectedData.name}</td>
                    <td class="text-center pt-2">
                        <CopyButton text={selectedData.account}/>
                        <!-- Link to Members Combined Token Account Search -->
                        <!-- <button> 
                            <a href="/account/{clickedData.token}?id={clickedData.from_account}" target="_blank">
                                <span class="pl-2 text-xl">🔎︎</span>
                            </a>
                        </button> -->
                        <!-- delete button -->
                        {#if showDeleteLoader == false}
                            <button on:click={() => { 
                                returnMessage({type: "delete", account: selectedData.account}); 
                                showDeleteLoader = true; }
                            }>
                                <img class="copy" src={isDarkMode ? delete2 : delete1} alt="Delete Account" width="20px" style="margin-left:5px"/>
                            </button>
                        {:else}
                            <span>
                                <Loading style={"loaderBlueSmall"}/>
                            </span>
                        {/if}
                    </td>
                </tr>
            </table>
        {/if}
    </div>
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