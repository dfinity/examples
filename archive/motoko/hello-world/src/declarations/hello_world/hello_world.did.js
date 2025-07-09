export const idlFactory = ({ IDL }) => {
  return IDL.Service({ 'main' : IDL.Func([], [], ['oneway']) });
};
export const init = ({ IDL }) => { return []; };
