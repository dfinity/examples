import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'current' : ActorMethod<[], string>,
  'next' : ActorMethod<[], string>,
}
