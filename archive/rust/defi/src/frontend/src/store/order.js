import { writable } from 'svelte/store';

export const orders = writable([]);
export const currentOrder = writable({});