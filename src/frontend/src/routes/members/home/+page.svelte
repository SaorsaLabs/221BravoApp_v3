<script>
	import LayoutCombine from '../../../lib/componants/shared/layoutCombine.svelte';
	import Head from "../../../lib/componants/head/head.svelte";
	import HeadStats from "../../../lib/componants/head/headStats.svelte";
	import Foot from "../../../lib/componants/foot/foot.svelte";
	import { authStore } from '../../../lib/stores/authStore.js';
    import { onMount, onDestroy } from 'svelte';
	import Tools from '../../../lib/componants/members/tools.svelte';
	import Stats from '../../../lib/componants/members/stats.svelte';

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
		<!-- Tool Bar -->
		<div class="w-full dark:bg-primary-500/40 bg-primary-700 rounded p-1 mb-4">
			<div class="container mx-auto px-4 py-5 min-w-full md:min-w-0">
				<Tools/>
			</div>
		</div>

		<div class="container mx-auto px-4 py-1 min-w-full md:min-w-0 mb-5">
			<Stats/>
		</div>
		
		<div class="container mx-auto px-4 py-1 min-w-full md:min-w-0 mb-5">
			More coming soon...
		</div>
		
	</span>

	<span slot="foot">
		<Foot/>
	</span>
</LayoutCombine>

<style>
	iframe {
	margin: 0 auto;
    display: block;
    border: none; /* Remove border from iframe */
    width: 550px; /* Make iframe full width */
    height: 550px; /* Make iframe full height */
  }
</style>