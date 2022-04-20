import { Actor, HttpAgent } from "@dfinity/agent";
import { DelegationIdentity } from "@dfinity/identity";
import { AuthClient } from "@dfinity/auth-client";
import { Principal } from "@dfinity/principal";

const signInBtn = document.getElementById("signinBtn");
const signOutBtn = document.getElementById("signoutBtn");
const whoamiBtn = document.getElementById("whoamiBtn");
const hostUrlEl = document.getElementById("hostUrl");
const whoAmIResponseEl = document.getElementById("whoamiResponse");
const canisterIdEl = document.getElementById("canisterId");
const principalEl = document.getElementById("principal");
const delegationEl = document.getElementById("delegation");
const expirationEl = document.getElementById("expiration");
const iiUrlEl = document.getElementById("iiUrl");
const maxTimeToLiveEl = document.getElementById("maxTimeToLive");

let authClient;

const init = async () => {
  authClient = await AuthClient.create();

  const updateView = () => {
    const identity = authClient.getIdentity();

    principalEl.innerText = identity.getPrincipal();
    if (identity instanceof DelegationIdentity) {
      delegationEl.innerText = JSON.stringify(identity.getDelegation().toJSON(), undefined, 2);

      // cannot use Math.min, as we deal with bigint here
      const nextExpiration =
        identity.getDelegation().delegations
         .map(d => d.delegation.expiration)
         .reduce((current, next) => next < current ? next : current);
      expirationEl.innerText = nextExpiration - BigInt(Date.now()) * BigInt(1000_000);
    } else {
      delegationEl.innerText = "Current identity is not a DelegationIdentity";
      expirationEl.innerText = "N/A";
    }
  }

  updateView();

  signInBtn.onclick = async () => {
    if (BigInt(maxTimeToLiveEl.value) > BigInt(0)) {
      authClient.login({
        identityProvider: iiUrlEl.value,
        maxTimeToLive: BigInt(maxTimeToLive.value),
        onSuccess: updateView
      })
    } else {
      authClient.login({
        identityProvider: iiUrlEl.value,
        onSuccess: updateView
      });
    }
  };

  signOutBtn.onclick = async () => {
    authClient.logout();
    updateView();
  };
};

init();

whoamiBtn.addEventListener("click", async () => {
  const identity = await authClient.getIdentity();

  // We either have an Agent with an anonymous identity (not authenticated),
  // or already authenticated agent, or parsing the redirect from window.location.
  const idlFactory = ({ IDL }) =>
    IDL.Service({
      whoami: IDL.Func([], [IDL.Principal], ['query']),
    });

  const canisterId = Principal.fromText(canisterIdEl.value);

  const actor = Actor.createActor(idlFactory, {
    agent: new HttpAgent({
      host: hostUrlEl.value,
      identity,
    }),
    canisterId,
  });

  whoAmIResponseEl.innerText = "Loading...";

  // Similar to the sample project on dfx new:
  actor.whoami().then((principal) => {
    whoAmIResponseEl.innerText = principal.toText();
  }).catch(err => {
    console.error("Failed to fetch whoami", err)
  });
});
