import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Main {
  'getLogs' : ActorMethod<[], Array<string>>,
  'getMerchant' : ActorMethod<[], Response>,
  'setCourierApiKey' : ActorMethod<[string], Response_1>,
  'setLedgerId' : ActorMethod<[string], Response_1>,
  'updateMerchant' : ActorMethod<[Merchant], Response>,
}
export interface Merchant {
  'email_address' : string,
  'phone_notifications' : boolean,
  'name' : string,
  'email_notifications' : boolean,
  'phone_number' : string,
}
export interface Response {
  'status' : number,
  'data' : [] | [Merchant],
  'status_text' : string,
  'error_text' : [] | [string],
}
export interface Response_1 {
  'status' : number,
  'data' : [] | [string],
  'status_text' : string,
  'error_text' : [] | [string],
}
export interface _SERVICE extends Main {}
