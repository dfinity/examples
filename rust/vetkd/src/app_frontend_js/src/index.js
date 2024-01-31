import { createActor, app_backend } from "../../declarations/app_backend";
import * as vetkd from "ic-vetkd-utils";
import { AuthClient } from "@dfinity/auth-client"
import { HttpAgent, Actor } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";

let fetched_symmetric_key = null;
let app_backend_actor = app_backend;
let app_backend_principal = await Actor.agentOf(app_backend_actor).getPrincipal();
document.getElementById("principal").innerHTML = annotated_principal(app_backend_principal);

document.getElementById("get_symmetric_key_form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const button = e.target.querySelector("button");
  button.setAttribute("disabled", true);
  const result = document.getElementById("get_symmetric_key_result");

  result.innerText = "Fetching symmetric key...";
  const aes_256_key = await get_aes_256_gcm_key();
  result.innerText = "Done. AES-GCM-256 key available for local usage.";

  button.removeAttribute("disabled");

  fetched_symmetric_key = aes_256_key;
  update_plaintext_button_state();
  update_ciphertext_button_state();

  return false;
});

document.getElementById("encrypt_form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const button = e.target.querySelector("button");
  button.setAttribute("disabled", true);
  const result = document.getElementById("encrypt_result");

  result.innerText = "Encrypting...";
  const ciphertext = await aes_gcm_encrypt(document.getElementById("plaintext").value, fetched_symmetric_key);

  result.innerText = "ciphertext: " + ciphertext;

  button.removeAttribute("disabled");
  return false;
});

document.getElementById("decrypt_form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const button = e.target.querySelector("button");
  button.setAttribute("disabled", true);
  const result = document.getElementById("decrypt_result");

  result.innerText = "Decrypting...";
  try {
    const plaintext = await aes_gcm_decrypt(document.getElementById("ciphertext").value, fetched_symmetric_key);
    result.innerText = "plaintext: " + plaintext;
  } catch (e) {
    result.innerText = "Error: " + e;
  }

  button.removeAttribute("disabled");
  return false;
});

document.getElementById("plaintext").addEventListener("keyup", async (e) => {
  update_plaintext_button_state();
});

document.getElementById("ciphertext").addEventListener("keyup", async (e) => {
  update_ciphertext_button_state();
});

function update_plaintext_button_state() {
  const submit_plaintext_button = document.getElementById("submit_plaintext");
  if (document.getElementById("plaintext").value === "" || fetched_symmetric_key === null) {
    submit_plaintext_button.setAttribute("disabled", true);
  } else {
    submit_plaintext_button.removeAttribute("disabled");
  }
}

function update_ciphertext_button_state() {
  const submit_ciphertext_button = document.getElementById("submit_ciphertext");
  if (document.getElementById("ciphertext").value === "" || fetched_symmetric_key === null) {
    submit_ciphertext_button.setAttribute("disabled", true);
  } else {
    submit_ciphertext_button.removeAttribute("disabled");
  }
}

async function get_aes_256_gcm_key() {
  const seed = window.crypto.getRandomValues(new Uint8Array(32));
  const tsk = new vetkd.TransportSecretKey(seed);
  const ek_bytes_hex = await app_backend_actor.encrypted_symmetric_key_for_caller(tsk.public_key());
  const pk_bytes_hex = await app_backend_actor.symmetric_key_verification_key();
  return tsk.decrypt_and_hash(
    hex_decode(ek_bytes_hex),
    hex_decode(pk_bytes_hex),
    app_backend_principal.toUint8Array(),
    32,
    new TextEncoder().encode("aes-256-gcm")
  );
}

async function aes_gcm_encrypt(message, rawKey) {
  const iv = window.crypto.getRandomValues(new Uint8Array(12)); // 96-bits; unique per message
  const aes_key = await window.crypto.subtle.importKey("raw", rawKey, "AES-GCM", false, ["encrypt"]);
  const message_encoded = new TextEncoder().encode(message);
  const ciphertext_buffer = await window.crypto.subtle.encrypt(
    { name: "AES-GCM", iv: iv },
    aes_key,
    message_encoded
  );
  const ciphertext = new Uint8Array(ciphertext_buffer);
  var iv_and_ciphertext = new Uint8Array(iv.length + ciphertext.length);
  iv_and_ciphertext.set(iv, 0);
  iv_and_ciphertext.set(ciphertext, iv.length);
  return hex_encode(iv_and_ciphertext);
}

async function aes_gcm_decrypt(ciphertext_hex, rawKey) {
  const iv_and_ciphertext = hex_decode(ciphertext_hex);
  const iv = iv_and_ciphertext.subarray(0, 12); // 96-bits; unique per message
  const ciphertext = iv_and_ciphertext.subarray(12);
  const aes_key = await window.crypto.subtle.importKey("raw", rawKey, "AES-GCM", false, ["decrypt"]);
  let decrypted = await window.crypto.subtle.decrypt(
    { name: "AES-GCM", iv: iv },
    aes_key,
    ciphertext
  );
  return new TextDecoder().decode(decrypted);
}

document.getElementById("ibe_encrypt_form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const button = e.target.querySelector("button");
  button.setAttribute("disabled", true);
  const result = document.getElementById("ibe_encrypt_result");

  try {
    const ibe_ciphertext = await ibe_encrypt(document.getElementById("ibe_plaintext").value);
    result.innerText = "IBE ciphertext: " + ibe_ciphertext;
  } catch (e) {
    result.innerText = "Error: " + e;
  }

  button.removeAttribute("disabled");
  return false;
});

