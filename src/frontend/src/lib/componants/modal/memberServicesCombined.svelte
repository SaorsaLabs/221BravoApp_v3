<script lang="ts">
	import type { SvelteComponent } from 'svelte';
    import { getModalStore } from '@skeletonlabs/skeleton';
    import AddressBook from './addressBook.svelte';
    import PriceAlerts from './priceAlerts.svelte';
    

	// Props
	/** Exposes parent props to this component. */
	export let parent: SvelteComponent;
	const modalStore = getModalStore();
    let type = $modalStore[0].meta.modalType;
    let width = $modalStore[0].meta?.width ?? "w-4/5";
    let pageSelect = $modalStore[0].meta?.pageSelect ?? 0;
    let saveAccount = $modalStore[0].meta?.saveAccount ?? undefined;
    let saveCross = $modalStore[0].meta?.saveCross ?? undefined;

	// Notes: Use `w-screen h-screen` to fit the visible canvas size.
	const cBase = `bg-surface-100-800-token ${width} h-2/3 p-4 overflow-y-auto border-2 border-surface-500 rounded m-4`;

    // returnMessage
    function handleMessage(data){
        // console.log(data.detail);
        // // close modal
        // if (data.detail == "logged-in"){
        //     parent.onClose();
        // }
        // if (data.detail == "not-member"){
        //     setTimeout(()=>{ parent.onClose() }, 2000); 
        // }
    }   

</script>

{#if $modalStore[0]}
    <div class="{cBase}">
        <div class="float-right"><button class="btn variant-filled" on:click={parent.onClose}>X</button></div>
        <!-- CONTENT -->
        {#if type == "addressbook"}
            <AddressBook pageSelect={pageSelect} inputAccount={saveAccount}/>
        {:else if type == "pricealerts"}
            <PriceAlerts showPage={pageSelect} saveCross={saveCross}/>
        {:else if type == "notmember"}
        Login to use Members Tools 😎
        {/if}
    </div>
{/if}

