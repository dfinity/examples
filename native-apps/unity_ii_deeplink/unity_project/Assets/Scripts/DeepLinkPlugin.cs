using UnityEngine;
using EdjCase.ICP.Agent;
using EdjCase.ICP.Agent.Identities;
using EdjCase.ICP.Agent.Models;
using EdjCase.ICP.Candid.Models;
using EdjCase.ICP.Candid.Utilities;
using Newtonsoft.Json;
using System;
using System.Web;
using System.Collections.Generic;

namespace IC.GameKit
{
    public class DeepLinkPlugin : MonoBehaviour
    {
        TestICPAgent mTestICPAgent = null;

        private void Awake()
        {
            // Register action for deep link activated.
            Application.deepLinkActivated += OnDeepLinkActivated;
        }

        public void Start()
        {
            mTestICPAgent = gameObject.GetComponent<TestICPAgent>();
        }

        public void OpenBrowser()
        {
            var target = mTestICPAgent.greetFrontend + "?sessionkey=" + ByteUtil.ToHexString(mTestICPAgent.TestIdentity.PublicKey.ToDerEncoding());
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

            var delegationString = HttpUtility.UrlDecode(url.Substring(indexOfDelegation + kDelegationParam.Length));
            mTestICPAgent.DelegationIdentity = ConvertJsonToDelegationIdentity(delegationString);
        }

        internal DelegationIdentity ConvertJsonToDelegationIdentity(string jsonDelegation)
        {
            var delegationChainModel = JsonConvert.DeserializeObject<DelegationChainModel>(jsonDelegation);
            if (delegationChainModel == null && delegationChainModel.delegations.Length == 0)
            {
                Debug.LogError("Invalid delegation chain.");
                return null;
            }

            // Initialize DelegationIdentity.
            var delegations = new List<SignedDelegation>();
            foreach (var signedDelegationModel in delegationChainModel.delegations)
            {
                var pubKey = SubjectPublicKeyInfo.FromDerEncoding(ByteUtil.FromHexString(signedDelegationModel.delegation.pubkey));
                var expiration = ICTimestamp.FromNanoSeconds(Convert.ToUInt64(signedDelegationModel.delegation.expiration, 16));
                var delegation = new Delegation(pubKey, expiration);

                var signature = ByteUtil.FromHexString(signedDelegationModel.signature);
                var signedDelegation = new SignedDelegation(delegation, signature);
                delegations.Add(signedDelegation);
            }

            var chainPublicKey = SubjectPublicKeyInfo.FromDerEncoding(ByteUtil.FromHexString(delegationChainModel.publicKey));
            var delegationChain = new DelegationChain(chainPublicKey, delegations);
            var delegationIdentity = new DelegationIdentity(mTestICPAgent.TestIdentity, delegationChain);

            return delegationIdentity;
        }
    }
}
