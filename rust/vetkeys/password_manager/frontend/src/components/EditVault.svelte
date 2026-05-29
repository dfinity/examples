<script lang="ts">
    import { type VaultModel } from "../lib/vault";
    import { vaultsStore } from "../store/vaults";
    import Header from "./Header.svelte";
    import SharingEditor from "./SharingEditor.svelte";
    // @ts-expect-error: svelte-icons have some problems with ts declarations
    import Trash from "svelte-icons/fa/FaTrash.svelte";
    import { auth } from "../store/auth";
    import Spinner from "./Spinner.svelte";
    import { location } from "svelte-spa-router";
    import { Principal } from "@icp-sdk/core/principal";
    import { onDestroy } from "svelte";

    let currentRoute = "";
    const unsubscribeCurrentRoute = location.subscribe((value) => {
        currentRoute = value;
    });
    onDestroy(unsubscribeCurrentRoute);

    let editedVault: VaultModel;
    let updating = false;
    let deleting = false;
    let canManage = false;

    function deleteVault() {}

    $: {
        if (
            $auth.state === "initialized" &&
            $vaultsStore.state === "loaded" &&
            !editedVault &&
            currentRoute.split("/").length > 2
        ) {
            const split = currentRoute.split("/");
            const vaultOwner = Principal.fromText(split[split.length - 2]);
            const vaultName = decodeURIComponent(split[split.length - 1]);
            const vault = $vaultsStore.list.find(
                (v) =>
                    v.owner.compareTo(vaultOwner) === "eq" &&
                    v.name === vaultName,
            );

            if (vault) {
                editedVault = { ...vault };
                const me = $auth.client.getIdentity().getPrincipal();
                if (vault.owner.compareTo(me) === "eq") {
                    canManage = true;
                } else {
                    const user = vault.users.find(
                        ([p]) => p.compareTo(me) === "eq",
                    );
                    if (user) {
                        canManage = "ReadWriteManage" in user[1];
                    }
                }
            }
        }
    }
</script>

{#if editedVault}
    <Header>
        <span slot="title"> Edit vault </span>
        <button
            slot="actions"
            class="btn btn-ghost {deleting ? 'loading' : ''} {!canManage
                ? 'hidden'
                : ''}"
            on:click={deleteVault}
            disabled={updating || deleting}
        >
            {#if !deleting}
                <span class="w-6 h-6 p-1"><Trash /></span>
            {/if}

            {deleting ? "Deleting..." : ""}
        </button>
    </Header>
    <main class="p-4">
        {#if $vaultsStore.state === "loaded"}
            <hr class="mt-10" />
            <SharingEditor {editedVault} {canManage} />
        {:else if $vaultsStore.state === "loading"}
            Loading vaults...
        {/if}
    </main>
{:else}
    <Header>
        <span slot="title"> Edit vault </span>
    </Header>
    <main class="p-4">
        {#if $vaultsStore.state === "loading"}
            <Spinner />
            Loading vault...
        {:else if $vaultsStore.state === "loaded"}
            <div class="alert alert-error">Could not find vault.</div>
        {/if}
    </main>
{/if}
