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

    public type P2WpkhAddress = Text;

    public let CURVE = Curves.secp256k1;

    /// Lowercase variant Network type including #regtest for local development.
    /// Note: mo:bitcoin uses PascalCase (#Mainnet etc.) — see
    /// https://github.com/caffeinelabs/motoko-bitcoin/issues/22
    public type Network = {
        #mainnet;
        #testnet;
        #regtest;
    };

    /// Bridge type for mo:bitcoin address-generation functions (which use PascalCase).
    /// Can be removed once mo:bitcoin#22 is fixed.
    public type NetworkCamelCase = BitcoinTypes.Network;

    public func network_to_network_camel_case(network : Network) : NetworkCamelCase {
        switch (network) {
            case (#regtest) #Regtest;
            case (#testnet) #Testnet;
            case (#mainnet) #Mainnet;
        };
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
