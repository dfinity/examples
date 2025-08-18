export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'current' : IDL.Func([], [IDL.Text], ['query']),
    'next' : IDL.Func([], [IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };
