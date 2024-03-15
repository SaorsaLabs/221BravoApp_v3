<script>
    import logo from '$lib/images/221B_sml.png';
    import icLogo from '$lib/images/InternetComputer.png';
</script>


<footer class="bg-surface-700 text-white border-b-2 border-primary-700/10 drop-shadow-lg">

   	
    <div class="container mx-auto px-4 flex flex-wrap min-w-full min-w-full md:min-w-0">
        <div class="w-full flex flex-col lg:flex-row gap-2 pb-10 pt-4">
            <!-- OUTER COL 1 -->
            <div class="rounded lg:w-1/2 w-full">
                
                <!-- INNER COL 1-->
                <div class="container flex flex-wrap min-w-full min-w-full md:min-w-0">
                    <div class="w-full flex gap-2">
                        <div class="p-1 rounde w-1/2">
                            <a href="/"><img class="pt-2 logo" src={logo} alt="221Bravo Logo" /></a>
                            <p class="text-sm pt-2">Saorsa Labs 2024 ©️</p>

                            <p class="pt-6 text-sm pl-1">Powered by </p>
                            <a href="https://internetcomputer.org/"><img class="pt-2 icLogo" src={icLogo} alt="Internet Computer Logo"/></a>
                        </div>
                        <div class="p-1 rounded w-1/2">
                            <p class="text-lg pt-1 text-tertiary-500"><b>App</b></p>
                            <a href="/"><p class="pt-2">Home</p></a>
                            <a href="/"><p class="pt-2">Search Accounts</p></a>
                            <a href="/blocks/ICP"><p class="pt-2">Search Blocks</p></a>
                            <a href="/members"><p class="pt-2">Members</p></a>
                        </div>
                    </div>
                </div>
            </div>
            <!-- OUTER COL 2 -->
            <div class="rounded lg:w-1/2 w-full">
                <!-- INNER COL 2 -->
                <div class="container flex flex-wrap min-w-full min-w-full md:min-w-0">
                    <div class="w-full flex gap-2">
                        <div class="p-1 rounde w-1/2">
                            <p class="text-lg pt-1 text-tertiary-500"><b>About</b></p>
                            <a href="https://internetcomputer.org/"><p class="pt-2">Internet Computer 
                                <svg viewBox="0 0 24 24" focusable="false" class="outLink -mt-1"><g fill="none" stroke="currentColor" stroke-linecap="round" stroke-width="2"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path><path d="M15 3h6v6"></path><path d="M10 14L21 3"></path></g></svg>
                            </p>
                            </a>
                            <a href="/terms"><p class="pt-2">Terms of Service</p></a>
                            <a href="/terms"><p class="pt-2">Privacy</p></a>
                        </div>
                        <div class="p-1 rounded w-1/2">
                            <p class="text-lg pt-1 text-tertiary-500"><b>More</b></p>
                            <a href="https://twitter.com/221BravoApp"><p class="pt-2">Twitter 
                                <svg viewBox="0 0 24 24" focusable="false" class="outLink -mt-1"><g fill="none" stroke="currentColor" stroke-linecap="round" stroke-width="2"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path><path d="M15 3h6v6"></path><path d="M10 14L21 3"></path></g></svg>
                                </p>
                            </a>
                            <a href="https://oc.app/community/vrtdr-dyaaa-aaaar-a2exq-cai/channel/283806532254715438641103320620325336219"><p class="pt-2">Open Chat 
                                <svg viewBox="0 0 24 24" focusable="false" class="outLink -mt-1"><g fill="none" stroke="currentColor" stroke-linecap="round" stroke-width="2"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path><path d="M15 3h6v6"></path><path d="M10 14L21 3"></path></g></svg>
                                </p>
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</footer>

<style>
    .logo{
        height:50px;
    }
    .outLink{
        height: 1em;
        width: 1em;
        display: inline-block;
        vertical-align: middle;
        margin-inline: 2px;
    }

    .icLogo{
        width: 175px;
        display:flex;
        vertical-align: middle;
        margin-inline: 5px;
    }
</style>