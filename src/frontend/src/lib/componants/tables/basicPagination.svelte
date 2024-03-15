<script>
    import { createEventDispatcher } from 'svelte';
    export let page = 1;
    export let max = 2;

    const dispatch = createEventDispatcher();
    function setPage(change){
        page += change;
        if (page<1) page = 1;
        if (page>max) page = max;
        dispatch('click', {
			page
		});
    }
</script>
<nav class="alignRight">
        <button class="bg-warning-600/80 rounded m-2 mt-3 p-1 pl-2 pr-2" on:click={() => setPage(-1)}>PREV</button>
        {#if page >= 3}
        <button class="bg-warning-600/80 rounded m-2 mt-3 p-1 pl-2 pr-2" on:click={() => setPage(-2)}>{page-2}</button>
        {/if}
        {#if page >= 2}
        <button class="bg-warning-600/80 rounded m-2 mt-3 p-1 pl-2 pr-2" on:click={() => setPage(-1)}>{page-1}</button>
        {/if}
        <button class="bg-warning-600/80 rounded m-2 mt-3 p-1 pl-2 pr-2" on:click={() => setPage(0)}>{page}</button>
        {#if page <= max-1}
        <button class="bg-warning-600/80 rounded m-2 mt-3 p-1 pl-2 pr-2" on:click={() => setPage(+1)}>{page+1}</button>
        {/if}
        {#if page <= max-2}
        <button class="bg-warning-600/80 rounded m-2 mt-3 p-1 pl-2 pr-2" on:click={() => setPage(+2)}>{page+2}</button>
        {/if}
        <button class="bg-warning-600/80 rounded m-2 mt-3 p-1 pl-2 pr-2" on:click={() => setPage(+1)}>NEXT</button>
</nav>
<style>
    .alignRight{
        text-align: right;
        margin-right: 10px;
    }
</style>