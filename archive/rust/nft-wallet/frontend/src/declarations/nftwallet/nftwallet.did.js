export const idlFactory = ({ IDL }) => {
  const TokenIndex = IDL.Nat64;
  const Nft = IDL.Record({ 'canister' : IDL.Principal, 'index' : TokenIndex });
  const Error = IDL.Variant({
    'CanisterError' : IDL.Null,
    'Trap' : IDL.Record({ 'message' : IDL.Text }),
    'CannotNotify' : IDL.Null,
    'NoSuchToken' : IDL.Null,
    'Unauthorized' : IDL.Null,
    'NotOwner' : IDL.Null,
  });
  const ManageResult = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  return IDL.Service({
    'is_authorized' : IDL.Func([], [IDL.Bool], ['query']),
    'onDIP721Received' : IDL.Func(
        [IDL.Principal, IDL.Principal, TokenIndex, IDL.Vec(IDL.Nat8)],
        [],
        [],
      ),
    'owned_nfts' : IDL.Func([], [IDL.Vec(Nft)], ['query']),
    'register' : IDL.Func([Nft], [ManageResult], []),
    'set_authorized' : IDL.Func([IDL.Principal, IDL.Bool], [ManageResult], []),
    'tokenTransferNotification' : IDL.Func(
        [
          IDL.Text,
          IDL.Variant({ 'principal' : IDL.Principal, 'address' : IDL.Text }),
          IDL.Nat,
          IDL.Vec(IDL.Nat8),
        ],
        [IDL.Opt(IDL.Nat)],
        [],
      ),
    'transfer' : IDL.Func(
        [Nft, IDL.Principal, IDL.Opt(IDL.Bool)],
        [ManageResult],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return [IDL.Opt(IDL.Vec(IDL.Principal))]; };
