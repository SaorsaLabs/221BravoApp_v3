<script>
	import LayoutCombine from '../../lib/componants/shared/layoutCombine.svelte';
	import Head from "../../lib/componants/head/head.svelte";
	import HeadStats from "../../lib/componants/head/headStats.svelte";
	import Foot from "../../lib/componants/foot/foot.svelte";
    import { onMount, onDestroy } from 'svelte';
	let ex = 0;
	let examples = [
		"https://szqsa-nyaaa-aaaal-qbjua-cai.raw.ic0.app/546.html",
		"https://szqsa-nyaaa-aaaal-qbjua-cai.raw.ic0.app/437.html",
		"https://t243g-zaaaa-aaaal-qbjsq-cai.raw.ic0.app/81.html",
		"https://t243g-zaaaa-aaaal-qbjsq-cai.raw.ic0.app/205.html",
		"https://t243g-zaaaa-aaaal-qbjsq-cai.raw.ic0.app/265.html"
		];
    onMount(() => {

    });

	onDestroy(() => {

	});
</script>

<svelte:head>
	<title>Members - 221Bravo App</title>
</svelte:head>

<LayoutCombine>
	<span slot="headStats">
		<HeadStats/>
	</span>
	<span slot="head">
		<Head/>
	</span>
	<span slot="body" class="pb-4"> 
		<div class="container mx-auto px-4 py-5 flex flex-wrap min-w-full md:min-w-0 content-center justify-center items-center min-h-80">
			<p class="h3 text-warning-500">Membership services coming soon...</p>
		</div>
		<hr>
		<div class="container mx-auto px-4 py-5 flex flex-wrap min-w-full md:min-w-0 content-center justify-center items-center">
			<p class="h3">Check out our Dynamic Membership NFTs</p>
		</div>
		<div class="container mx-auto px-4 flex flex-wrap min-w-full min-w-full md:min-w-0 mb-5">
			<div class="w-full flex flex-col lg:flex-row gap-2 ">
				<div class="p-1 lg:w-1/2 w-full">
					<div class="w-full flex justify-center mb-3">
						<button class="pl-2 pr-2 bg-tertiary-500 rounded m-1" on:click={()=>{ ex = 0 }}>Example 1</button>
						<button class="pl-2 pr-2 bg-tertiary-500 rounded m-1" on:click={()=>{ ex = 1 }}>Example 2</button>
						<button class="pl-2 pr-2 bg-tertiary-500 rounded m-1" on:click={()=>{ ex = 2 }}>Example 3</button>
						<button class="pl-2 pr-2 bg-tertiary-500 rounded m-1" on:click={()=>{ ex = 3 }}>Example 4</button>
						<button class="pl-2 pr-2 bg-tertiary-500 rounded m-1" on:click={()=>{ ex = 4 }}>Example 5</button>
					</div>
					<iframe loading="lazy" title="" scrolling="no" src={examples[ex]} frameborder="0" allowfullscreen></iframe>
				</div>
				<div class="bg-surface-400/40 p-1 rounded lg:w-1/2 w-full">
					<div style="padding: 8px;">
							<p class="h3 text-black-500 dark:text-primary-600" style="text-align: center;">Become a member of 221Bravo.App and get access to exclusive content and tools.</p>
						{@html "<br>"}
						<p>
							We use Genesis II NFTs (see examples!) as your membership card giving you ultimate control over your membership and the ability to transfer or sell it at any time! 
						</p>	
						{@html "<br>"}
						<li>Exchanges and other interesting accounts are named</li>
						<li>Access premium stats, address book, visual block explorer and other tools</li>
						<li>Access to exclusive members chat</li>
						<li>Access to exclusive research and reports</li>
						<li>First in line for future events</li>
						<li>No monthly costs!</li>
						{@html "<br>"}
						<!-- <img class="headAlign" src={nftBanner} alt="" width="100%"/> -->
						<p  class="h4" style="padding-top: 10px; text-align:center;">Get your membership today from Toniq NFT Marketplace</p> 
					
						<div style="width: 100%; text-align:center; padding-top: 5px;"> 
							<a href="https://toniq.io/marketplace/genesis-ii" target="_blank">
								<button class="pl-2 pr-2 bg-tertiary-500 rounded m-1 mt-10"> <p class="h4">Buy Membership NFT</p></button>
							</a>
						</div>
					</div>
				</div>
			</div>
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