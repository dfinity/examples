import "./style.css";
import { idlFactory } from "./declarations/basic_bls_signing/backend.did";
import { Principal } from "@icp-sdk/core/principal";
import { AuthClient } from "@icp-sdk/auth/client";
import { Actor, HttpAgent, type ActorSubclass } from "@icp-sdk/core/agent";
import { _SERVICE } from "./declarations/basic_bls_signing/backend.did";
import { DerivedPublicKey, verifyBlsSignature } from "@icp-sdk/vetkeys";
import type { Signature } from "./declarations/basic_bls_signing/backend.did";
import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";

const canisterEnv = safeGetCanisterEnv<{
  "PUBLIC_CANISTER_ID:basic_bls_signing": string;
}>();

let myPrincipal: Principal | undefined = undefined;
let authClient: AuthClient | undefined;
let basicBlsSigningActor: ActorSubclass<_SERVICE> | undefined;
// let canisterPublicKey: DerivedPublicKey | undefined;
let myVerificationKey: DerivedPublicKey | undefined;

async function getBasicBlsSigningActor(): Promise<ActorSubclass<_SERVICE>> {
  if (basicBlsSigningActor) return basicBlsSigningActor;
  const canisterId = canisterEnv?.["PUBLIC_CANISTER_ID:basic_bls_signing"];
  if (!canisterId) {
    throw Error("Canister ID for basic_bls_signing is not set");
  }
  if (!authClient) {
    throw Error("Auth client is not initialized");
  }
  const agent = await HttpAgent.create({
    identity: await authClient.getIdentity(),
    host: window.location.origin,
    ...(canisterEnv?.IC_ROOT_KEY ? { rootKey: canisterEnv.IC_ROOT_KEY } : {}),
  });
  basicBlsSigningActor = Actor.createActor(idlFactory, { agent, canisterId });
  return basicBlsSigningActor;
}

export async function login(client: AuthClient): Promise<void> {
  try {
    const identity = await client.signIn({
      maxTimeToLive: BigInt(1800) * BigInt(1_000_000_000),
    });
    myPrincipal = identity.getPrincipal();
    updateUI(true);
  } catch (error: unknown) {
    alert("Authentication failed: " + error);
  }
}

export function logout() {
  void authClient?.signOut();
  myPrincipal = undefined;
  myVerificationKey = undefined;
  basicBlsSigningActor = undefined;
  updateUI(false);
  document.getElementById("signaturesList")!.classList.toggle("hidden", true);
}

async function initAuth() {
  const isLocalEnv =
    window.location.hostname === "localhost" ||
    window.location.hostname.endsWith(".localhost");
  authClient = new AuthClient({
    identityProvider: isLocalEnv
      ? "http://id.ai.localhost:8000/#authorize"
      : undefined,
  });
  const isAuthenticated = authClient.isAuthenticated();

  if (isAuthenticated) {
    myPrincipal = (await authClient.getIdentity()).getPrincipal();
    updateUI(true);
  } else {
    updateUI(false);
  }
}

function updateUI(isAuthenticated: boolean) {
  const loginButton = document.getElementById("loginButton")!;
  const principalDisplay = document.getElementById("principalDisplay")!;
  const logoutButton = document.getElementById("logoutButton")!;
  const signingActions = document.getElementById("signingActions")!;
  const customSignatureForm = document.getElementById("customSignatureForm")!;
  const signaturesList = document.getElementById("signaturesList")!;

  loginButton.classList.toggle("hidden", isAuthenticated);
  principalDisplay.classList.toggle("hidden", !isAuthenticated);
  logoutButton.classList.toggle("hidden", !isAuthenticated);
  signingActions.classList.toggle("hidden", !isAuthenticated);
  customSignatureForm.classList.toggle("hidden", true);
  signaturesList.classList.toggle("hidden", true);

  if (isAuthenticated && myPrincipal) {
    principalDisplay.textContent = `Principal: ${myPrincipal.toString()}`;
  }
}

function handleLogin() {
  if (!authClient) {
    alert("Auth client not initialized");
    return;
  }
  void login(authClient);
}

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
    <h1>Basic BLS Signing using VetKeys</h1>
    <div class="principal-container">
      <div id="principalDisplay" class="principal-display"></div>
      <button id="logoutButton">Logout</button>
    </div>
    <div class="login-container">
      <button id="loginButton">Login</button>
    </div>
    <div id="signingActions" class="buttons">
      <button id="signMessageButton">Sign Message</button>
      <button id="listSignaturesButton">List Signatures</button>
      <button id="customSignatureButton">Verify Custom Signature</button>
    </div>
    <div id="customSignatureForm">
      <h3>Verify Custom Signature</h3>
      <form id="submitSignatureForm">
        <div>
          <label for="message">Message</label>
          <input type="text" id="message" required>
        </div>
        <div>
          <label for="signature">Signature (hex)</label>
          <input type="text" id="signature" required>
        </div>
        <div>
          <label for="pubkey">Public key (hex)</label>
          <input type="text" id="pubkey" required>
        </div>
        <button type="submit">Submit</button>
      </form>
    </div>
    <div id="signaturesList">
      <h3>My Signatures</h3>
      <div id="signatures"></div>
    </div>
  </div>
