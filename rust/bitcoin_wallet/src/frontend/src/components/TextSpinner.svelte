<script lang="ts">
  import { onDestroy, onMount } from 'svelte';

  let variants = ['...', '..', '.', '..'];
  let index = 0;
  let intervalHandle: ReturnType<typeof setInterval> | null = null;

  onMount(() => {
    intervalHandle = setInterval(() => {
      index = (index + 1) % variants.length;
    }, 300);
  });

  onDestroy(() => {
    if (intervalHandle !== null) {
      clearInterval(intervalHandle);
    }
  });
</script>

{variants[index]}
