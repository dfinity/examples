export const idlFactory = ({ IDL }) => {
  const AccountIdentifier__1 = IDL.Variant({
    'principal' : IDL.Principal,
    'blob' : IDL.Vec(IDL.Nat8),
    'text' : IDL.Text,
  });
  const AccountIdentifierToBlobSuccess = IDL.Vec(IDL.Nat8);
  const AccountIdentifierToBlobErr = IDL.Record({
    'kind' : IDL.Variant({
      'InvalidAccountIdentifier' : IDL.Null,
      'Other' : IDL.Null,
    }),
    'message' : IDL.Opt(IDL.Text),
  });
  const AccountIdentifierToBlobResult = IDL.Variant({
    'ok' : AccountIdentifierToBlobSuccess,
    'err' : AccountIdentifierToBlobErr,
  });
  const Permissions = IDL.Record({
    'canGet' : IDL.Vec(IDL.Principal),
    'canVerify' : IDL.Vec(IDL.Principal),
  });
  const Token = IDL.Record({ 'symbol' : IDL.Text });
  const Details = IDL.Record({
    'meta' : IDL.Vec(IDL.Nat8),
    'description' : IDL.Text,
  });
  const CreateInvoiceArgs = IDL.Record({
    'permissions' : IDL.Opt(Permissions),
    'token' : Token,
    'details' : IDL.Opt(Details),
    'amount' : IDL.Nat,
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
  const Time = IDL.Int;
  const Invoice = IDL.Record({
    'id' : IDL.Nat,
    'permissions' : IDL.Opt(Permissions),
    'creator' : IDL.Principal,
    'destination' : AccountIdentifier,
    'token' : TokenVerbose,
    'paid' : IDL.Bool,
    'verifiedAtTime' : IDL.Opt(Time),
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
  const GetAccountIdentifierArgs = IDL.Record({
    'principal' : IDL.Principal,
    'token' : Token,
  });
  const GetAccountIdentifierSuccess = IDL.Record({
    'accountIdentifier' : AccountIdentifier,
  });
  const GetAccountIdentifierErr = IDL.Record({
    'kind' : IDL.Variant({ 'InvalidToken' : IDL.Null, 'Other' : IDL.Null }),
    'message' : IDL.Opt(IDL.Text),
  });
  const GetAccountIdentifierResult = IDL.Variant({
    'ok' : GetAccountIdentifierSuccess,
    'err' : GetAccountIdentifierErr,
  });
  const GetBalanceArgs = IDL.Record({ 'token' : Token });
  const GetBalanceSuccess = IDL.Record({ 'balance' : IDL.Nat });
  const GetBalanceErr = IDL.Record({
    'kind' : IDL.Variant({
      'NotFound' : IDL.Null,
      'InvalidToken' : IDL.Null,
      'Other' : IDL.Null,
    }),
    'message' : IDL.Opt(IDL.Text),
  });
  const GetBalanceResult = IDL.Variant({
    'ok' : GetBalanceSuccess,
    'err' : GetBalanceErr,
  });
  const GetInvoiceArgs = IDL.Record({ 'id' : IDL.Nat });
  const GetInvoiceSuccess = IDL.Record({ 'invoice' : Invoice });
  const GetInvoiceErr = IDL.Record({
    'kind' : IDL.Variant({
      'NotFound' : IDL.Null,
      'NotAuthorized' : IDL.Null,
      'InvalidInvoiceId' : IDL.Null,
      'Other' : IDL.Null,
    }),
    'message' : IDL.Opt(IDL.Text),
  });
  const GetInvoiceResult = IDL.Variant({
    'ok' : GetInvoiceSuccess,
    'err' : GetInvoiceErr,
  });
  const TransferArgs = IDL.Record({
    'destination' : AccountIdentifier,
    'token' : Token,
    'amount' : IDL.Nat,
  });
  const TransferSuccess = IDL.Record({ 'blockHeight' : IDL.Nat64 });
  const TransferError = IDL.Record({
    'kind' : IDL.Variant({
      'InvalidDestination' : IDL.Null,
      'BadFee' : IDL.Null,
      'InvalidToken' : IDL.Null,
      'Other' : IDL.Null,
      'InsufficientFunds' : IDL.Null,
    }),
    'message' : IDL.Opt(IDL.Text),
  });
  const TransferResult = IDL.Variant({
    'ok' : TransferSuccess,
    'err' : TransferError,
  });
  const VerifyInvoiceArgs = IDL.Record({ 'id' : IDL.Nat });
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
    'accountIdentifierToBlob' : IDL.Func(
        [AccountIdentifier__1],
        [AccountIdentifierToBlobResult],
        [],
      ),
    'create_invoice' : IDL.Func([CreateInvoiceArgs], [CreateInvoiceResult], []),
    'get_account_identifier' : IDL.Func(
        [GetAccountIdentifierArgs],
        [GetAccountIdentifierResult],
        ['query'],
      ),
    'get_balance' : IDL.Func([GetBalanceArgs], [GetBalanceResult], []),
    'get_invoice' : IDL.Func([GetInvoiceArgs], [GetInvoiceResult], ['query']),
    'transfer' : IDL.Func([TransferArgs], [TransferResult], []),
    'verify_invoice' : IDL.Func([VerifyInvoiceArgs], [VerifyInvoiceResult], []),
  });
};
export const init = ({ IDL }) => { return []; };
