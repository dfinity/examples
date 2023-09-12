namespace IC.GameKit
{
    public class DelegationModel
    {
        public string expiration;
        public string pubkey;
    }

    public class SignedDelegationModel
    {
        public DelegationModel delegation;
        public string signature;
    }

    public class DelegationChainModel
    {
        public SignedDelegationModel[] delegations;
        public string publicKey;
    }
}
