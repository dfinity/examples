import "./style.css";
import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";
import { createActor } from "./bindings/backend";
import { AuthClient, LocalStorage } from "@icp-sdk/auth/client";
import { HttpAgent } from "@icp-sdk/core/agent";
import { Principal } from "@icp-sdk/core/principal";
import {
  TransportSecretKey,
  EncryptedVetKey,
  DerivedPublicKey,
  IbeCiphertext,
  IbeIdentity,
  IbeSeed,
} from "@icp-sdk/vetkeys";

const canisterEnv = safeGetCanisterEnv();

let backendActor;
let myPrincipal;
let authClient;
let symKeyMaterial;
let ibeDerivedPublicKey;

async function getActor() {
  if (backendActor) return backendActor;
  const canisterId = canisterEnv?.["PUBLIC_CANISTER_ID:backend"];
  if (!canisterId) throw new Error("Canister ID for backend is not set. Deploy first.");
  const agent = await HttpAgent.create({
    identity: authClient ? await authClient.getIdentity() : undefined,
    host: window.location.origin,
    rootKey: canisterEnv?.IC_ROOT_KEY,
  });
  backendActor = createActor(canisterId, { agent });
  return backendActor;
}

function setStatus(id, msg) {
  const el = document.getElementById(id);
  if (el) el.textContent = msg;
}

function getInputValue(id) {
  return document.getElementById(id)?.value ?? "";
}

function hexEncode(bytes) {
  return Array.from(bytes)
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}

function hexDecode(str) {
  const matches = str.match(/.{1,2}/g);
  if (!matches) throw new Error("Invalid hex string");
  return new Uint8Array(matches.map((b) => parseInt(b, 16)));
}

async function handleFetchVetKey() {
  if (!myPrincipal) {
    setStatus("sym-status", "Please log in first.");
    return;
  }
  setStatus("sym-status", "Fetching VetKey…");
  try {
    const actor = await getActor();

    const pkBytes = Uint8Array.from(await actor.symmetric_key_verification_key());
    const dpk = DerivedPublicKey.deserialize(pkBytes);

    const tsk = TransportSecretKey.random();
    const encBytes = Uint8Array.from(
      await actor.encrypted_symmetric_key_for_caller(tsk.publicKeyBytes())
    );

    const vetKey = EncryptedVetKey.deserialize(encBytes).decryptAndVerify(
      tsk,
      dpk,
      myPrincipal.toUint8Array()
    );

    symKeyMaterial = await vetKey.asDerivedKeyMaterial();
    setStatus("sym-status", "VetKey fetched. Ready to encrypt/decrypt.");
    updateSymButtons();
  } catch (e) {
    setStatus("sym-status", `Error: ${e}`);
  }
}

async function handleEncrypt() {
  const plaintext = getInputValue("plaintext");
  if (!plaintext) {
    setStatus("encrypt-result", "Enter plaintext first.");
    return;
  }
  try {
    const ciphertext = await symKeyMaterial.encryptMessage(
      new TextEncoder().encode(plaintext),
      "basic-vetkd-demo",
      ""
    );
    const ciphertextHex = hexEncode(ciphertext);
    setStatus("encrypt-result", "Ciphertext (hex): " + ciphertextHex);
    document.getElementById("ciphertext").value = ciphertextHex;
  } catch (e) {
    setStatus("encrypt-result", `Error: ${e}`);
  }
}

async function handleDecrypt() {
  const ciphertextHex = getInputValue("ciphertext");
  if (!ciphertextHex) {
    setStatus("decrypt-result", "Enter ciphertext first.");
    return;
  }
  try {
    const plaintext = await symKeyMaterial.decryptMessage(
      hexDecode(ciphertextHex),
      "basic-vetkd-demo",
      ""
    );
    setStatus("decrypt-result", "Plaintext: " + new TextDecoder().decode(plaintext));
  } catch (e) {
    setStatus("decrypt-result", `Error: ${e}`);
  }
}

