import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'get' : ActorMethod<
    [],
    { 'certificate' : [] | [Uint8Array], 'value' : number }
  >,
  'inc' : ActorMethod<[], number>,
  'set' : ActorMethod<[number], undefined>,
}
