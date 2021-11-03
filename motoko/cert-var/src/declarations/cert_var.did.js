export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'get' : IDL.Func(
        [],
        [
          IDL.Record({
            'certificate' : IDL.Opt(IDL.Vec(IDL.Nat8)),
            'value' : IDL.Nat32,
          }),
        ],
        ['query'],
      ),
    'inc' : IDL.Func([], [IDL.Nat32], []),
    'set' : IDL.Func([IDL.Nat32], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
