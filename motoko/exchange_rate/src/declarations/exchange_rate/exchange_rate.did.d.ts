import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CanisterHttpResponsePayload {
  'status' : bigint,
  'body' : Uint8Array,
  'headers' : Array<HttpHeader>,
}
export interface ExchangeRate {
  'get_rates' : ActorMethod<[TimeRange], RatesWithInterval>,
  'test_random_http_with_transform' : ActorMethod<
    [string],
    { 'Ok' : string } |
      { 'Err' : string },
  >,
  'transform' : ActorMethod<[TransformArgs], CanisterHttpResponsePayload>,
}
export interface HttpHeader { 'value' : string, 'name' : string }
export type Rate = string;
export interface RatesWithInterval {
  'interval' : bigint,
  'rates' : Array<[Timestamp, Rate]>,
}
export interface TimeRange { 'end' : Timestamp, 'start' : Timestamp }
export type Timestamp = bigint;
export interface TransformArgs {
  'context' : Uint8Array,
  'response' : CanisterHttpResponsePayload,
}
export interface _SERVICE extends ExchangeRate {}
