export const idlFactory = ({ IDL }) => {
  const Timestamp = IDL.Nat64;
  const TimeRange = IDL.Record({ 'end' : Timestamp, 'start' : Timestamp });
  const Rate = IDL.Text;
  const RatesWithInterval = IDL.Record({
    'interval' : IDL.Nat64,
    'rates' : IDL.Vec(IDL.Tuple(Timestamp, Rate)),
  });
  const HttpHeader = IDL.Record({ 'value' : IDL.Text, 'name' : IDL.Text });
  const CanisterHttpResponsePayload = IDL.Record({
    'status' : IDL.Nat,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HttpHeader),
  });
  const TransformArgs = IDL.Record({
    'context' : IDL.Vec(IDL.Nat8),
    'response' : CanisterHttpResponsePayload,
  });
  const ExchangeRate = IDL.Service({
    'get_rates' : IDL.Func([TimeRange], [RatesWithInterval], []),
    'test_random_http_with_transform' : IDL.Func(
        [IDL.Text],
        [IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text })],
        [],
      ),
    'transform' : IDL.Func(
        [TransformArgs],
        [CanisterHttpResponsePayload],
        ['query'],
      ),
  });
  return ExchangeRate;
};
export const init = ({ IDL }) => { return []; };
