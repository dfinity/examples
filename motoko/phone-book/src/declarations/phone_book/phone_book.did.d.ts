import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Entry { 'desc' : string, 'phone' : Phone }
export type Name = string;
export type Phone = string;
export interface _SERVICE {
  'insert' : ActorMethod<[Name, Entry], undefined>,
  'lookup' : ActorMethod<[Name], [] | [Entry]>,
}
