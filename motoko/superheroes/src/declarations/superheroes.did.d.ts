import type { Principal } from '@dfinity/principal';
export type List = [] | [[string, List]];
export interface Superhero { 'superpowers' : List, 'name' : string }
export type SuperheroId = number;
export interface _SERVICE {
  'create' : (arg_0: Superhero) => Promise<SuperheroId>,
  'delete' : (arg_0: SuperheroId) => Promise<boolean>,
  'read' : (arg_0: SuperheroId) => Promise<[] | [Superhero]>,
  'update' : (arg_0: SuperheroId, arg_1: Superhero) => Promise<boolean>,
}