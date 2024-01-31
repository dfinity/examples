import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type List = [] | [[string, List]];
export interface Superhero { 'superpowers' : List, 'name' : string }
export type SuperheroId = number;
export interface _SERVICE {
  'create' : ActorMethod<[Superhero], SuperheroId>,
  'delete' : ActorMethod<[SuperheroId], boolean>,
  'read' : ActorMethod<[SuperheroId], [] | [Superhero]>,
  'update' : ActorMethod<[SuperheroId, Superhero], boolean>,
}
