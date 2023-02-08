export const idlFactory = ({ IDL }) => {
  const Timestamp = IDL.Nat64;
  const TimeRange = IDL.Record({ 'end' : Timestamp, 'start' : Timestamp });
  const Rate = IDL.Float32;
  const RatesMap = IDL.Vec(IDL.Tuple(Timestamp, Rate));
  const RatesWithInterval = IDL.Record({
    'interval' : IDL.Nat64,
    'rates' : RatesMap,
  });
  return IDL.Service({
    'get_rates' : IDL.Func([TimeRange], [RatesWithInterval], []),
  });
};
export const init = ({ IDL }) => { return []; };
