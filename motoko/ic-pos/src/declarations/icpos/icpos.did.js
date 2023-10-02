export const idlFactory = ({ IDL }) => {
  const Merchant = IDL.Record({
    'email_address' : IDL.Text,
    'phone_notifications' : IDL.Bool,
    'name' : IDL.Text,
    'email_notifications' : IDL.Bool,
    'phone_number' : IDL.Text,
  });
  const Response = IDL.Record({
    'status' : IDL.Nat16,
    'data' : IDL.Opt(Merchant),
    'status_text' : IDL.Text,
    'error_text' : IDL.Opt(IDL.Text),
  });
  const Response_1 = IDL.Record({
    'status' : IDL.Nat16,
    'data' : IDL.Opt(IDL.Text),
    'status_text' : IDL.Text,
    'error_text' : IDL.Opt(IDL.Text),
  });
  const Main = IDL.Service({
    'getLogs' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'getMerchant' : IDL.Func([], [Response], ['query']),
    'setCourierApiKey' : IDL.Func([IDL.Text], [Response_1], []),
    'setLedgerId' : IDL.Func([IDL.Text], [Response_1], []),
    'updateMerchant' : IDL.Func([Merchant], [Response], []),
  });
  return Main;
};
export const init = ({ IDL }) => { return [IDL.Nat]; };
