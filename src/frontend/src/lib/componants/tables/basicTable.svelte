<script>
    import { onMount } from 'svelte';
    import { createEventDispatcher } from 'svelte';
    import BasicPagination from './basicPagination.svelte';

    export let data = []; // [{..},{..}]
    export let tableHeaders = []; // format [{public: String, data: String}]

    let sortedData = data;
    let dataLen = sortedData?.length ?? 0;
    let page = 0;
    let totalPages = [];
    let itemsPerPage = 5;
    let currentPageRows = [];
    let maxPage;

    $: currentPageRows = totalPages.length > 0 ? totalPages[page] : [];
    const paginate = (items) => {
    const pages = Math.ceil(items.length / itemsPerPage);
    maxPage = pages;
    const paginatedItems = Array.from({ length: pages }, (_, index) => {
      const start = index * itemsPerPage;
      return items.slice(start, start + itemsPerPage);
    });
    totalPages = [...paginatedItems];
  };

    function pageChange(event){
        page = event.detail.page-1; //-1 to match array
    }

    onMount(async () => {
        paginate(sortedData);
	});
    
    const dispatch = createEventDispatcher();

    const sendDataToParent = (data) => {
        dispatch('dataPassed', data);
    };
   
</script>


<table class="mainTable">
    <!-- HEAD ROW -->
    <tr>
        {#each tableHeaders as header}
            <th> {header.public} </th>	
        {/each}
    </tr>
    <!-- DATA ROWS -->
    {#each currentPageRows as ROW}
        <tr>
            {#each tableHeaders as D}
                <td>
                    {#if D.button == false}
                        {ROW[D.data]}
                    {:else if D.button == true}
                    <button on:click={sendDataToParent(ROW)}>{D.buttonText}</button>
                    {/if}
                </td>
            {/each}
        </tr>
    {/each}
</table>
{#if maxPage > 1}
    <BasicPagination max={maxPage} on:click={pageChange}/>
{/if}

<style>
    	.mainTable {
		border-spacing: 0;
		width: 100%;
        margin-bottom: 10px;
	}
	th {
		text-transform: uppercase;
        background-color: rgba(61, 61, 61, 0.5);
	}
	th, td {
		text-align: center;
		padding: 5px;
	}
</style>