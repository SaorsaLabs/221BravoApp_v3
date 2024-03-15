<script lang="ts">
	import type { SvelteComponent } from 'svelte';
	// Stores
	import { getModalStore } from '@skeletonlabs/skeleton';
	// Props
	/** Exposes parent props to this component. */
	export let parent: SvelteComponent;
	const modalStore = getModalStore();
    let token = $modalStore[0].meta.token;
    let id = $modalStore[0].meta.id;
	// Notes: Use `w-screen h-screen` to fit the visible canvas size.
	const cBase = 'bg-surface-100-800-token w-screen h-screen p-4 overflow-y-auto border-2 border-surface-500 rounded m-4';
</script>

{#if $modalStore[0]}
    <div class="{cBase}">
        <div class="float-right"><button class="btn variant-filled" on:click={parent.onClose}>X</button></div>
        <iframe title="" src="/account/modal/{token}?id={id}" frameborder="0" allowfullscreen></iframe>
    </div>
{/if}

<style>
iframe {
    border: none; /* Remove border from iframe */
    width: 90vw; /* Make iframe full width */
    height: 90vh; /* Make iframe full height */
  }
  
</style>
