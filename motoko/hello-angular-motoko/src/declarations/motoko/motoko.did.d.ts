import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'greet' : ActorMethod<[string], string>,
  'test' : ActorMethod<[], string>,
}