async function handleIbeEncrypt() {
  const plaintext = getInputValue("ibe-plaintext");
  const receiverText = getInputValue("ibe-receiver");
  if (!plaintext || !receiverText) {
    setStatus("ibe-encrypt-result", "Fill in both plaintext and recipient principal.");
    return;
  }
  setStatus("ibe-encrypt-result", "Fetching IBE encryption key…");
  try {
    if (!ibeDerivedPublicKey) {
      const actor = await getActor();
      const pkBytes = Uint8Array.from(await actor.ibe_encryption_key());
      ibeDerivedPublicKey = DerivedPublicKey.deserialize(pkBytes);
    }
    const receiver = Principal.fromText(receiverText);
    const ciphertext = IbeCiphertext.encrypt(
      ibeDerivedPublicKey,
      IbeIdentity.fromPrincipal(receiver),
      new TextEncoder().encode(plaintext),
      IbeSeed.random()
    );
    const ciphertextHex = hexEncode(ciphertext.serialize());
    setStatus("ibe-encrypt-result", "IBE ciphertext (hex): " + ciphertextHex);
    document.getElementById("ibe-ciphertext").value = ciphertextHex;
  } catch (e) {
    setStatus("ibe-encrypt-result", `Error: ${e}`);
  }
}

async function handleIbeDecrypt() {
  if (!myPrincipal) {
    setStatus("ibe-decrypt-result", "Please log in first.");
    return;
  }
  const ciphertextHex = getInputValue("ibe-ciphertext");
  if (!ciphertextHex) {
    setStatus("ibe-decrypt-result", "Enter IBE ciphertext first.");
    return;
  }
  setStatus("ibe-decrypt-result", "Fetching IBE decryption key…");
  try {
    const actor = await getActor();
    if (!ibeDerivedPublicKey) {
      const pkBytes = Uint8Array.from(await actor.ibe_encryption_key());
      ibeDerivedPublicKey = DerivedPublicKey.deserialize(pkBytes);
    }
    const tsk = TransportSecretKey.random();
    const encBytes = Uint8Array.from(
      await actor.encrypted_ibe_decryption_key_for_caller(tsk.publicKeyBytes())
    );
    const vetKey = EncryptedVetKey.deserialize(encBytes).decryptAndVerify(
      tsk,
      ibeDerivedPublicKey,
      myPrincipal.toUint8Array()
    );
    const ibeCiphertext = IbeCiphertext.deserialize(hexDecode(ciphertextHex));
    const plaintext = ibeCiphertext.decrypt(vetKey);
    setStatus("ibe-decrypt-result", "Plaintext: " + new TextDecoder().decode(plaintext));
  } catch (e) {
    setStatus("ibe-decrypt-result", `Error: ${e}`);
  }
}

function updateSymButtons() {
  const hasKey = symKeyMaterial !== undefined;
  document.getElementById("encrypt-btn").disabled = !hasKey;
  document.getElementById("decrypt-btn").disabled = !hasKey;
}

function updateUI(isAuthenticated) {
  document.getElementById("login-btn").classList.toggle("hidden", isAuthenticated);
  document.getElementById("logout-btn").classList.toggle("hidden", !isAuthenticated);
  document.getElementById("principal-display").classList.toggle("hidden", !isAuthenticated);
  if (isAuthenticated && myPrincipal) {
    document.getElementById("principal-display").textContent =
      `Principal: ${myPrincipal.toString()}`;
  }
}

async function initAuth() {
  const isLocal =
    window.location.hostname === "localhost" ||
    window.location.hostname.endsWith(".localhost");
  authClient = new AuthClient({
    identityProvider: isLocal
      ? "http://id.ai.localhost:8000/authorize"
      : "https://id.ai/authorize",
    ...(isLocal ? { storage: new LocalStorage(), keyType: "Ed25519" } : {}),
  });
  if (authClient.isAuthenticated()) {
    myPrincipal = (await authClient.getIdentity()).getPrincipal();
    updateUI(true);
  } else {
    updateUI(false);
  }
}

