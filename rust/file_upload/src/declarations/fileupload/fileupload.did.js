export const idlFactory = ({ IDL }) => {
  const ChunkArgs = IDL.Record({
    'chunk_index' : IDL.Nat64,
    'chunk' : IDL.Vec(IDL.Nat8),
    'filename' : IDL.Text,
  });
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const StreamingCallbackToken = IDL.Record({
    'key' : IDL.Text,
    'index' : IDL.Nat64,
    'content_encoding' : IDL.Text,
  });
  const StreamingStrategy = IDL.Variant({
    'Callback' : IDL.Record({
      'token' : StreamingCallbackToken,
      'callback' : IDL.Func([], [], []),
    }),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'upgrade' : IDL.Opt(IDL.Bool),
    'streaming_strategy' : IDL.Opt(StreamingStrategy),
    'status_code' : IDL.Nat16,
  });
  const StreamingCallbackHttpResponse = IDL.Record({
    'token' : IDL.Opt(StreamingCallbackToken),
    'body' : IDL.Vec(IDL.Nat8),
  });
  return IDL.Service({
    'commit_batch' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Text), IDL.Text],
        [IDL.Text],
        [],
      ),
    'create_chunk' : IDL.Func([ChunkArgs], [IDL.Text], []),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'http_request_streaming_callback' : IDL.Func(
        [StreamingCallbackToken],
        [StreamingCallbackHttpResponse],
        ['query'],
      ),
    'list_chunks' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'read' : IDL.Func([IDL.Nat64, IDL.Nat64], [IDL.Vec(IDL.Nat8)], ['query']),
    'stablegrow' : IDL.Func([IDL.Nat64], [IDL.Nat64], []),
    'stablesize' : IDL.Func([], [IDL.Nat64], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
