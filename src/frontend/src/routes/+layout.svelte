<script>
	import '../app.postcss';
	import { initializeStores } from '@skeletonlabs/skeleton';
	initializeStores();
	import { Modal, Toast } from '@skeletonlabs/skeleton';
	import { dev } from '$app/environment';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { cookieStore } from '../lib/stores/cookieStore.js';
	import Analytics from '../lib/componants/shared/analytics2.svelte';
	
	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

	if ('serviceWorker' in navigator) {
		addEventListener('load', function () {
			navigator.serviceWorker.register('../service-worker.js', {
				type: dev ? 'module' : 'classic'
			});
		});
	}

	onMount(() => {
		checkCookieSettings();
	});
	let allowCookies = false;
	async function checkCookieSettings(){
		if(browser){
			let cs = await cookieStore.read();
			if (cs?.data?.allowCookies == true){
				allowCookies = true;
			}
		}
	}

</script>
{#if allowCookies == true}
	<Analytics allowCookies={true}/>
{/if}
<Modal />
<Toast />

<slot />
