<script lang="ts">
  import Notifications from './components/Notifications.svelte';
  import LoginPage from './pages/LoginPage.svelte';
  import ReceivePage from './pages/ReceivePage.svelte';
  import SendPage from './pages/SendPage.svelte';
  import WalletPage from './pages/WalletPage.svelte';
  import { auth } from './store/auth';
  import { route } from './store/router';
</script>

<main class="max-w-3xl px-4 mx-auto">
  {#if $auth.state === 'initializing-auth'}
    <p>Initializing...</p>
  {:else if $auth.state === 'anonymous'}
    <LoginPage />
  {:else if $auth.state === 'authenticated'}
    {#if $route == ''}
      <WalletPage auth={$auth} />
    {:else if $route == 'receive'}
      <ReceivePage auth={$auth} />
    {:else if $route == 'send'}
      <SendPage auth={$auth} />
    {/if}
  {:else if $auth.state === 'error'}
    <LoginPage error={$auth.error} />
    <!-- <p>An error occurred initializing Internet Identity.</p> -->
  {/if}

  <Notifications />
</main>

<style lang="postcss" global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;

  @layer components {
    .btn {
      @apply rounded-full h-14 px-4 border-0 font-medium font-inter text-lg flex justify-center items-center;
    }
    .btn-icon {
      @apply rounded-full h-8 w-8 flex justify-center items-center;
    }
    .btn-gray {
      @apply bg-neutral-400 disabled:bg-neutral-600 text-white hover:bg-neutral-500 active:bg-neutral-500 disabled:hover:bg-neutral-600;
    }
    .btn-black {
      @apply bg-black text-white hover:bg-neutral-700 active:bg-neutral-700;
    }
    .btn-blue {
      @apply bg-blue-500 disabled:bg-slate-500 text-white hover:bg-blue-600 active:bg-blue-600 disabled:hover:bg-slate-500;
    }
  }

  @font-face {
    font-family: 'Inter';
    font-weight: 700;
    font-display: swap;
    src: url('/fonts/Inter-Bold.ttf');
  }
  @font-face {
    font-family: 'Inter';
    font-weight: 500;
    font-display: swap;
    src: url('/fonts/Inter-Medium.ttf');
  }
  @font-face {
    font-family: 'Inter';
    font-weight: 400;
    font-display: swap;
    src: url('/fonts/Inter-Regular.ttf');
  }

  html,
  body {
    min-height: 100vh;
  }
</style>
