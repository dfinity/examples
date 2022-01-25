<script>
    import { onMount } from 'svelte';
    import { idlFactory } from '../../../declarations/backend/backend.did.js';

    let plugWalletInstalled = window.ic ? true : false;
    let plugWalletConnected;
    let plugActor;
    let agent;

    const DEX_CANISTER_ID = process.env.DEFI_DAPP_CANISTER_ID;
    const whiteList = [DEX_CANISTER_ID];
    const host =  "localhost:8000"

    
    onMount(async () => {
        if(plugWalletInstalled) {
            if(await window.ic.plug.isConnected) {
                plugWalletConnected = true; 
                // create plug actor        
                await window.ic.plug.createAgent({whiteList, host});
                window.ic.plug.agent.fetchRootKey();
                plugActor = await window.ic.plug.createActor({
                    canisterId: DEX_CANISTER_ID,
                    interfaceFactory: idlFactory,
                    agent: window.ic.plug.agent
                });
                console.log(plugActor)
                const test = await plugActor.whoami();
                console.log(test)

            }
            else {
                let requestConnection = await window.ic.plug.requestConnect();
                if(requestConnection) {
                    plugWalletConnected = true;
                }
                else {
                    plugWalletConnected = false;
                }
            }
        }
	});

    async function connectToPlugWallet() {
        if(window.ic.plug) {
            try {       
                let connected = await window.ic.plug.requestConnect({whiteList});
                if(connected) {
                   plugWalletConnected = true;
                   // create plug actor
                   plugActor = window.ic.plug.createActor({
                       canisterId: DEX_CANISTER_ID,
                       interfaceFactory: idlFactory,
                   });
                }
           }
           catch {
               console.log('Did not connect Plug Wallet')
           }
        }
    }
</script>

<div>
    {#if plugWalletInstalled}
        <div>
            {#if plugWalletConnected}
                <button on:click={connectToPlugWallet} >Connect to Plug Wallet</button>
                {:else}
                <p>You plug wallet is connected!</p>
            {/if}
        </div>
    {:else}
        <div>
            <p>You do not appear to have the Plug Wallet Chrome extension installed</p>
            <p>To Connect to your Plug Wallet, install the Chrome Extension</p>
            <p>
                <a  target='_blank'
                rel='noopener noreferrer'
                href="https://chrome.google.com/webstore/detail/plug/cfbfdhimifdmdehjmkdobpcjfefblkjm">
                Download Here
                </a>
            </p>
        </div>  
    {/if}
</div>