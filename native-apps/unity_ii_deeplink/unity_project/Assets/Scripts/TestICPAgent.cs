using UnityEngine;
using UnityEngine.UI;
using EdjCase.ICP.Agent;
using EdjCase.ICP.Agent.Agents;
using EdjCase.ICP.Agent.Identities;
using EdjCase.ICP.Candid.Models;
using EdjCase.ICP.Candid.Utilities;
using System.Collections.Generic;
using EdjCase.ICP.Agent.Models;
using System;

namespace IC.GameKit
{
    public class TestICPAgent : MonoBehaviour
    {
        public string greetFrontend = "https://qsgof-4qaaa-aaaan-qekqq-cai.icp0.io/";
        public string greetBackendCanister = "qvhir-riaaa-aaaan-qekqa-cai";

        Text mMyPrincipalText = null;
        Button mGreetButton = null;
        Ed25519Identity mEd25519Identity = null;
        DelegationChainModel mDelegation = null;

        public Ed25519Identity TestIdentity { get { return mEd25519Identity; } }

        internal DelegationChainModel Delegation {
            get { return mDelegation; } 
            set 
            {
                mDelegation = value;
                
                if (mDelegation != null && mGreetButton != null)
                {
                    mGreetButton.interactable = true;
                }
            }
        }

        // Start is called before the first frame update
        void Start()
        {
            var go = GameObject.Find("My Princinpal");
            mMyPrincipalText = go?.GetComponent<Text>();

            var buttonGo = GameObject.Find("Button_Greet");
            mGreetButton = buttonGo?.GetComponent<Button>();

            mEd25519Identity = Ed25519Identity.Generate();
        }

        // Update is called once per frame
        void Update()
        {
        }

        public void Greet()
        {
            if (mDelegation == null)
                return;

            CallCanister(mDelegation);
        }

        private async void CallCanister(DelegationChainModel delegationChainModel)
        {
            Debug.Assert(delegationChainModel != null && delegationChainModel.delegations.Length >= 1);

            // Initialize DelegationIdentity.
            var chainPublicKey = SubjectPublicKeyInfo.FromDerEncoding(ByteUtil.FromHexString(delegationChainModel.publicKey));
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
            var delegationChain = new DelegationChain(chainPublicKey, delegations);
            var delegationIdentity = new DelegationIdentity(TestIdentity, delegationChain);

            // Initialize HttpAgent.
            var agent = new HttpAgent(delegationIdentity);

            var canisterId = Principal.FromText(greetBackendCanister);

            // Intialize the client and make the call.
            var client = new GreetingClient.GreetingClient(agent, canisterId);
            var content = await client.Greet();

            if (mMyPrincipalText != null)
                mMyPrincipalText.text = content;
        }
    }
}
