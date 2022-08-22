<script lang="ts">
  import {
    dismissNotification,
    Notification,
    notifications,
  } from '../store/notifications';
  import { fly, fade } from 'svelte/transition';

  // const classMap: Record<Notification['type'], string> = {
  //   error: 'alert-error',
  //   success: 'alert-success',
  // };
</script>

<div
  class="absolute right-4 bottom-4 flex flex-col left-4 md:left-auto md:w-96 space-y-4"
>
  {#each $notifications as n (n.id)}
    <div
      class="bg-white flex items-center w-full drop-shadow-md p-4  bg-opacity-100  "
      in:fly={{ x: 100, duration: 200 }}
      out:fade
    >
      {#if n.type === 'error'}
        <svg
          width="32"
          height="32"
          viewBox="0 0 32 32"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <rect width="32" height="32" rx="8" fill="#FBEDDD" />
          <path
            d="M23.1683 22H8.83167C8.0405 22 7.56266 21.1248 7.99049 20.4592L15.1588 9.3085C15.5525 8.69618 16.4475 8.69618 16.8412 9.3085L24.0095 20.4592C24.4373 21.1248 23.9595 22 23.1683 22Z"
            fill="#EC6637"
          />
          <path
            d="M16.7901 12.7273L16.6516 17.8196H15.3519L15.2099 12.7273H16.7901ZM16.0018 20.0923C15.7674 20.0923 15.5662 20.0095 15.3981 19.8438C15.23 19.6757 15.1471 19.4744 15.1495 19.2401C15.1471 19.008 15.23 18.8092 15.3981 18.6435C15.5662 18.4777 15.7674 18.3949 16.0018 18.3949C16.2267 18.3949 16.4244 18.4777 16.5948 18.6435C16.7653 18.8092 16.8517 19.008 16.854 19.2401C16.8517 19.3963 16.8103 19.5395 16.7298 19.6697C16.6516 19.7976 16.5487 19.9006 16.4208 19.9787C16.293 20.0545 16.1533 20.0923 16.0018 20.0923Z"
            fill="white"
          />
        </svg>
      {:else if n.type === 'success'}
        <svg
          width="32"
          height="32"
          viewBox="0 0 32 32"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          class="shrink-0"
        >
          <rect width="32" height="32" rx="4" fill="#E3F7ED" />
          <path
            d="M10 16L14.5 20L22.5 12"
            stroke="#489D72"
            stroke-width="2"
            stroke-linecap="round"
          />
        </svg>
      {/if}

      <p class="ml-2 text-gray-500 flex-1 notification">{n.message}</p>
      <button
        class="text-2xl text-gray-400 w-8 h-8 shrink-0 ml-2"
        on:click={() => dismissNotification(n.id)}>&times;</button
      >
    </div>
  {/each}
</div>

<style>
  .notification {
    word-break: break-word;
  }
</style>
