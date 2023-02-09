import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface ChunkArgs {
  'chunk_index' : bigint,
  'chunk' : Uint8Array,
  'filename' : string,
}
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Uint8Array,
  'headers' : Array<[string, string]>,
}
export interface HttpResponse {
  'body' : Uint8Array,
  'headers' : Array<[string, string]>,
  'upgrade' : [] | [boolean],
  'streaming_strategy' : [] | [StreamingStrategy],
  'status_code' : number,
}
export interface StreamingCallbackHttpResponse {
  'token' : [] | [StreamingCallbackToken],
  'body' : Uint8Array,
}
export interface StreamingCallbackToken {
  'key' : string,
  'index' : bigint,
  'content_encoding' : string,
}
export type StreamingStrategy = {
    'Callback' : {
      'token' : StreamingCallbackToken,
      'callback' : [Principal, string],
    }
  };
export interface _SERVICE {
  'commit_batch' : ActorMethod<[string, Array<string>, string], string>,
  'create_chunk' : ActorMethod<[ChunkArgs], string>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'http_request_streaming_callback' : ActorMethod<
    [StreamingCallbackToken],
    StreamingCallbackHttpResponse
  >,
  'list_chunks' : ActorMethod<[], Array<string>>,
  'read' : ActorMethod<[bigint, bigint], Uint8Array>,
  'stablegrow' : ActorMethod<[bigint], bigint>,
  'stablesize' : ActorMethod<[], bigint>,
}
