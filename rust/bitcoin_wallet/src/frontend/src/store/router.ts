import { writable } from 'svelte/store';

function createRouter() {
  const hash = window.location.hash.slice(1);
  const { subscribe, set } = writable(hash);

  window.addEventListener('hashchange', () => {
    set(window.location.hash.slice(1));
  });

  return {
    subscribe,

    navigate: function (to: string) {
      window.location.hash = to;
    },
  };
}

export const route = createRouter();
