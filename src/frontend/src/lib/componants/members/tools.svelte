<script>
    import { getModalStore} from '@skeletonlabs/skeleton';
	import MemberServicesCombined from '../modal/memberServicesCombined.svelte';

    // for popup
	const modalStore = getModalStore();

    async function handleAddressClick(){
		const c = { ref: MemberServicesCombined };
		const modal = {
			type: 'component',
			component: c,
			title: 'Custom Form Component',
			body: '',
			meta: { modalType: "addressbook", pageSelect: 0, saveAccount: undefined}, 
			//response: (r) => console.log('response:', r)
		};
		modalStore.trigger(modal);
	}

	async function handleAlertClick(){
		const c = { ref: MemberServicesCombined };
		const modal = {
			type: 'component',
			component: c,
			title: 'Custom Form Component',
			body: '',
			meta: { modalType: "pricealerts" },
			//response: (r) => console.log('response:', r)
		};
		modalStore.trigger(modal);
	}
</script>

<div class="flex flex-col lg:flex-row gap-4 pr-2">
        <div class="bg-primary-400/40 flex-1 rounded p-1 pl-2">
            TOOLS: 
            <div class="w-full" style="display: flex; justify-content: center;">
                <div class="flex flex-row gap-4 pr-2 w-80">
                    <div class="flex-1 rounded p-1 pl-2">
                        <a href="./account+">
                            <button class="bg-secondary-500 p-2 rounded">
                                <p>Account View +</p>
                            </button>
                        </a>
                    </div>
                    <div class="flex-1 rounded p-1 pl-2">
                        <a href="./blocks+">
                            <button class="bg-secondary-500 p-2 rounded">
                                <p>Block View +</p>
                            </button>
                        </a>
                    </div>
                    <div class="flex-1 rounded p-1 pl-2">
                        <a href="https://oc.app/community/vrtdr-dyaaa-aaaar-a2exq-cai/channel/283806532254715438641103320620325336219" target="_blank">
                            <button class="bg-secondary-500 p-2 rounded">
                                <p>Chat (OC)</p>
                            </button>
                        </a>
                    </div>
                </div>
            </div>
        </div>
    

    <div class="bg-primary-400/40 flex-1 rounded p-1 pl-2">
        YOUR DATA: 
        <div class="w-full" style="display: flex; justify-content: center;">
            <div class="flex flex-row gap-4 pr-2 w-80">
                <div class="flex-1 rounded p-1 pl-2">
                        <button class="bg-secondary-500 p-2 rounded" on:click={()=>{ handleAddressClick() }}>
                            <p>Address Book</p>
                        </button>
                </div>
                <div class="flex-1 rounded p-1 pl-2">
                    <button class="bg-secondary-500 p-2 rounded" on:click={()=>{ handleAlertClick() }} >
                        <p>Price Alerts</p>
                    </button>
                </div>
            </div>
        </div>
    </div>
    <div class="bg-primary-400/40 flex-1 rounded p-1 pl-2">
        PARTNERS: 
        <div class="w-full" style="display: flex; justify-content: center;">
            <div class="flex flex-row gap-4 pr-2 w-80">
                <div class="flex-1 rounded p-1 pl-2">
                    <a href="https://t5t44-naaaa-aaaah-qcutq-cai.raw.icp0.io/" target="_blank">
                        <button class="bg-secondary-500 p-2 rounded">
                            <p>NFT Geek</p>
                        </button>
                    </a>
                </div>
                <div class="flex-1 rounded p-1 pl-2">
                    <a href="https://f2djv-5aaaa-aaaah-qdbea-cai.raw.icp0.io/" target="_blank">
                        <button class="bg-secondary-500 p-2 rounded pl-4 pr-4">
                            <p>VP <br/> Geek</p>
                        </button>
                    </a>
                </div>
                <div class="flex-1 rounded p-1 pl-2">
                     <a href="https://xdtth-dyaaa-aaaah-qc73q-cai.raw.icp0.io/" target="_blank">
                        <button class="bg-secondary-500 p-2 pl-4 pr-4 rounded">
                            <p>ID <br/>  Geek</p>
                        </button>
                     </a>
                </div>
            </div>
        </div>
    </div>
</div>