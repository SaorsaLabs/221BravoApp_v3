<script lang="ts">
	import type { SvelteComponent } from 'svelte';
    import { getModalStore } from '@skeletonlabs/skeleton';
    import UserProfileContent from './userProfileContent.svelte';


	// Props
	/** Exposes parent props to this component. */
	export let parent: SvelteComponent;
	const modalStore = getModalStore();
    
	// Notes: Use `w-screen h-screen` to fit the visible canvas size.
	const cBase = 'bg-surface-100-800-token w-1/3 h-2/3 p-4 overflow-y-auto border-2 border-surface-500 rounded m-4';

    // returnMessage
    function handleMessage(data){
        // close modal
        if (data.detail == "logged-in"){
            parent.onClose();
        }
        if (data.detail == "not-member"){
            setTimeout(()=>{ parent.onClose() }, 2000); 
        }
    }   

</script>

{#if $modalStore[0]}
    <div class="{cBase}">
        <div class="float-right"><button class="btn variant-filled" on:click={parent.onClose}>X</button></div>
        <!-- <p class="h4 mb-2">User Profile: </p> -->
        <UserProfileContent on:message={handleMessage}/>
    </div>
{/if}

