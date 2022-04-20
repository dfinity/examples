<script>
    import { Principal } from '@dfinity/principal';
    import * as nftAgent from '../nft';
    export let nft;
    import Loader, {loadSpinner, hideSpinner} from '../components/Loader.svelte';
    import { addTransaction } from '../storage/transactionHistory.js';
    import page from 'page';
    import {Form, FormGroup, Checkbox, Button, TextInput, Select, SelectItem, Loading, ToastNotification } from "carbon-components-svelte";
    // import { loop_guard } from 'svelte/internal';

    export let pageState;
    let loaderId = "transferLoader";
    let message;
    let nextPage = true;
    let principal;
    let notify;
    let confirmed = false;
    $:canSubmit = confirmed && validData({principal, notify});
    let loading = false;
    let showNotification = false;
    let error = false;

    function showSnackbar() {
        showNotification = true;
    }
    function hideSnackbar() {
        showNotification = false;
        if (nextPage) {
            pageState.transactions = true;
            pageState = pageState;
            page('/transactions');
        } else {
            page('/');
        }
    }
    function transfer() {
        if (!validData({principal, notify})) {
            return;
        }
        let shouldNotify;
        switch (notify) {
            case 'no':
                shouldNotify = false;
                break;
            case 'yes':
                shouldNotify = true;
                break;
            case 'maybe':
                shouldNotify = null;
                break;
        }
        loading = true;

        nftAgent.transfer(nft, principal, shouldNotify).then((result) => {
            loading = false;
            console.log(JSON.stringify(result));
            if ("Err" in result) {
                error = true;
                console.error(JSON.stringify(result.Err))
                message = JSON.stringify(result.Err);
                nextPage = false;
            }
            if ("Ok" in result) {
                message = "Successfully transferred."
                addTransaction(nft.index, `Transferred NFT to ${principal}`);
            }
            showSnackbar();
        });
    }
    let errorMessage1;
    let errorMessage2;
    function validData({principal, notify}) {
        if (principal) {
            try {
                Principal.fromText(principal);
            } catch {
                errorMessage1 = 'Invalid principal';
                return false;
            }
        }
        if (notify) {
            if (!(['no', 'maybe', 'yes'].includes(notify))) {
                errorMessage2 = 'Select a valid notify option';
                return false;
            }
        }
        errorMessage1 = null;
        errorMessage2 = null;
        return true;
    }
    let _isAuthorized = nftAgent.isAuthorized();
</script>

<div id="transferView"><Loading active={loading}/>
    <Loader named={loaderId}/>
    {#await _isAuthorized then isAuthorized}
    {#if isAuthorized}

    {#if showNotification}
    <div id="toast">
        {#if error}
        <ToastNotification
            kind="error" title="Failed"
            subtitle={message}
            caption={new Date().toLocaleString()}>
                <Button style="margin-bottom: 1.5em; height: 2.5em; min-height: 0;" on:click={(e) => {e.preventDefault(); hideSnackbar()}}>Okay</Button>
        </ToastNotification>
        {/if}
        <ToastNotification
            kind="success" title="Success"
            subtitle={message}
            caption={new Date().toLocaleString()}>
                <Button style="margin-bottom: 1.5em; height: 2.5em; min-height: 0;" on:click={(e) => {e.preventDefault(); hideSnackbar()}}>Okay</Button>
        </ToastNotification>
    </div>
    {/if}
        <Form style="padding: 50px 30px 30px; border: solid 1px grey; broder-radius: 10px; border-radius: 15px;"
            on:submit={(e) => {
                e.preventDefault();
                transfer();
            }}
            ><h2>Transfer {nft.name} ({nft.symbol}) #{nft.index}</h2>
            <FormGroup>
                <TextInput size="large" labelText="Principal to transfer to" placeholder="Enter valid Principal..." bind:value={principal} on:change={(e) => validData({principal})}/>
                {#if errorMessage1}
                    <p class="error">{errorMessage1}</p>
                {/if}
            </FormGroup>
            <FormGroup>
                <Select labelText="Notify Recipient?" selected="select" on:change={(e) =>
                    notify=e.detail}>
                    <SelectItem value="select" text="Select from the following..." />
                    <SelectItem value="no" text="No" />
                    <SelectItem value="maybe" text="Yes, but transfer anyway if unsupported" />
                    <SelectItem value="yes" text="Yes, and don't transfer without it" />
                </Select>
                {#if errorMessage2}
                    <p class="error">{errorMessage2}</p>
                {/if}
            </FormGroup>
            <FormGroup>
                <Checkbox id="checkbox-0" labelText="I understand that if this principal is wrong, I'm probably not getting this NFT back." bind:checked={confirmed}/>
            </FormGroup>
            <div style="display: flex; justify-content: space-between;">
                <Button kind="secondary" style="width: 48%;" on:click={(e)=> page(`/${nft.canister}/${nft.index}`)} type="button">Cancel</Button>
                <Button style="width: 48%;" disabled={canSubmit? false : true} type="submit">Transfer NFT</Button>
            </div>
        </Form>
    {:else}
        <p>You must be an authorized user to transfer NFTs out of this wallet.</p>
    {/if}
    {:catch}
    <p>You can't transfer NFTs out of this wallet if the canister can't be accessed.</p>
    {/await}
</div>

<style>
    h2 {
        /* text-align: center; */
        padding-bottom: 1em;
        font-size: 30px;
    }
    #transferView {
        padding-top: 10%;
        padding-bottom: 10%;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    #bt {
        border-radius: 2px;
        margin: 5px;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 0.7em;
        color: white;
    }
    #toast {
        position: fixed;
        margin: auto;
        top: 200px;
        z-index: 10;
    }
</style>
