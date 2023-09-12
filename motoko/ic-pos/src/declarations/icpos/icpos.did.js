export const idlFactory = ({ IDL }) => {
  const Merchant = IDL.Record({
    email_address: IDL.Text,
    phone_notifications: IDL.Bool,
    name: IDL.Text,
    email_notifications: IDL.Bool,
    phone_number: IDL.Text,
  });
  const ResponseMerchant = IDL.Record({
    status: IDL.Nat16,
    data: IDL.Opt(Merchant),
    status_text: IDL.Text,
    error_text: IDL.Opt(IDL.Text),
  });
  const ResponseText = IDL.Record({
    status: IDL.Nat16,
    data: IDL.Opt(IDL.Text),
    status_text: IDL.Text,
    error_text: IDL.Opt(IDL.Text),
  });
  return IDL.Service({
    getMerchant: IDL.Func([], [ResponseMerchant], ["query"]),
    updateMerchant: IDL.Func([Merchant], [ResponseMerchant], []),
    setCourierApiKey: IDL.Func([IDL.Text], [ResponseText], []),
  });
};
export const init = ({ IDL }) => {
  return [];
};
