export const idlFactory = ({ IDL }) => {
  const Permissions = IDL.Record({
    'canGet' : IDL.Vec(IDL.Principal),
    'canVerify' : IDL.Vec(IDL.Principal),
  });
  const AccountIdentifier = IDL.Variant({
    'principal' : IDL.Principal,
    'blob' : IDL.Vec(IDL.Nat8),
    'text' : IDL.Text,
  });
  const TokenVerbose = IDL.Record({
    'decimals' : IDL.Int,
    'meta' : IDL.Opt(IDL.Record({ 'Issuer' : IDL.Text })),
    'symbol' : IDL.Text,
  });
  const Details = IDL.Record({
    'meta' : IDL.Vec(IDL.Nat8),
    'description' : IDL.Text,
  });
  const Invoice = IDL.Record({
    'id' : IDL.Nat,
    'permissions' : IDL.Opt(Permissions),
    'creator' : IDL.Principal,
    'destination' : AccountIdentifier,
    'token' : TokenVerbose,
    'paid' : IDL.Bool,
    'verifiedAtTime' : IDL.Opt(IDL.Int),
    'amountPaid' : IDL.Nat,
    'details' : IDL.Opt(Details),
    'amount' : IDL.Nat,
  });
  const CreateInvoiceSuccess = IDL.Record({ 'invoice' : Invoice });
  const CreateInvoiceErr = IDL.Record({
    'kind' : IDL.Variant({
      'InvalidDetails' : IDL.Null,
      'InvalidAmount' : IDL.Null,
      'InvalidDestination' : IDL.Null,
      'MaxInvoicesReached' : IDL.Null,
      'BadSize' : IDL.Null,
      'InvalidToken' : IDL.Null,
      'Other' : IDL.Null,
    }),
    'message' : IDL.Opt(IDL.Text),
  });
  const CreateInvoiceResult = IDL.Variant({
    'ok' : CreateInvoiceSuccess,
    'err' : CreateInvoiceErr,
  });
  const VerifyInvoiceSuccess = IDL.Variant({
    'Paid' : IDL.Record({ 'invoice' : Invoice }),
    'AlreadyVerified' : IDL.Record({ 'invoice' : Invoice }),
  });
  const VerifyInvoiceErr = IDL.Record({
    'kind' : IDL.Variant({
      'InvalidAccount' : IDL.Null,
      'TransferError' : IDL.Null,
      'NotFound' : IDL.Null,
      'NotAuthorized' : IDL.Null,
      'InvalidToken' : IDL.Null,
      'InvalidInvoiceId' : IDL.Null,
      'Other' : IDL.Null,
      'NotYetPaid' : IDL.Null,
      'Expired' : IDL.Null,
    }),
    'message' : IDL.Opt(IDL.Text),
  });
  const VerifyInvoiceResult = IDL.Variant({
    'ok' : VerifyInvoiceSuccess,
    'err' : VerifyInvoiceErr,
  });
  return IDL.Service({
    'check_license_status' : IDL.Func([], [IDL.Bool], ['query']),
    'create_invoice' : IDL.Func([], [CreateInvoiceResult], []),
    'get_invoice' : IDL.Func([IDL.Nat], [IDL.Opt(Invoice)], ['query']),
    'reset_license' : IDL.Func([], [], []),
    'verify_invoice' : IDL.Func([IDL.Nat], [VerifyInvoiceResult], []),
  });
};
export const init = ({ IDL }) => { return []; };