document.getElementById("ibe_decrypt_form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const button = e.target.querySelector("button");
  button.setAttribute("disabled", true);
  const result = document.getElementById("ibe_decrypt_result");

  try {
    const ibe_plaintext = await ibe_decrypt(document.getElementById("ibe_ciphertext").value);
    result.innerText = "IBE plaintext: " + ibe_plaintext;
  } catch (e) {
    result.innerText = "Error: " + e;
  }

  button.removeAttribute("disabled");
  return false;
});

document.getElementById("ibe_plaintext").addEventListener("keyup", async (e) => {
  update_ibe_encrypt_button_state();
});

document.getElementById("ibe_principal").addEventListener("keyup", async (e) => {
  update_ibe_encrypt_button_state();
});

document.getElementById("ibe_ciphertext").addEventListener("keyup", async (e) => {
  update_ibe_decrypt_button_state();
});

function update_ibe_encrypt_button_state() {
  const ibe_encrypt_button = document.getElementById("ibe_encrypt");
  if (document.getElementById("ibe_plaintext").value === "" || document.getElementById("ibe_principal").value === "") {
    ibe_encrypt_button.setAttribute("disabled", true);
  } else {
    ibe_encrypt_button.removeAttribute("disabled");
  }
}

function update_ibe_decrypt_button_state() {
  const ibe_decrypt_button = document.getElementById("ibe_decrypt");
  if (document.getElementById("ibe_ciphertext").value === "") {
    ibe_decrypt_button.setAttribute("disabled", true);
  } else {
    ibe_decrypt_button.removeAttribute("disabled");
  }
}

async function ibe_encrypt(message) {
  document.getElementById("ibe_encrypt_result").innerText = "Fetching IBE encryption key..."
  const pk_bytes_hex = await app_backend_actor.ibe_encryption_key();

  document.getElementById("ibe_encrypt_result").innerText = "Preparing IBE-encryption..."
  const message_encoded = new TextEncoder().encode(message);
  const seed = window.crypto.getRandomValues(new Uint8Array(32));
  let ibe_principal = Principal.fromText(document.getElementById("ibe_principal").value);

  document.getElementById("ibe_encrypt_result").innerText = "IBE-encrypting for principal" + ibe_principal.toText() + "...";
  const ibe_ciphertext = vetkd.IBECiphertext.encrypt(
    hex_decode(pk_bytes_hex),
    ibe_principal.toUint8Array(),
    message_encoded,
    seed
  );
  return hex_encode(ibe_ciphertext.serialize());
}

async function ibe_decrypt(ibe_ciphertext_hex) {
  document.getElementById("ibe_decrypt_result").innerText = "Preparing IBE-decryption..."
  const tsk_seed = window.crypto.getRandomValues(new Uint8Array(32));
  const tsk = new vetkd.TransportSecretKey(tsk_seed);
  document.getElementById("ibe_decrypt_result").innerText = "Fetching IBE decryption key..."
  const ek_bytes_hex = await app_backend_actor.encrypted_ibe_decryption_key_for_caller(tsk.public_key());
  document.getElementById("ibe_decrypt_result").innerText = "Fetching IBE enryption key (needed for verification)..."
  const pk_bytes_hex = await app_backend_actor.ibe_encryption_key();

  const k_bytes = tsk.decrypt(
    hex_decode(ek_bytes_hex),
    hex_decode(pk_bytes_hex),
    app_backend_principal.toUint8Array()
  );

  const ibe_ciphertext = vetkd.IBECiphertext.deserialize(hex_decode(ibe_ciphertext_hex));
  const ibe_plaintext = ibe_ciphertext.decrypt(k_bytes);
  return new TextDecoder().decode(ibe_plaintext);
}

document.getElementById("login").onclick = async (e) => {
  e.preventDefault();
  let authClient = await AuthClient.create();
  await new Promise((resolve) => {
    authClient.login({
      identityProvider: `http://127.0.0.1:4943/?canisterId=${process.env.INTERNET_IDENTITY_CANISTER_ID}`,
      onSuccess: resolve,
    });
  });
  // At this point we're authenticated, and we can get the identity from the auth client:
  const identity = authClient.getIdentity();
  // Using the identity obtained from the auth client, we can create an agent to interact with the IC.
  const agent = new HttpAgent({ identity });
  // Using the interface description of our webapp, we create an actor that we use to call the service methods. We override the global actor, such that the other button handler will automatically use the new actor with the Internet Identity provided delegation.
  app_backend_actor = createActor(process.env.APP_BACKEND_CANISTER_ID, {
    agent,
  });
  app_backend_principal = identity.getPrincipal();

  document.getElementById("principal").innerHTML = annotated_principal(app_backend_principal);

  fetched_symmetric_key = null;
  document.getElementById("get_symmetric_key_result").innerText = "";
  update_plaintext_button_state();
  update_ciphertext_button_state();

  return false;
};

function annotated_principal(principal) {
  let principal_string = principal.toString();
  if (principal_string == "2vxsx-fae") {
    return "Anonymous principal (2vxsx-fae)";
  } else {
    return "Principal: " + principal_string;
  }
}

const hex_decode = (hexString) =>
  Uint8Array.from(hexString.match(/.{1,2}/g).map((byte) => parseInt(byte, 16)));
const hex_encode = (bytes) =>
  bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '');