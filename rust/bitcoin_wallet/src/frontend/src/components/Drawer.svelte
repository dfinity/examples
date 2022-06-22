<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let open = false;

  const dispatch = createEventDispatcher<{ close: void }>();
</script>

<div
  class="bg-black bg-opacity-50 fixed inset-0 transition-opacity {open
    ? 'opacity-100'
    : 'pointer-events-none invisible opacity-0'}"
  on:click={() => dispatch('close')}
/>

<div
  class="fixed bottom-0 left-0 w-full md:w-[768px] md:left-[calc(50%-384px)] transition-transform p-12 rounded-t-2xl bg-white {open
    ? 'translate-y-0'
    : 'pointer-events-none invisible translate-y-full'}"
>
  <slot />
</div>
