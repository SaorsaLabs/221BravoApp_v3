<script lang="ts">
	import type { SvelteComponent } from 'svelte';
	// Stores
	import { getModalStore } from '@skeletonlabs/skeleton';
	// Props
	/** Exposes parent props to this component. */
	export let parent: SvelteComponent;

	const modalStore = getModalStore();

	// Notes: Use `w-screen h-screen` to fit the visible canvas size.
	const cBase = 'bg-surface-100-800-token w-4/5 h-2/3 p-4 overflow-y-auto border-2 border-surface-500 rounded m-4';
</script>

{#if $modalStore[0]}
    
    <div class="{cBase}">
        <div class="float-right"><button class="btn variant-filled" on:click={parent.onClose}>X</button></div>
        <h2 class="h2 mb-2 text-primary-700 dark:text-tertiary-500">Search tool - Help </h2>
        <hr class="mb-2 mt-3">
        <p class="text-wrap mb-2">The 221Bravo account search tool scans dozens of top token ledgers on the Internet Computer to bring you a comprehensive 
            view of account balances and transactions. </p>
        <p class="text-wrap mb-2">You can search by Principal, Account or ICRC Address:</p>
        <p class="text-primary-700 dark:text-tertiary-500"><b>Principal - example :</b></p> 
         <p class="mb-4 text-sm margin">az453-x2sxf-wewfl-pszbd-4u4rh-yq7nk-hxkrp-6yvo3-mnlce-zjvsg-qae </p> 
         
         <p class="text-primary-700 dark:text-tertiary-500"><b>Account - example :</b></p> 
         <p class="mb-4 text-sm"> 091651da9630d879e55dd335c011ce10327de19a03e85e60421efa237fd2e04a </p>
        
         <p class="text-primary-700 dark:text-tertiary-500"><b>ICRC (Principal.SubAccount) - example :</b></p> 
         <p class="text-sm">k7tml-iaaaa-aaaak-aecgq-cai.00000419a9b3205800cf8689d3711f7b5640d624d6b3a5ceae67285c9c70a2a7</p>
        <p class="text-sm pt-2"> When searching for an ICRC address, using the principal only will return the default ‘0’ sub-account for that ICRC address. 
            If you want to search a specific sub-account put a full stop '.' between 
            the principal and sub-account you are searching for. </p>
        <p class="text-sm pt-2">For example : searching <b>k7tml-iaaaa-aaaak-aecgq-cai</b> is the same as searching <b>k7tml-iaaaa-aaaak-aecgq-cai.0000000000000000000000000000000000000000000000000000000000000000</b></p>
    </div>
{/if}