document.querySelector("#app").innerHTML = `
  <div class="container">
    <h1>VetKD Raw API Demo</h1>
    <p class="subtitle">
      Demonstrates the raw VetKD management canister API for
      symmetric key derivation (AES-GCM-256) and identity-based encryption (IBE).
    </p>

    <div class="auth-bar">
      <span id="principal-display" class="hidden"></span>
      <button id="login-btn">Login with Internet Identity</button>
      <button id="logout-btn" class="hidden">Logout</button>
    </div>

    <hr />

    <section>
      <h2>Symmetric Key (AES-GCM-256)</h2>
      <p class="hint">
        Derives a per-caller AES-GCM-256 key via the VetKD management canister.
        Login is required so the key is bound to your principal.
      </p>
      <button id="fetch-vetkey-btn">Fetch VetKey</button>
      <p id="sym-status" class="status"></p>

      <div class="form-row">
        <label for="plaintext">Plaintext</label>
        <input id="plaintext" type="text" placeholder="Enter text to encrypt" />
        <button id="encrypt-btn" disabled>Encrypt</button>
      </div>
      <p id="encrypt-result" class="result"></p>

      <div class="form-row">
        <label for="ciphertext">Ciphertext (hex)</label>
        <input id="ciphertext" type="text" placeholder="Paste hex ciphertext" />
        <button id="decrypt-btn" disabled>Decrypt</button>
      </div>
      <p id="decrypt-result" class="result"></p>
    </section>

    <hr />

    <section>
      <h2>Identity-Based Encryption (IBE)</h2>
      <p class="hint">
        Encrypts for any principal without needing their public key.
        Decryption requires the recipient to be logged in.
      </p>

      <div class="form-group">
        <div class="form-row">
          <label for="ibe-plaintext">Plaintext</label>
          <input id="ibe-plaintext" type="text" placeholder="Enter text to encrypt" />
        </div>
        <div class="form-row">
          <label for="ibe-receiver">Recipient principal</label>
          <input id="ibe-receiver" type="text" placeholder="Principal ID" />
        </div>
        <button id="ibe-encrypt-btn">IBE Encrypt</button>
      </div>
      <p id="ibe-encrypt-result" class="result"></p>

      <div class="form-row">
        <label for="ibe-ciphertext">IBE ciphertext (hex)</label>
        <input id="ibe-ciphertext" type="text" placeholder="Paste IBE hex ciphertext" />
        <button id="ibe-decrypt-btn">IBE Decrypt (for myself)</button>
      </div>
      <p id="ibe-decrypt-result" class="result"></p>
    </section>
  </div>
`;

document.getElementById("login-btn").addEventListener("click", () => {
  void (async () => {
    try {
      const identity = await authClient.signIn({
        maxTimeToLive: BigInt(1800) * BigInt(1_000_000_000),
      });
      myPrincipal = identity.getPrincipal();
      backendActor = undefined;
      updateUI(true);
    } catch (e) {
      alert(`Login failed: ${e}`);
    }
  })();
});

document.getElementById("logout-btn").addEventListener("click", () => {
  void authClient?.signOut();
  myPrincipal = undefined;
  backendActor = undefined;
  symKeyMaterial = undefined;
  ibeDerivedPublicKey = undefined;
  setStatus("sym-status", "");
  updateUI(false);
  updateSymButtons();
});

document.getElementById("fetch-vetkey-btn").addEventListener("click", () => void handleFetchVetKey());
document.getElementById("encrypt-btn").addEventListener("click", () => void handleEncrypt());
document.getElementById("decrypt-btn").addEventListener("click", () => void handleDecrypt());
document.getElementById("ibe-encrypt-btn").addEventListener("click", () => void handleIbeEncrypt());
document.getElementById("ibe-decrypt-btn").addEventListener("click", () => void handleIbeDecrypt());

void initAuth();
