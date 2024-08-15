export const idlFactory = ({ IDL }) => {
  const SchnorrAlgorithm = IDL.Variant({
    'ed25519' : IDL.Null,
    'bip340secp256k1' : IDL.Null,
  });
  return IDL.Service({
    'for_test_only_change_management_canister_id' : IDL.Func(
        [IDL.Text],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text })],
        [],
      ),
    'public_key' : IDL.Func(
        [SchnorrAlgorithm],
        [
          IDL.Variant({
            'Ok' : IDL.Record({ 'public_key_hex' : IDL.Text }),
            'Err' : IDL.Text,
          }),
        ],
        [],
      ),
    'sign' : IDL.Func(
        [IDL.Text, SchnorrAlgorithm],
        [
          IDL.Variant({
            'Ok' : IDL.Record({ 'signature_hex' : IDL.Text }),
            'Err' : IDL.Text,
          }),
        ],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
