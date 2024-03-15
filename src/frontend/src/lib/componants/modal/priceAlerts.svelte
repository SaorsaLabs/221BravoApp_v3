<script>
import { updateOCID, getUserData, addUserAlert, getUserAlerts, removeUserAlert } from '../../code/fetch/userData';
import BasicTable from '../tables/basicTable.svelte';
import { onMount } from 'svelte';
import Loading from '../shared/loading.svelte';
import HelpIconLight from '$lib/images/icons/helpLight.png';
import { getAllTokenData } from '../../code/utils';

let data;
let head = [
    {public: "Alert ID", data: "id", button: false}, 
    {public: "Cross", data: "cross", button: false}, 
    {public: "Direction", data: "arrow", button: false},  
    {public: "Price Level", data: "price", button: false},
    {public: "-", data: "button", button: true, buttonText: "Delete Alert"},
];
let dataLoaded = false;
export let showPage = "Loading";
export let saveCross = "ICP/USD";

onMount( () => { 
    if (showPage != "newAlert") { 
        loadData(); 
    } else {
        loadDropdown(); 
    }
});

let ocIDInput;
let backendUserData;
let hasError = false;
let errorText = "";
let showOCSaving = false;
let ocSavingOutput  = "";
let noAlerts = false;
let firstLoad = false;
let dropDownloaded = false; 
let allTokens;
let directionDropdown = undefined; // 1 is UP 0 is DN
let alertPrice;
let savingAlert = false;
let alertWarningText = "";

function loadDropdown(){
    dropDownloaded = false;
    allTokens = getAllTokenData();
    dropDownloaded = true;
}

// Load User Data/ Alerts
async function loadData(){
    backendUserData = await getUserData();
    //console.log(backendUserData);
    if (backendUserData == "Not Logged In") { 
        hasError = true;
        errorText = "You are not logged in. Please login and try again.";
        showPage = "Error";
     } else if (backendUserData[0].user_oc_principal.length > 0) {
        showPage = "Alerts";
        await fetchUserAlerts();
    } else {
        firstLoad = true;
        showPage = "SetOCID";
    }
}

// Save OpenChat ID
async function ocSaveClick(){
    ocSavingOutput = "";
    // check input
    if (ocIDInput == "" || ocIDInput == undefined){
        ocSavingOutput = "Please add your Openchat ID. Click the ? if you need help."
    }
    showOCSaving = true;
    let update = await updateOCID(ocIDInput);
    ocSavingOutput = update;
    showOCSaving = false;
    showPage = "Alerts";
}

// Impl for fetching User Alerts. 
async function fetchUserAlerts(){
    showPage = "Loading";
    let res = await getUserAlerts();
    let resLen = res[0]?.length ?? 0;
    if (resLen == 0) {
        noAlerts = true;
        dataLoaded = true;
    } else {
        data = res[0];
        for(let i=0; i<data.length; i++){
            if (data[i].direction == "0") { data[i].arrow = "⬇️ Cross Below" } else { data[i].arrow = "⬆️ Cross Above" };
        }
        dataLoaded = true;
    }
    showPage = "Alerts";
}

// Remove user alert
async function deleteAlert(event){
    showPage = "Loading";
    await removeUserAlert(event.detail.cross, event.detail.id);
    await fetchUserAlerts();
}

// Set New Alert
async function addAlert(){
    savingAlert = true;
    alertWarningText = "";
    // validate input
    if (directionDropdown == undefined) {
        alertWarningText = "Please select a direction for the alert";
        savingAlert = false;
        return;
    }
    if (alertPrice <= 0 || alertPrice == undefined) {
        alertWarningText = "Alert price must be greater than 0";
        savingAlert = false;
        return;
    }
    let save = await addUserAlert(saveCross, Number(alertPrice), Number(directionDropdown));
    alertWarningText = `Alert added ID : ${save}`;
    alertPrice = undefined;
    directionDropdown = undefined;
    savingAlert = false;
}

// async function addAlert(){
//     console.log("CLICKED")
//     let data = await addUserAlert("CHAT/ICP", 0.60, 0);
//     console.log(data);
// }



$: data;
$: showPage;
</script>