`;

// Add event listeners
document.getElementById("loginButton")!.addEventListener("click", handleLogin);
document.getElementById("logoutButton")!.addEventListener("click", logout);
document.getElementById("signMessageButton")!.addEventListener("click", () => {
  void (async () => {
    const message = prompt("Enter message to sign:");
    if (message) {
      try {
        await (await getBasicBlsSigningActor()).sign_message(message);
        alert("Created and stored signature successfully.");
      } catch (error) {
        alert(`Error: ${error as Error}`);
      }
    }
  })();
});

document
  .getElementById("customSignatureButton")!
  .addEventListener("click", () => {
    document
      .getElementById("customSignatureForm")!
      .classList.toggle("hidden", false);
    document.getElementById("signaturesList")!.classList.toggle("hidden", true);
  });

document
  .getElementById("listSignaturesButton")!
  .addEventListener("click", () => {
    void listSignatures();
  });

document
  .getElementById("submitSignatureForm")!
  .addEventListener("submit", (e) => {
    e.preventDefault();
    const message = (document.getElementById("message") as HTMLInputElement)
      .value;
    const signatureHex = (
      document.getElementById("signature") as HTMLInputElement
    ).value;
    const pubkeyHex = (document.getElementById("pubkey") as HTMLInputElement)
      .value;
    const messageBytes = new TextEncoder().encode(message);

    try {
      const signatureBytes = new Uint8Array(
        signatureHex.match(/.{1,2}/g)!.map((byte) => parseInt(byte, 16)),
      );
      const pubkeyBytes = new Uint8Array(
        pubkeyHex.match(/.{1,2}/g)!.map((byte) => parseInt(byte, 16)),
      );

      const verificationKey = DerivedPublicKey.deserialize(pubkeyBytes);

      const result = verifyBlsSignature(
        verificationKey,
        messageBytes,
        signatureBytes,
      );
      alert(`Verification result: ${result ? "Valid" : "INVALID"}`);
    } catch {
      alert("Verification failed.");
    }
  });

async function listSignatures() {
  const actor = await getBasicBlsSigningActor();
  const signatures: Array<Signature> = await actor.get_my_signatures();
  const signaturesDiv = document.getElementById("signatures")!;
  signaturesDiv.innerHTML = "";

  if (signatures.length === 0) {
    signaturesDiv.innerHTML = `
        <div class="no-signatures">
          <p>No signatures have been published yet.</p>
        </div>
      `;
  } else {
    if (!myVerificationKey) {
      const myVerificationKeyRaw = await actor.get_my_verification_key();
      myVerificationKey = DerivedPublicKey.deserialize(
        Uint8Array.from(myVerificationKeyRaw),
      );
    }
    const myVerificationKeyHex = Array.from(myVerificationKey.publicKeyBytes())
      .map((b) => b.toString(16).padStart(2, "0"))
      .join("");

    for (const signatureData of signatures.slice().reverse()) {
      const signatureHex = Array.from(signatureData.signature)
        .map((b) => b.toString(16).padStart(2, "0"))
        .join("");

      // Convert nanoseconds to milliseconds and create a Date object
      const timestamp = new Date(Number(signatureData.timestamp) / 1_000_000);
      const formattedDate = timestamp.toLocaleString();

      const signatureElement = document.createElement("div");
      signatureElement.className = "signature";

      const isValid = verifyBlsSignature(
        myVerificationKey,
        new TextEncoder().encode(signatureData.message),
        Uint8Array.from(signatureData.signature),
      );

      signatureElement.innerHTML = `
        <h5>Signed message: ${signatureData.message}</h5>
        <p class="signature-hex">Signature: ${signatureHex}</p>
        <p class="verification-key-hex">Public key: ${myVerificationKeyHex}</p>
        <p class="verification-status ${isValid ? "valid" : "invalid"}">Verification: ${isValid ? "Valid" : "Invalid"}</p>
        <p class="timestamp">Added: ${formattedDate}</p>
          `;

      signaturesDiv.appendChild(signatureElement);
    }
  }

  document.getElementById("signaturesList")!.classList.toggle("hidden", false);
  document
    .getElementById("customSignatureForm")!
    .classList.toggle("hidden", true);
}

// Initialize auth
void initAuth();
