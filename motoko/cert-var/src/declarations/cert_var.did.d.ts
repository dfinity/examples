import type { Principal } from '@dfinity/principal';
export interface _SERVICE {
  'get' : () => Promise<
      { 'certificate' : [] | [Array<number>], 'value' : number }
    >,
  'inc' : () => Promise<number>,
  'set' : (arg_0: number) => Promise<undefined>,
}
