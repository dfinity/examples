export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'get' : IDL.Func([], [IDL.Nat], []),
    'increment' : IDL.Func([], [IDL.Nat], []),
    'set' : IDL.Func([IDL.Nat], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
