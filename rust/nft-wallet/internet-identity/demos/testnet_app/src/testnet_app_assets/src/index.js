import { Actor, HttpAgent } from '@dfinity/agent';
import { AuthClient } from "@dfinity/auth-client";
import { idlFactory as testnet_app_idl, canisterId as testnet_app_id } from '../../declarations/testnet_app';


// Autofills the <input> for the II Url to point to our testnet.
document.body.onload = () => {
  const testnetII = "https://rdmx6-jaaaa-aaaaa-aaadq-cai.identity.dfinity.network/"
  document.getElementById("iiUrl").value = testnetII;
};

document.getElementById("loginBtn").addEventListener("click", async () => {
  const iiUrl = document.getElementById("iiUrl").value;
  console.log("Authenticating with II", iiUrl);
  const authClient = await AuthClient.create();
  // This API is just weird
  await new Promise((resolve, reject) => {
    authClient.login({
      identityProvider: iiUrl,
      onSuccess: resolve,
      onError: reject
    })
  });
  const identity = authClient.getIdentity();

  console.log({identity});
  console.log(`canister ID: ${testnet_app_id}`);

  const agent = new HttpAgent({ identity });
  const testnet_app = Actor.createActor(testnet_app_idl, { agent, canisterId: testnet_app_id });
  const whoAmI = await testnet_app.whoami();
  console.log({whoAmI});

  document.getElementById("loginStatus").innerText = whoAmI.toText();

});
