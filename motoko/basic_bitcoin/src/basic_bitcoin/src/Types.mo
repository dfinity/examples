import Curves "../../../motoko-bitcoin/src/ec/Curves";

module Types {
    public type SendRequest = {
        destination_address : Text;
        amount_in_satoshi : Satoshi;
    };

    public type ECDSAPublicKeyReply = {
        public_key : Blob;
        chain_code : Blob;
    };

    public type EcdsaKeyId = {
        curve : EcdsaCurve;
        name : Text;
    };

    public type EcdsaCurve = {
        #secp256k1;
    };

    public type SignWithECDSAReply = {
        signature : Blob;
    };

    public type ECDSAPublicKey = {
        canister_id : ?Principal;
        derivation_path : [Blob];
        key_id : EcdsaKeyId;
    };

    public type SignWithECDSA = {
        message_hash : Blob;
        derivation_path : [Blob];
        key_id : EcdsaKeyId;
    };

    public type Satoshi = Nat64;
    public type MillisatoshiPerByte = Nat64;
    public type Cycles = Nat;
    public type BitcoinAddress = Text;
    public type BlockHash = [Nat8];
    public type Page = [Nat8];

    public let CURVE = Curves.secp256k1;
    
    /// The type of Bitcoin network the dapp will be interacting with.
    public type Network = {
        #Mainnet;
        #Testnet;
        #Regtest;
    };

    /// A reference to a transaction output.
    public type OutPoint = {
        txid : Blob;
        vout : Nat32;
    };

    /// An unspent transaction output.
    public type Utxo = {
        outpoint : OutPoint;
        value : Satoshi;
        height : Nat32;
    };

    /// A request for getting the balance for a given address.
    public type GetBalanceRequest = {
        address : BitcoinAddress;
        network : Network;
        min_confirmations : ?Nat32;
    };

    /// A filter used when requesting UTXOs.
    public type UtxosFilter = {
        #MinConfirmations : Nat32;
        #Page : Page;
    };

    /// A request for getting the UTXOs for a given address.
    public type GetUtxosRequest = {
        address : BitcoinAddress;
        network : Network;
        filter : ?UtxosFilter;
    };

    /// The response returned for a request to get the UTXOs of a given address.
    public type GetUtxosResponse = {
        utxos : [Utxo];
        tip_block_hash : BlockHash;
        tip_height : Nat32;
        next_page : ?Page;
    };

    /// A request for getting the current fee percentiles.
    public type GetCurrentFeePercentilesRequest = {
        network : Network;
    };

    public type SendTransactionRequest = {
        transaction : [Nat8];
        network : Network;
    };
}