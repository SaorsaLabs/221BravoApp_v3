<script>
	import LayoutCombine from '../../../lib/componants/shared/layoutCombine.svelte';
	import Head from "../../../lib/componants/head/head.svelte";
	import HeadStats from "../../../lib/componants/head/headStats.svelte";
	import Foot from "../../../lib/componants/foot/foot.svelte";
	import { authStore } from '../../../lib/stores/authStore.js';
    import { onMount, onDestroy } from 'svelte';

	
    onMount(() => {
		let x = authStore.read();
		let loggedIn = x.data.loggedIn;
		if(loggedIn == "false" || loggedIn == false) goto("/");
    });

	onDestroy(() => {

	});
</script>

<svelte:head>
	<title>Members Home - 221Bravo App</title>
</svelte:head>

<LayoutCombine>
	<span slot="headStats">
		<HeadStats/>
	</span>
	<span slot="head">
		<Head/>
	</span>
	<span slot="body" class="pb-4"> 
		<div class="container mx-auto px-4 py-5 flex flex-wrap min-w-full md:min-w-0">
			<p class="h3">Members Only Area</p>
		</div>
		<div class="container mx-auto px-4 py-5 flex flex-wrap min-w-full md:min-w-0">
			<p>Work In Progress!</p>
		</div>
	</span>

	<span slot="foot">
		<Foot/>
	</span>
</LayoutCombine>