<!-- HEADING -->
{#if showPage == "SetOCID" && firstLoad == false}
    <div class="flex"><p class="h3">Update OpenChat ID</p></div>
{:else if showPage == "Alerts"}
    <div class="flex"><p class="h3">🔔 User Alerts</p></div>
{/if}

<!-- BUTTONS -->
{#if firstLoad == false}
    {#if showPage == "Alerts"}
        <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => { showPage = "Alerts" }}>
            Price Alerts  
        </button>
    {:else}
        <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => { showPage = "Alerts"; fetchUserAlerts(); }}>
            Price Alerts 
        </button>
    {/if}
    <span> | </span>
    {#if showPage == "SetOCID"}
        <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => { showPage = "SetOCID" }}>
            Update ID 
        </button>
    {:else}
        <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => { showPage = "SetOCID" }}>
            Update ID
        </button>
    {/if}
    <span> | </span>
    {#if showPage == "newAlert"}
        <button class="bg-warning-600/80 rounded m-2 pl-1 pr-1" on:click={() => { () => { showPage = "newAlert"; } }}>
            Add New
        </button>
    {:else}
        <button class="bg-tertiary-500/50 rounded m-2 pl-1 pr-1" on:click={() => { showPage = "newAlert"; alertWarningText = ""; }}>
           Add New
        </button>
    {/if}
{/if}

{#if showPage == "Loading" || showPage == 0}
    <Loading/>
    <div class="flex pb-3 content-center justify-center items-center pt-3">    
        Loading Alert Data...
    </div>
{:else if showPage == "SetOCID"}
    <div class="flex flex-col w-full pb-3 content-center justify-center items-center">    
        <div class="flex text-warning-500"><p>Add your OpenChat user ID to use Price Alerts
            <a href="https://221bravo-app.gitbook.io/guide/app/price-alerts" target="_blank">
                <button class="float-right pr-1 pl-2">
                    <img src={HelpIconLight} alt="Help Logo" width="20px"/>
                </button>
            </a>
        </p></div>
        <div class="flex w-80 pt-2">
            <input class="input pl-2" type="text" placeholder="Open Chat ID" bind:value={ocIDInput}/>
        </div>
        <div class="flex w-full justify-center">
            {#if showOCSaving == true}
                <div class="pt-2">
                    <Loading/>
                </div>
            {:else}
                {ocSavingOutput}
            {/if}
        </div>
        <div class="flex">
            <button class="bg-warning-600/80 rounded m-2 mt-3 p-1 pl-2 pr-2" on:click={()=>{ ocSaveClick()}}>
                SAVE
            </button>
        </div>
    </div>
{:else if showPage == "Alerts"}
    {#if dataLoaded == true && noAlerts == false}
        <div class="pt-2">
            <BasicTable data={data} tableHeaders={head} on:dataPassed={deleteAlert}/>
        </div>
    {:else if dataLoaded == true && noAlerts == true}
        <p class="pt-2"> You don't have any Price Alerts yet </p>  
    {/if}
{:else if showPage == "newAlert"}

    <div class="flex flex-col sm:flex-row gap-4 pr-2">
        <div class="flex-1 p-1 pl-2">
            <p class="pb-1">Token:</p>
            {#if dropDownloaded == true}
                <select class="select" bind:value={saveCross}>
                    {#each allTokens as T}
                        <option value={T.tradePair} >{T.tradePair}</option>
                    {/each}
                </select>
            {/if}
        </div>
        <div class="flex-1 rounded p-1 pl-2">
            <p class="pb-1">Direction:</p>
            <select class="select" bind:value={directionDropdown}>
                <option value=1>Cross Over (UP)</option>
                <option value=0>Cross Under (DOWN)</option>
            </select>
        </div>
        <div class="flex-1 rounded p-1 pl-2">
            <p class="pb-1">Alert Price ($):</p>
            <input class="input pl-3 p-2" type="number" placeholder="Alert Price" bind:value={alertPrice}/>
        </div>
    </div>
    <p class="pl-3">{alertWarningText}</p>
    {#if savingAlert == false}
        <div class="w-11/12"> 
            <button class="bg-warning-600/80 rounded m-2 mt-3 p-1 pl-2 pr-2 float-right" on:click={()=>{ addAlert() }}>
                SAVE
            </button>
        </div>
    {:else}
        <Loading/>
    {/if}
    
{:else if showPage == "Error"}
    Error Page
{/if}




