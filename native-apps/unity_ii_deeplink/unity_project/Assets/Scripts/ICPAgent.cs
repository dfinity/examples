using System;
using System.Threading.Tasks;
using UnityEngine;
using UnityEngine.UI;
using EdjCase.ICP.Agent.Agents;
using EdjCase.ICP.Agent.Identities;
using EdjCase.ICP.Candid.Models;

namespace IC.GameKit
{
    public class ICPAgent : MonoBehaviour
    {
        // Set these in the Unity Inspector before building.
        // After running `icp deploy`, retrieve the values with:
        //   icp canister status ii-bridge -i  → paste the resulting URL as iiBridgeUrl
        //   icp canister status backend -i   → paste the canister ID as greetBackendCanister
        // For local emulator testing via adb reverse, use the subdomain URL:
        //   iiBridgeUrl  → http://ii-bridge.local.localhost:8000
        //   icGateway      → http://localhost:8000
        public string iiBridgeUrl = "";
        public string greetBackendCanister = "";
        // IC HTTP gateway — mainnet default; change for local/emulator testing.
        public string icGateway = "https://icp-api.io";

        Text mMyPrincipalText = null;
        Button mSignInButton = null;
        Button mGreetButton = null;
        Ed25519Identity mEd25519Identity = null;
        DelegationIdentity mDelegationIdentity = null;

        public Ed25519Identity SessionIdentity { get { return mEd25519Identity; } }

        internal DelegationIdentity DelegationIdentity
        {
            get { return mDelegationIdentity; }
            set
            {
                mDelegationIdentity = value;

                if (mDelegationIdentity != null)
                {
                    if (mGreetButton != null) mGreetButton.interactable = true;
                    if (mSignInButton != null) mSignInButton.gameObject.SetActive(false);
                    if (mMyPrincipalText != null) mMyPrincipalText.text = "Signed in ✓";
                }
                else
                {
                    if (mGreetButton != null) mGreetButton.interactable = false;
                    if (mSignInButton != null) mSignInButton.gameObject.SetActive(true);
                }
            }
        }

        async void Start()
        {
            var go = GameObject.Find("My Principal");
            mMyPrincipalText = go?.GetComponent<Text>();

            var signInGo = GameObject.Find("Button_Browser");
            mSignInButton = signInGo?.GetComponent<Button>();

            var buttonGo = GameObject.Find("Button_Greet");
            mGreetButton = buttonGo?.GetComponent<Button>();

            // Disable sign-in while the session key is being generated so the
            // button cannot be tapped before SessionIdentity is ready.
            if (mSignInButton != null) mSignInButton.interactable = false;

            try
            {
                // Generate the Ed25519 key on a background thread — crypto key
                // generation can block for several seconds on some devices/emulators.
                mEd25519Identity = await Task.Run(() => Ed25519Identity.Generate());
                if (mSignInButton != null) mSignInButton.interactable = true;
            }
            catch (Exception e)
            {
                Debug.LogError("[ICPAgent] Failed to generate session key: " + e.Message);
                if (mMyPrincipalText != null)
                    mMyPrincipalText.text = "Failed to initialise — restart the app.";
            }
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

            if (string.IsNullOrEmpty(greetBackendCanister))
            {
                Debug.LogError("[ICPAgent] greetBackendCanister is not set — enter the canister ID in the Inspector.");
                if (mMyPrincipalText != null)
                    mMyPrincipalText.text = "Canister ID not configured — set it in the Inspector.";
                return;
            }

            if (mGreetButton != null) mGreetButton.interactable = false;
            try
            {
                var agent = new HttpAgent(DelegationIdentity, new Uri(icGateway));
                var canisterId = Principal.FromText(greetBackendCanister);
                var client = new GreetingClient.GreetingClient(agent, canisterId);
                var content = await client.Greet();

                if (mMyPrincipalText != null)
                    mMyPrincipalText.text = content;
                if (mGreetButton != null) mGreetButton.interactable = true;
            }
            catch (Exception e)
            {
                Debug.LogError("[ICPAgent] Greet failed: " + e.Message);
                if (mMyPrincipalText != null)
                    mMyPrincipalText.text = "Greet failed: " + e.Message;
                DelegationIdentity = null;
            }
        }
    }
}
