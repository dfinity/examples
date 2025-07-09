<script>
  import LoggedIn from "../components/LoggedIn.svelte";
  import LoggedOut from "../components/LoggedOut.svelte";
  import { auth } from "../stores/auth";
  import { onMount } from "svelte";

  onMount(() => {
    $auth.init();
  });
</script>

<main>
  {#if $auth.isReady}
    {#if $auth.isAuthenticated}
      <LoggedIn />
    {:else}
      <LoggedOut />
    {/if}
  {/if}
</main>
