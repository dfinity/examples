import type { Principal } from '@dfinity/principal';
export interface _SERVICE {
  'current' : () => Promise<string>,
  'next' : () => Promise<string>,
}