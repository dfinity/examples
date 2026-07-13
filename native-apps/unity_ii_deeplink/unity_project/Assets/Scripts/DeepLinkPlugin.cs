using UnityEngine;
using EdjCase.ICP.Agent;
using EdjCase.ICP.Agent.Identities;
using EdjCase.ICP.Agent.Models;
using EdjCase.ICP.Candid.Models;
using System;
using System.Linq;
using System.Collections.Generic;

namespace IC.GameKit
{
    public class DeepLinkPlugin : MonoBehaviour
    {
        ICPAgent mIcpAgent = null;

        private void Awake()
        {
            // Register action for deep link activated.
            Application.deepLinkActivated += OnDeepLinkActivated;
        }

        public void Start()
        {
            mIcpAgent = gameObject.GetComponent<ICPAgent>();
            // Handle deep link if the app was cold-started from one.
            if (!string.IsNullOrEmpty(Application.absoluteURL))
                OnDeepLinkActivated(Application.absoluteURL);
        }

        public void SignIn()
        {
            var separator = mIcpAgent.iiBridgeUrl.Contains("?") ? "&" : "?";
            var target = mIcpAgent.iiBridgeUrl + separator + "sessionkey=" + ToHex(mIcpAgent.SessionIdentity.PublicKey.ToDerEncoding());
            Application.OpenURL(target);
        }

        public void OnDeepLinkActivated(string url)
        {
            if (string.IsNullOrEmpty(url))
                return;

            const string kDelegationParam = "delegation=";
            var indexOfDelegation = url.IndexOf(kDelegationParam);
            if (indexOfDelegation == -1)
            {
                Debug.LogError("Cannot find delegation");
                return;
            }

            var delegationString = Uri.UnescapeDataString(url.Substring(indexOfDelegation + kDelegationParam.Length));
            mIcpAgent.DelegationIdentity = ConvertJsonToDelegationIdentity(delegationString);
        }

        private static string ToHex(byte[] bytes) =>
            BitConverter.ToString(bytes).Replace("-", "").ToLower();

        private static byte[] FromHex(string hex)
        {
            var result = new byte[hex.Length / 2];
            for (int i = 0; i < result.Length; i++)
                result[i] = Convert.ToByte(hex.Substring(i * 2, 2), 16);
            return result;
        }

        internal DelegationIdentity ConvertJsonToDelegationIdentity(string jsonDelegation)
        {
            try
            {
                var delegationChainModel = JsonUtility.FromJson<DelegationChainModel>(jsonDelegation);
                if (delegationChainModel == null || delegationChainModel.delegations == null || delegationChainModel.delegations.Length == 0)
                {
                    Debug.LogError("[DeepLinkPlugin] Invalid delegation chain (null or empty).");
                    return null;
                }

                // Verify the last delegation targets this session's public key.
                // Prevents a delegation issued for a different session from being accepted.
                var lastPubKey = FromHex(delegationChainModel.delegations[delegationChainModel.delegations.Length - 1].delegation.pubkey);
                var sessionKey = mIcpAgent.SessionIdentity.PublicKey.ToDerEncoding();
                if (!lastPubKey.SequenceEqual(sessionKey))
                {
                    Debug.LogError("[DeepLinkPlugin] Session key mismatch — delegation was not issued for this session.");
                    return null;
                }

                // Initialize DelegationIdentity.
                var delegations = new List<SignedDelegation>();
                foreach (var signedDelegationModel in delegationChainModel.delegations)
                {
                    var pubKey = SubjectPublicKeyInfo.FromDerEncoding(FromHex(signedDelegationModel.delegation.pubkey));
                    var expiration = ICTimestamp.FromNanoSeconds(Convert.ToUInt64(signedDelegationModel.delegation.expiration, 16));
                    var delegation = new Delegation(pubKey, expiration);

                    var signature = FromHex(signedDelegationModel.signature);
                    var signedDelegation = new SignedDelegation(delegation, signature);
                    delegations.Add(signedDelegation);
                }

                var chainPublicKey = SubjectPublicKeyInfo.FromDerEncoding(FromHex(delegationChainModel.publicKey));
                var delegationChain = new DelegationChain(chainPublicKey, delegations);
                var delegationIdentity = new DelegationIdentity(mIcpAgent.SessionIdentity, delegationChain);

                Debug.Log("[DeepLinkPlugin] DelegationIdentity created successfully.");
                return delegationIdentity;
            }
            catch (Exception e)
            {
                Debug.LogError("[DeepLinkPlugin] Failed to parse delegation chain: " + e.Message);
                return null;
            }
        }
    }
}
