export const idlFactory = ({ IDL }) => {
  return IDL.Service({ 'whoami' : IDL.Func([], [IDL.Principal], ['query']) });
};
export const init = ({ IDL }) => { return []; };
