import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Rate = number;
export type RatesMap = Array<[Timestamp, Rate]>;
export interface RatesWithInterval { 'interval' : bigint, 'rates' : RatesMap }
export interface TimeRange { 'end' : Timestamp, 'start' : Timestamp }
export type Timestamp = bigint;
export interface _SERVICE {
  'get_rates' : ActorMethod<[TimeRange], RatesWithInterval>,
}
