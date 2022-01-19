import { writable } from 'svelte/store';
import { createActor } from '../lib/actor';

export const auth = writable({
  loggedIn: false,
  actor: createActor(),
});
