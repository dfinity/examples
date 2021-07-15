export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'current' : IDL.Func([], [IDL.Text], ['query']),
    'next' : IDL.Func([], [IDL.Text], []),
    'stableState' : IDL.Func([], [IDL.Text], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };