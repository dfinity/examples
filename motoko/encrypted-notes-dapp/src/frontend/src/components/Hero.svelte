<script lang="ts">
  import { AuthState, login } from '../store/auth';
  import Spinner from './Spinner.svelte';

  export let auth: Extract<
    AuthState,
    {
      state:
        | 'initializing-auth'
        | 'initializing-crypto'
        | 'synchronizing'
        | 'anonymous'
        | 'error';
    }
  >;
</script>

<div class="hero min-h-screen">
  <div class="text-center hero-content">
    <div class="max-w-xl pb-16">
      <h1 class="mb-5 text-5xl font-bold text-primary dark:text-white">
        Encrypted Notes
      </h1>
      <p class="mb-5 text-xl font-semibold">
        Your private notes on the Internet Computer.
      </p>
      <p class="mb-5">
        A safe place to store your personal lists, thoughts, ideas or
        passphrases and much more...
      </p>

      {#if auth.state === 'initializing-auth' || auth.state === 'initializing-crypto'}
        <div class="text-lg font-semibold mt-8">
          <Spinner />
          Initializing...
        </div>
      {:else if auth.state === 'synchronizing'}
        <div class="text-lg font-semibold">
          <Spinner />
          Synchronizing... Please keep the app open on a device that's already added.
        </div>
      {:else if auth.state === 'anonymous'}
        <button class="btn btn-primary" on:click={() => login()}
          >Please login to start writing notes</button
        >
      {:else if auth.state === 'error'}
        <div class="text-lg font-semibold mt-8">An error occurred.</div>
      {/if}
    </div>
  </div>
  <div class="absolute bottom-0 text-center left-0 right-0 pb-8">
    <img
      src="/img/ic-badge-powered-by-crypto_label-stripe-white-text.png"
      alt="Powered by the Internet Computer"
      class="hidden dark:inline h-4"
    />
    <img
      src="/img/ic-badge-powered-by-crypto_label-stripe-dark-text.png"
      alt="Powered by the Internet Computer"
      class="dark:hidden inline h-4"
    />
  </div>
</div>
