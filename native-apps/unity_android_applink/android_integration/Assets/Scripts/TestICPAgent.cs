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
using System.IO;
using Newtonsoft.Json;
using System.Web;

namespace IC.GameKit
{
    public class TestICPAgent : MonoBehaviour
    {
        public string greetBackendCanister = "72rj2-biaaa-aaaan-qdatq-cai";
        Text mMyPrincipalText = null;
        Button mGreetButton = null;
        Ed25519Identity mEd25519Identity = null;
        DelegationChainModel mDelegation = null;

        public Ed25519Identity TestIdentity { get { return mEd25519Identity; } }

        // Start is called before the first frame update
        void Start()
        {
            var go = GameObject.Find("My Princinpal");
            mMyPrincipalText = go?.GetComponent<Text>();

            var buttonGo = GameObject.Find("Button_Greet");
            mGreetButton = buttonGo?.GetComponent<Button>();

            mEd25519Identity = Ed25519Identity.Create();
        }

        // Update is called once per frame
        void Update()
        {
        }

        public void OnMessageSent(string delegationPath)
        {
            if (string.IsNullOrEmpty(delegationPath) || !File.Exists(delegationPath))
                return;

            //Debug.Log("Identity path '" + identityPath + "' exists.");

            var parameters = File.ReadAllText(delegationPath);
            //Debug.Log("Params length is: " + parameters.Length);

            const string kDelegationParam = "delegation=";
            var indexOfDelegation = parameters.IndexOf(kDelegationParam);
            if (indexOfDelegation == -1)
            {
                Debug.LogError("Cannot find delegation");
                return;
            }

            var delegationString = HttpUtility.UrlDecode(parameters.Substring(indexOfDelegation + kDelegationParam.Length));
            mDelegation = JsonConvert.DeserializeObject<DelegationChainModel>(delegationString);

            if (mDelegation != null && mGreetButton != null)
            {
                mGreetButton.interactable = true;
            }
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
            var chainPublicKey = DerEncodedPublicKey.FromDer(ByteUtil.FromHexString(delegationChainModel.publicKey));
            var delegations = new List<SignedDelegation>();
            foreach (var signedDelegationModel in delegationChainModel.delegations)
            {
                var pubKey = DerEncodedPublicKey.FromDer(ByteUtil.FromHexString(signedDelegationModel.delegation.pubkey));
                var expiration = ICTimestamp.FromNanoSeconds(Convert.ToUInt64(signedDelegationModel.delegation.expiration, 16));
                var delegation = new Delegation(pubKey.Value, expiration);

                var signature = ByteUtil.FromHexString(signedDelegationModel.signature);
                var signedDelegation = new SignedDelegation(delegation, signature);
                delegations.Add(signedDelegation);
            }
            var delegationChain = new DelegationChain(chainPublicKey, delegations);
            var delegationIdentity = new DelegationIdentity(TestIdentity, delegationChain);

            // Initialize HttpAgent.
            var agent = new HttpAgent(delegationIdentity);

            Principal canisterId = Principal.FromText(greetBackendCanister);

            // Intialize Client and make the call.
            var client = new GreetingClient.GreetingClient(agent, canisterId);
            var content = await client.Greet();

            if (mMyPrincipalText != null)
                mMyPrincipalText.text = content;
        }
    }
}
