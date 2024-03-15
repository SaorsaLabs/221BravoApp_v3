<script>
	import LayoutCombine from '../../lib/componants/shared/layoutCombine.svelte';
	import Head from "../../lib/componants/head/head.svelte";
	import HeadStats from "../../lib/componants/head/headStats.svelte";
	import Foot from "../../lib/componants/foot/foot.svelte";
</script>

<svelte:head>
	<title>T+C/ Privacy - 221Bravo App</title>
</svelte:head>

<LayoutCombine>
	<span slot="headStats">
		<HeadStats/>
	</span>
	<span slot="head">
		<Head/>
	</span>
	<span slot="body" class="pb-4"> 
		<div class="container mx-auto px-4 py-5 flex flex-wrap min-w-full md:min-w-0 content-center min-h-80">
			<div style="padding: 10px;">
				<b>IMPORTANT DISCLAIMER:</b> 
				<p class="pt-2">
				All content provided herein our website (221Bravo.App), hyperlinked sites, 
				associated applications, forums, blogs, social media accounts and other platforms 
				(“Site”) is for your general information only.
				We make no warranties of any kind in relation to our content, including but not limited 
				to accuracy and updatedness. No part of the content that we provide constitutes 
				financial advice, legal advice or any other form of advice meant for your specific 
				reliance for any purpose. Any use or reliance on our content is solely at your own 
				risk and discretion. 
				</p>
				<p class="pt-2">
				You should conduct your own research, review, analyse and verify 
				our content before relying on them. Trading is a highly risky activity that can lead 
				to major losses, please therefore consult your financial advisor before making any 
				decision. No content on our Site is meant to be a solicitation or offer.
				</p>
			</div>
		</div>
		<hr/>
		<div class="container mx-auto px-4 py-5 flex flex-wrap min-w-full md:min-w-0 min-h-80">
			<div style="padding: 10px;">
				<h3 class="h3">Privacy Policy</h3>

				<p class="pt-2">
				Welcome to 221Bravo.App (“we”, “our”, or “us”). We respect your privacy and are committed to protecting it through our compliance with this policy.
				</p>

				<p class="pt-4">
				<b>This policy describes:</b>
				</p>
				<p class="pt-2">
				The types of information we may collect or that you may provide when you visit our website, 221Bravo.App.

				Our practices for collecting, using, maintaining, protecting, and disclosing that information.
				</p>
				<p class="pt-4">
				<b>Information We Collect and How We Collect It</b>
				</p>
				<p class="pt-2">
					We do not store any personal identifiable data via our Canister Smart Contracts other than information provided voluntarily by you through your ‘Address Book’ (See member’s section of our website). 
					This information is stored securely within a smart contract and can only be accessed by yourself and 221Bravo Admins. 
					Access to this information is only for the purposes of backing up and maintaining the application. No data is sold to or processed by 3rd party organisations.
					Users wishing to remove named accounts can do so via the members section of the website. 
				</p>
				<p class="pt-2">
					If you agree, we will collect page view and other statistics via Fathom Analytics (A privacy-first analytics platform) - This information is used by our developers to better understand what users find useful and improve our services. 
				</p>
				<p class="pt-2">
				The Members Chat function is a third-party plugin provided by OpenChat (https://oc.app). No data is collected, processed or stored by us in regards the chat functionality.
				</p>
				<p class="pt-2">
				We implement measures designed to secure all data from accidental loss and from unauthorized access, use, alteration, and disclosure.
				</p>
				<p class="pt-4">
				<b>Publicly Available Information</b>
				</p>
				<p class="pt-2">
					221Bravo.App collects data from numerous public data sources, including public ledgers and third-party aggregators which we do not control. 221Bravo app does not share user data with any third party organizations. 
				</p>
				
				<p class="pt-4">
					<b>Changes to Our Privacy Policy</b>
				</p>
			
				<p class="pt-2">
					It is our policy to post any changes we make to our privacy policy on this page. If we make material changes to how we treat our users' personal information, we will notify you through a notice on the website home page.	
				</p>
				<p class="pt-2 pb-4">
					Contact Information:
					To ask questions or comment about this privacy policy and our privacy practices, contact us at: hello@saorsalabs.net
				</p>
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