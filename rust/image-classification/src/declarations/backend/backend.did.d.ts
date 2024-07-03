import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type Addition = { 'Ok' : Embedding } |
  { 'Err' : Error };
export interface BoundingBox {
  'top' : number,
  'left' : number,
  'bottom' : number,
  'right' : number,
}
export type Detection = { 'Ok' : BoundingBox } |
  { 'Err' : Error };
export interface Embedding { 'v0' : Array<number> }
export interface Error { 'message' : string }
export interface Person { 'label' : string, 'score' : number }
export type Recognition = { 'Ok' : Person } |
  { 'Err' : Error };
export interface _SERVICE {
  'add' : ActorMethod<[string, Uint8Array | number[]], Addition>,
  'detect' : ActorMethod<[Uint8Array | number[]], Detection>,
  'recognize' : ActorMethod<[Uint8Array | number[]], Recognition>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
