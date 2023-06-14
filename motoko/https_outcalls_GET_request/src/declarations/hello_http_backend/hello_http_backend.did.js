export const idlFactory = ({ IDL }) => {
  return IDL.Service({ 'get_icp_usd_exchange' : IDL.Func([], [IDL.Text], []) });
};
export const init = ({ IDL }) => { return []; };
