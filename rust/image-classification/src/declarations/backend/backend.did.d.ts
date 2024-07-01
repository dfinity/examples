import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface BoundingBox {
  'top' : number,
  'left' : number,
  'bottom' : number,
  'right' : number,
}
export interface DetectionError { 'message' : string }
export type DetectionResult = { 'Ok' : BoundingBox } |
  { 'Err' : DetectionError };
export interface Embedding { 'v0' : Array<number> }
export interface EmbeddingError { 'message' : string }
export type EmbeddingResult = { 'Ok' : Embedding } |
  { 'Err' : EmbeddingError };
export interface Recognize { 'label' : string, 'score' : number }
export interface RecognizeError { 'message' : string }
export type RecognizeResult = { 'Ok' : Recognize } |
  { 'Err' : RecognizeError };
export interface _SERVICE {
  'add' : ActorMethod<[string, Uint8Array | number[]], EmbeddingResult>,
  'detect' : ActorMethod<[Uint8Array | number[]], DetectionResult>,
  'detect_query' : ActorMethod<[Uint8Array | number[]], DetectionResult>,
  'embedding' : ActorMethod<[Uint8Array | number[]], EmbeddingResult>,
  'recognize' : ActorMethod<[Uint8Array | number[]], RecognizeResult>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
