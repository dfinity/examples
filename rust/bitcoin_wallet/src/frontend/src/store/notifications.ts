import { writable } from 'svelte/store';

export interface Notification {
  type: 'error' | 'success';
  message: string;
  id: number;
}

export type NewNotification = Omit<Notification, 'id'>;

let nextId = 0;

export const notifications = writable<Notification[]>([]);

export function addNotification(notification: NewNotification, timeout = 2000) {
  const id = nextId++;

  notifications.update(($n) => [...$n, { ...notification, id }]);

  if (timeout > 0) {
    setTimeout(() => {
      notifications.update(($n) => $n.filter((n) => n.id != id));
    }, timeout);
  }
}

export function showError(e: any, message: string): never {
  addNotification({ type: 'error', message });
  console.error(e.stack);
  throw e;
}

export function dismissNotification(id: number) {
  notifications.update(($n) => $n.filter((n) => n.id != id));
}
