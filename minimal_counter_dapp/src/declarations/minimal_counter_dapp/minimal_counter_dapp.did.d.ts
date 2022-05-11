import type { Principal } from '@dfinity/principal';
export interface _SERVICE {
  'get' : () => Promise<bigint>,
  'increment' : () => Promise<bigint>,
  'set' : (arg_0: bigint) => Promise<undefined>,
}
