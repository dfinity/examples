import type { ActorMethod } from "@dfinity/agent";

export interface Merchant {
  name: string;
  email_notifications: boolean;
  email_address: string;
  phone_notifications: boolean;
  phone_number: string;
}

export interface Response<T> {
  status: number;
  status_text: string;
  data: T | null;
  error_text: string | null;
}

export interface _SERVICE {
  getMerchant: ActorMethod<[], Response<[Merchant]>>;
  updateMerchant: ActorMethod<[Merchant], Response<[Merchant]>>;
  setCourierApiKey: ActorMethod<[string], string>;
  getLogs: ActorMethod<[], string[]>;
}
