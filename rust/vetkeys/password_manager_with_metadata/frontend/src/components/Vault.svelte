<script lang="ts">
    import { type VaultModel, summarize } from "../lib/vault";
    import { link, location } from "svelte-spa-router";
    import { onDestroy } from "svelte";
    import { vaultsStore } from "../store/vaults";
    import { Principal } from "@icp-sdk/core/principal";
    import Header from "./Header.svelte";
    import Spinner from "./Spinner.svelte";
    // @ts-expect-error: svelte-icons have some problems with ts declarations
    import GiOpenTreasureChest from "svelte-icons/gi/GiOpenTreasureChest.svelte";
    import { auth } from "../store/auth";
    import SharingEditor from "./SharingEditor.svelte";
    import type { AccessRights } from "@icp-sdk/vetkeys/encrypted_maps";

    export let vault: VaultModel = {
        name: "",
        owner: Principal.managementCanister(),
        passwords: [],
        users: [],
    };
    export let vaultSummary: string = "";
    export let accessRights: AccessRights = { Read: null };

    export let currentRoute = "";
    const unsubscribeCurrentRoute = location.subscribe((value) => {
        currentRoute = decodeURI(value);
    });
    onDestroy(unsubscribeCurrentRoute);

    // Parse owner and vault name from the URL once; stored separately so the
    // vault lookup below stays reactive to store updates (e.g. from the poller).
    let parsedRoute: { owner: Principal; vaultName: string } | null = null;
    $: if (currentRoute.split("/").length > 2 && parsedRoute === null) {
        const split = currentRoute.split("/");
        parsedRoute = {
            owner: Principal.fromText(split[split.length - 2]),
            vaultName: split[split.length - 1], // already decoded via decodeURI on subscribe
        };
    }

    // Re-runs whenever the store updates so new passwords from the poller appear.
    $: if (
        $vaultsStore.state === "loaded" &&
        $auth.state === "initialized" &&
        parsedRoute !== null
    ) {
        const { owner: targetOwner, vaultName: targetVaultName } = parsedRoute;
        const searchedForVault = $vaultsStore.list.find(
            (v) =>
                v.owner.compareTo(targetOwner) === "eq" &&
                v.name === targetVaultName,
        );
        if (!searchedForVault) {
            vaultSummary =
                "could not find vault " +
                targetVaultName +
                " owned by " +
                targetOwner.toText();
        } else {
            vault = searchedForVault;
            vaultSummary = summarize(vault);
            void $auth.client.getIdentity().then((identity) => {
                const me = identity.getPrincipal();
                if (vault.owner.compareTo(me) === "eq") {
                    accessRights = { ReadWriteManage: null };
                } else {
                    const foundRights = vault.users.find(
                        (user) => user[0].compareTo(me) === "eq",
                    );
                    accessRights = foundRights
                        ? foundRights[1]
                        : { Read: null };
                }
            });
        }
    }
</script>

<Header>
    <span slot="title" class="flex h-full items-center gap-2">
        <span style="width: 64px; height: 64px;" class="inline-block">
            <GiOpenTreasureChest />
        </span>
        Vault: {vault.name}
    </span>
    <svelte:fragment slot="actions">
        {#if $vaultsStore.state === "loaded" && $vaultsStore.list.length > 0}
            <a class="btn btn-primary" href="/" use:link>New password</a>
        {/if}
    </svelte:fragment>
</Header>

<main class="relative flex min-h-screen flex-col p-4 pb-24">
    {#if $vaultsStore.state === "loading"}
        <Spinner />
        Loading vault...
    {:else if $vaultsStore.state === "loaded"}
        <div class="pointer-events-none">
            <h2 class="mb-2 line-clamp-3 text-lg font-bold">
                {vaultSummary}
            </h2>
        </div>
        <div class="mt-5"></div>
        <SharingEditor
            editedVault={vault}
            canManage={"ReadWriteManage" in accessRights}
        />

        <div class="mt-5"></div>

        <div class="pointer-events-none">
            <h2 class="mb-2 line-clamp-3 text-lg font-bold">Passwords</h2>
        </div>
        {#if vault.passwords.length === 0}
            <div class="pt-8 text-center italic">
                You don't have any passwords in this vault.
            </div>
            <div class="pt-8 text-center">
                <a href="/" use:link class="btn btn-primary"
                    >Add a new password</a
                >
            </div>
        {:else}
            <div
                class="grid max-w-7xl grid-cols-1 gap-3 sm:grid-cols-2
            md:grid-cols-3"
            >
                {#each vault.passwords as password ((password[1].owner, password[1].parentVaultName, password[1].passwordName))}
                    <a
                        class="bg-base rounded-md·border·border-base-300·p-4·transition-transform⏎hover:-translate-y-2·dark:border-base-300·dark:bg-base-100"
                        use:link
                        href={`/edit/vaults/${vault.owner.toText()}/${vault.name}/${password[1].passwordName}`}
                    >
                        <div class="pointer-events-none">
                            <h2
                                class="mb-2 line-clamp-3 text-lg font-bold"
                                style="word-break: break-all;"
                            >
                                {password[1].passwordName}: "{password[1]
                                    .content}"
                            </h2>
                        </div>
                    </a>
                {/each}
            </div>
        {/if}
        <div class="flex-grow"></div>
        <div class="text-center">
            <a href="/vaults" use:link class="btn btn-primary">
                Back to overview
            </a>
        </div>
    {/if}
</main>
