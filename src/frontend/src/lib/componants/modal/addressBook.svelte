<script>
    import Loading from '../shared/loading.svelte';
    import { createEventDispatcher} from 'svelte';
    import AddressbookTable from '../tables/addressbookTable.svelte';
    import { onMount } from 'svelte';
    import { getAllAccounts, saveAccount, deleteAccount } from '../../code/fetch/addressBook.js';
    import { shortenString } from '../../code/utils.js';
    import Fuse from 'fuse.js';

    export let inputAccount = "";
    export let pageSelect = 0;

    let intervalId;
    let loading = true;
    let addressInput;
    let fuse;
    let searchActive = false;
    let inputName, inputSubAccount;
    let savingAccount =  false;
    let showSaveOutcome = false;
    let saveOutcome = "";
    let tableSwap = 0;

    // FUSE Search
    let searchResults = [];
    const fuseOptions = {
        isCaseSensitive: false,
        keys: [
            {name: 'account', weight: 0.3},
            {name: 'name', weight: 0.7},
        ]
    };

    // fetch all accounts
    let allAccountData;
    let accProcessed = []; 
    let searchProcessed = [];
    async function fetchAllAccounts(){
        accProcessed = [];
        allAccountData = await getAllAccounts();
        let i, dataLen;
        let count = 0;
        let shortAc;
        dataLen = allAccountData[0]?.length ?? 0;
        for(i=0; i<dataLen; i++){
            shortAc= shortenString(allAccountData[0][i][0], 20,10);
            accProcessed.push(
                {count, account: allAccountData[0][i][0], name: allAccountData[0][i][1], shortAccount: shortAc}
            );
            count++;
        }
        fuse = new Fuse(accProcessed, fuseOptions);
        loading = false;
    }

    // onmount
    onMount(()=>{
        fetchAllAccounts();
    });

    // return message to userProfile modal
    const dispatch = createEventDispatcher();
    function returnMessage(message) {
        dispatch('message', message);
        clearInterval(intervalId); 
    }

    // handle message from addressbook table
    async function handleMessage(data){
        // delete account message
        if (data?.detail?.type == "delete"){
            let ot = await deleteAccount(data.detail.account);
            // fetch updated addressbook
            await fetchAllAccounts();
            // redo any search
            handleInputChange();
            // update table
            tableSwap = tableSwap == 0 ? 1 : 0;
        }
    }   

    // search bar impl
    function handleInputChange(){
        if(addressInput?.length != undefined){
            searchResults = [];
            searchProcessed = [];
            searchResults = fuse.search(addressInput);
            // max 50 results
            if (searchResults?.length > 50) {
                searchResults = searchResults.slice(0, 50);
            }
            // process results
            let ref = 0;
            for(let i=0; i<searchResults.length; i++){
                searchProcessed.push({
                    count: ref, 
                    account: searchResults[i]?.item.account,
                    name: searchResults[i]?.item.name,
                    shortAccount: searchResults[i]?.item.shortAccount 
                });
                ref++;
            }
            if (searchProcessed?.length > 0) searchActive = true;
        }
        if(addressInput?.length <= 1){
            searchResults = [];
            searchProcessed = [];
            searchActive = false;
        }
    }

    // save account
    async function saveAccountImpl() {
        savingAccount = true;
        let combined;
        if(inputSubAccount != undefined){
            combined = `${inputAccount}.${inputSubAccount}`;
        } else {
            combined = inputAccount;
        }
        saveOutcome = await saveAccount(combined, inputName);
        // outcome
        showSaveOutcome = true;
        // reset
        setTimeout(()=>{ resetSave() }, 1500)
        // refresh
        await fetchAllAccounts();
    }
    function resetSave(){
        showSaveOutcome = false;
        saveOutcome = "";
        savingAccount = false;
        inputAccount = undefined;
        inputSubAccount = undefined;
        inputName = undefined;
    }

   $: searchResults;
   $: searchProcessed;
   $: accProcessed;
</script>

{#if loading == true}
    <Loading/>
    <div class="flex pb-3 content-center justify-center items-center pt-3">    
        Loading your address book...
    </div>
    
{:else}
    <div class="flex"><p class="h3">Address Book</p></div>
    <div>
        {#if pageSelect == 0}
            <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => {
                    pageSelect = 0; 
                    addressInput = undefined; 
                    searchProcessed = [];
                    searchActive = false;
                }}>
                Search    
            </button>
        {:else}
            <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {
                pageSelect = 0; 
                addressInput = undefined; 
                searchProcessed = [];
                searchActive = false;
            }}>
                Search
            </button>
        {/if}
        <span> | </span>
        {#if pageSelect == 1}
            <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => {pageSelect = 1; }}>
                Add Account    
            </button>
        {:else}
            <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {pageSelect = 1; }}>
                Add Account
            </button>
        {/if}
        <span> | </span>
        {#if pageSelect == 2}
        <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => {pageSelect = 2; }}>
            See All Addresses    
        </button>
    {:else}
        <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => {pageSelect = 2; }}>
            See All Addresses
        </button>
    {/if}
    </div>
    <div class="flex-column pb-4 content-center justify-center items-center">
        {#if pageSelect == 0}
            <div class="flex pl-1 pt-2">
                <p>Search address-book:</p>
            </div>
            <div class="flex pt-2">
                <input class="input pl-2 flex-1 w-2/3" title="Addressbook Search Input" type="text" placeholder="Search by name or account" on:keyup={() => {handleInputChange()}} bind:value={addressInput}/>
            </div>
            {#if searchActive == true}
                {#if tableSwap == 0}
                    <div class="flex pt-4">
                        <AddressbookTable sourceData={searchProcessed} on:message={handleMessage}/>
                    </div>
                {:else}
                    <div class="flex pt-4">
                        <AddressbookTable sourceData={searchProcessed} on:message={handleMessage}/>
                    </div>
                {/if}
            {/if}
        {:else if pageSelect == 1}
            {#if savingAccount == false}
                ADD ACCOUNT
                <div class="flex pt-2">
                    <input class="input pl-2 flex-1 w-2/3" title="Add Account - Name" type="text" placeholder="Name" bind:value={inputName}/>
                </div>
                <div class="flex pt-2">
                    <input class="input pl-2 flex-1 w-2/3" title="Add Account - Account" type="text" placeholder="Account/ Principal" bind:value={inputAccount}/>
                </div>
                <div class="flex pt-2">
                    <input class="input pl-2 flex-1 w-2/3" title="Add Account - Account" type="text" placeholder="Sub-Account (optional)" bind:value={inputSubAccount}/>
                </div>
                <button class="bg-warning-600/80 rounded m-2 mt-3 p-1 pl-2 pr-2" on:click={() => { saveAccountImpl() }}>
                    Save
                </button>
            {:else}
                {#if showSaveOutcome == false}
                    <!-- Saving -->
                    <Loading/>
                    <div class="flex pb-3 content-center justify-center items-center pt-3">    
                        Saving your addressbook...
                    </div>
                {:else}
                    <!-- Outcome of save -->
                    <div class="flex pb-3 content-center justify-center items-center pt-5 bg-warning-600/80">    
                        {saveOutcome}
                    </div>
                {/if}


            {/if}

        {:else if pageSelect == 2}
            {#if tableSwap == 0}
                <div class="flex pt-4">
                    <AddressbookTable sourceData={accProcessed} on:message={handleMessage}/>
                </div>
            {:else}
                <div class="flex pt-4">
                    <AddressbookTable sourceData={accProcessed} on:message={handleMessage}/>
                </div>
            {/if}
        {/if}
    </div>
{/if}   