<script>
    import logo from '$lib/images/221B_sml.png';
    import {authStore} from '../../stores/authStore.js';
    import { onMount } from 'svelte';

    let memberLink = "/members";
    onMount(()=>{
        authStore.check();
		let x = authStore.read();
	    if (x.data.loggedIn == true || x.data.loggedIn == 'true') {
            memberLink = "/members/home";
        }
    });


</script>

<header class="bg-surface-700 text-white border-b-2 border-primary-700/10 drop-shadow-lg">
    <div class="container mx-auto px-4 py-3 min-w-full md:min-w-0">
        <!-- justify-between -->
        <div class="gap-8 columns-2 align-middle">
            <div><a href="/"><img class="logo" src={logo} alt="221Bravo Logo" /></a></div>
            <div class="flex justify-end h-full">
                <div class="flex justify-between gap-5 py-2">
                    <a href="/"><span>Top Tokens</span></a>
                    <span>|</span>
                    <a href="/blocks/icp"><span>Block Explorer</span></a>
                    <span>|</span>
                    <!-- if not logged in -->
                    <a href={memberLink}><span>Members</span></a>
                </div>
            </div>
        </div>
        
    </div>
</header>

<style>
    .head{
        flex: auto;
        background-color: rgba(0, 0, 0, 0.2);
         height: 68px;  /*keep row the same*/
         border-top: 0;
         border-left: 0;
         border-right: 0;
         border-bottom: 1px;
         border-color: rgba(4, 1, 75, 0.8);
         border-style: solid;
    }
    .logo{
        height:44px;
    }
</style>