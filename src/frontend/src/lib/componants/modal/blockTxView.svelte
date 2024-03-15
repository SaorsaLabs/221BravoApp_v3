<script lang="ts">
	import type { SvelteComponent } from 'svelte';
    import { nanoToDate } from '../../code/utils.js';
    import { getModalStore } from '@skeletonlabs/skeleton';
    import CopyButton from '../../componants/shared/copyButton.svelte';

	// Props
	/** Exposes parent props to this component. */
	export let parent: SvelteComponent;
	const modalStore = getModalStore();
    let clickedData = $modalStore[0].meta.clickedData;
    let adjTime = nanoToDate(Number(clickedData.tx_time));
    
	// Notes: Use `w-screen h-screen` to fit the visible canvas size.
	const cBase = 'bg-surface-100-800-token w-4/5 h-2/3 p-4 overflow-y-auto border-2 border-surface-500 rounded m-4';

</script>

{#if $modalStore[0]}
    <div class="{cBase}">
        <div class="float-right"><button class="btn variant-filled" on:click={parent.onClose}>X</button></div>
        <p class="h4 mb-2">Block Viewer: </p>
		<div class="flex flex-col sm:flex-row gap-4 pr-2">
            <div class="bg-surface-400/40 flex-1 rounded p-1 pl-2">
                <p class="dark:text-warning-500 text-error-600">Date/ Time (UTC)</p>
                {adjTime.shortTime} on {adjTime.dateOnly} 
            </div>
            <div class="bg-primary-400/40 flex-1 rounded p-1 pl-2">
                <p class="dark:text-warning-500 text-error-600">Type/ Value </p>
                {clickedData.tx_type} <span class="pl-2"> {clickedData.valueAdj} {clickedData.token}
            </div>
            <div class="bg-surface-400/40 flex-1 rounded p-1 pl-2">
                <p class="dark:text-warning-500 text-error-600">Block </p>
                {clickedData.block} on {clickedData.token} ledger
            </div>
        </div>
        <div class="pr-2 pt-2">
            <div class="bg-surface-400/40 rounded mt-2 p-1">
                <p class="dark:text-warning-500 text-error-600">From </p>
                {clickedData.from_account}
                {#if clickedData.fromGlobalName != null } {@html "<br>"} Public Name: <span class="dark:text-primary-500 text-error-600">{clickedData.fromGlobalName}</span>{/if}
                {#if clickedData.fromUserName != null } {@html "<br>"} Custom Name: <span class="dark:text-primary-500 text-error-600">{clickedData.fromUserName}</span>{/if}
                <p class="pt-1"><CopyButton text={clickedData.from_account}/>
                    {#if clickedData.from_account != "Token Ledger"}
                        <button> 
                            <a href="/account/{clickedData.token}?id={clickedData.from_account}" target="_blank">
                                <span class="pl-2 text-xl">🔎︎</span>
                            </a>
                        </button>
                    {/if}
                </p>
            </div>
            <div class="mt-2">
                <p class="h2 text-white-500 pl-2">⇊</p>
            </div>
            <div class="bg-surface-400/40 rounded mt-2 p-1">
                <p class="dark:text-warning-500 text-error-600">To </p>
                {clickedData.to_account}
                {#if clickedData.toGlobalName != null } {@html "<br>"} Public Name: <span class="dark:text-primary-500 text-error-600">{clickedData.toGlobalName}</span>{/if}
                {#if clickedData.toUserName != null } {@html "<br>"} Custom Name: <span class="dark:text-primary-500 text-error-600">{clickedData.toUserName}</span>{/if}
                <p class="pt-1"><CopyButton text={clickedData.to_account}/>
                    {#if clickedData.to_account != "Token Ledger"}
                        <button> 
                            <a href="/account/{clickedData.token}?id={clickedData.to_account}" target="_blank">
                                <span class="pl-2 text-xl">🔎︎</span>
                            </a>
                        </button>
                    {/if}
                </p>
            </div>
        </div>
    </div>
{/if}

<style>
iframe {
    border: none; /* Remove border from iframe */
    width: 90vw; /* Make iframe full width */
    height: 90vh; /* Make iframe full height */
  }
  
</style>
