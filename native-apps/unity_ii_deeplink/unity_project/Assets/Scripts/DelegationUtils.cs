using System;

namespace IC.GameKit
{
    [Serializable]
    public class DelegationModel
    {
        public string expiration;
        public string pubkey;
    }

    [Serializable]
    public class SignedDelegationModel
    {
        public DelegationModel delegation;
        public string signature;
    }

    [Serializable]
    public class DelegationChainModel
    {
        public SignedDelegationModel[] delegations;
        public string publicKey;
    }
}
