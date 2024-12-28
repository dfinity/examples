export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'decrement' : IDL.Func([], [IDL.Nat], []),
    'getCount' : IDL.Func([], [IDL.Nat], ['query']),
    'increment' : IDL.Func([], [IDL.Nat], []),
    'reset' : IDL.Func([], [IDL.Nat], []),
  });
};
export const init = ({ IDL }) => { return []; };
