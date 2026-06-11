import Curves "mo:bitcoin/ec/Curves";
import BitcoinTypes "mo:bitcoin/bitcoin/Types";
import IC "mo:ic/Types";

module Types {
    public type SendRequest = {
        destination_address : Text;
        amount_in_satoshi : Satoshi;
    };

    public type Satoshi = BitcoinTypes.Satoshi;

    /// millisatoshi per byte — matches the IC management canister's fee percentile unit
    public type MillisatoshiPerByte = IC.MillisatoshiPerByte;

    public type BitcoinAddress = IC.BitcoinAddress;


    public let CURVE = Curves.secp256k1;

    /// Network type using lowercase variants, matching the Bitcoin canister Candid.
    /// mo:bitcoin uses PascalCase internally — a conversion is applied when calling
    /// mo:bitcoin address-generation functions.
    public type Network = {
        #mainnet;
        #testnet;
        #regtest;
    };


    public type OutPoint = BitcoinTypes.OutPoint;
    public type Utxo = BitcoinTypes.Utxo;

    /// UTXO filter — matches the IC management canister's inline filter type exactly.
    public type UtxosFilter = {
        #min_confirmations : Nat32;
        #page : Blob;
    };

    /// Re-exported from mo:ic — uses Blob for tip_block_hash and next_page.
    public type GetUtxosResponse = IC.BitcoinGetUtxosResult;

    public type EcdsaSignFunction = (Text, [Blob], Blob) -> async Blob;

    public type SchnorrSignFunction = (Text, [Blob], Blob, ?IC.SchnorrAux) -> async Blob;

    public type P2trDerivationPaths = {
        key_path_derivation_path : [[Nat8]];
        script_path_derivation_path : [[Nat8]];
    };
};
