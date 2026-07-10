using System;
using UnityEngine;
using UnityEngine.UI;
using EdjCase.ICP.Agent.Agents;
using EdjCase.ICP.Agent.Identities;
using EdjCase.ICP.Candid.Models;

namespace IC.GameKit
{
    public class TestICPAgent : MonoBehaviour
    {
        // Set these in the Unity Inspector before building.
        // After running `icp deploy`, retrieve the values with:
        //   icp canister id frontend   → paste the resulting URL as greetFrontend
        //   icp canister id backend    → paste the canister ID as greetBackendCanister
        // For local testing on a device connected to the same WiFi network, use
        // the host machine's LAN IP, e.g. "http://192.168.1.42:8000/?canisterId=<id>"
        // and change icGateway to "http://192.168.1.42:8000".
        public string greetFrontend = "";
        public string greetBackendCanister = "";
        // IC HTTP gateway — mainnet default; change to local IP for device testing.
        public string icGateway = "https://ic0.app";

        Text mMyPrincipalText = null;
        Button mGreetButton = null;
        Ed25519Identity mEd25519Identity = null;
        DelegationIdentity mDelegationIdentity = null;

        public Ed25519Identity TestIdentity { get { return mEd25519Identity; } }

        internal DelegationIdentity DelegationIdentity
        {
            get { return mDelegationIdentity; } 
            set 
            {
                mDelegationIdentity = value;
                
                if (mDelegationIdentity != null && mGreetButton != null)
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
            CallCanisterGreet();
        }

        private async void CallCanisterGreet()
        {
            if (DelegationIdentity == null)
                return;

            // Initialize HttpAgent.
            var agent = new HttpAgent(DelegationIdentity, new Uri(icGateway));

            var canisterId = Principal.FromText(greetBackendCanister);

            // Initialize the client and make the call.
            var client = new GreetingClient.GreetingClient(agent, canisterId);
            var content = await client.Greet();

            if (mMyPrincipalText != null)
                mMyPrincipalText.text = content;
        }
    }
}
