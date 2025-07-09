import { createActor, app_backend } from "../../declarations/app_backend";
import { TransportSecretKey, EncryptedVetKey, DerivedPublicKey, IbeCiphertext, IbeIdentity, IbeSeed } from "@dfinity/vetkeys";
import { AuthClient } from "@dfinity/auth-client"
import { HttpAgent, Actor } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";

let fetched_derived_key_material = null;
let app_backend_actor = app_backend;
let app_backend_principal = await Actor.agentOf(app_backend_actor).getPrincipal();
document.getElementById("principal").innerHTML = annotated_principal(app_backend_principal);

document.getElementById("get_vetkey_form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const button = e.target.querySelector("button");
  button.setAttribute("disabled", true);
  const result = document.getElementById("get_vetkey_result");

  result.innerText = "Fetching vetKey...";
  const derived_key_material = await get_derived_key_material();
  result.innerText = "Done. vetKey available for local usage.";

  button.removeAttribute("disabled");

  fetched_derived_key_material = derived_key_material;
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
  const message = document.getElementById("plaintext").value;
  const message_encoded = new TextEncoder().encode(message);
  const ciphertext_hex = hex_encode(await fetched_derived_key_material.encryptMessage(message_encoded, "vetkd-demo"));

  result.innerText = "ciphertext: " + ciphertext_hex;

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
    const ciphertext_hex = document.getElementById("ciphertext").value;
    const plaintext_bytes = await fetched_derived_key_material.decryptMessage(hex_decode(ciphertext_hex), "vetkd-demo");
    const plaintext_string = new TextDecoder().decode(plaintext_bytes);
    result.innerText = "plaintext: " + plaintext_string;
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
  if (document.getElementById("plaintext").value === "" || fetched_derived_key_material === null) {
    submit_plaintext_button.setAttribute("disabled", true);
  } else {
    submit_plaintext_button.removeAttribute("disabled");
  }
}

function update_ciphertext_button_state() {
  const submit_ciphertext_button = document.getElementById("submit_ciphertext");
  if (document.getElementById("ciphertext").value === "" || fetched_derived_key_material === null) {
    submit_ciphertext_button.setAttribute("disabled", true);
  } else {
    submit_ciphertext_button.removeAttribute("disabled");
  }
}

async function get_derived_key_material() {
  const tsk = TransportSecretKey.random();

  const ek_bytes_hex = await app_backend_actor.encrypted_symmetric_key_for_caller(tsk.publicKeyBytes());
  const encryptedVetKey = new EncryptedVetKey(hex_decode(ek_bytes_hex));

  const pk_bytes_hex = await app_backend_actor.symmetric_key_verification_key();
  const dpk = DerivedPublicKey.deserialize(hex_decode(pk_bytes_hex));

  const vetKey = encryptedVetKey.decryptAndVerify(tsk, dpk, app_backend_principal.toUint8Array());

  return await vetKey.asDerivedKeyMaterial();
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
  const dpk = DerivedPublicKey.deserialize(hex_decode(pk_bytes_hex));

  document.getElementById("ibe_encrypt_result").innerText = "Preparing IBE-encryption..."
  const message_encoded = new TextEncoder().encode(message);
  let ibe_principal = Principal.fromText(document.getElementById("ibe_principal").value);

  document.getElementById("ibe_encrypt_result").innerText = "IBE-encrypting for principal" + ibe_principal.toText() + "...";
  const ibe_ciphertext = IbeCiphertext.encrypt(
    dpk,
    IbeIdentity.fromPrincipal(ibe_principal),
    message_encoded,
    IbeSeed.random(),
  );
  return hex_encode(ibe_ciphertext.serialize());
}

async function ibe_decrypt(ibe_ciphertext_hex) {
  document.getElementById("ibe_decrypt_result").innerText = "Fetching IBE enryption key (needed for verification)..."
  const pk_bytes_hex = await app_backend_actor.ibe_encryption_key();
  const dpk = DerivedPublicKey.deserialize(hex_decode(pk_bytes_hex));

  document.getElementById("ibe_decrypt_result").innerText = "Fetching IBE decryption key..."
  const tsk = TransportSecretKey.random();
  const ek_bytes_hex = await app_backend_actor.encrypted_ibe_decryption_key_for_caller(tsk.publicKeyBytes());
  const encryptedVetKey = new EncryptedVetKey(hex_decode(ek_bytes_hex));

  document.getElementById("ibe_decrypt_result").innerText = "Decrypting and verifying IBE decryption key..."
  const vetKey = encryptedVetKey.decryptAndVerify(
    tsk,
    dpk,
    app_backend_principal.toUint8Array()
  );

  document.getElementById("ibe_decrypt_result").innerText = "Using IBE decryption key to decrypt ciphertext..."
  const ibe_ciphertext = IbeCiphertext.deserialize(hex_decode(ibe_ciphertext_hex));
  const ibe_plaintext = ibe_ciphertext.decrypt(vetKey);
  return new TextDecoder().decode(ibe_plaintext);
}

document.getElementById("login").onclick = async (e) => {
  e.preventDefault();

  // According to https://github.com/dfinity/internet-identity?tab=readme-ov-file#local-replica,
  // for local deployments, the II URL must be different depending on the browser:
  // Chrome, Firefox: http://<canister_id>.localhost:4943
  // Safari: http://localhost:4943?canisterId=<canister_id>
  // 
  // Safari detection rules are according to: https://developer.mozilla.org/en-US/docs/Web/HTTP/Browser_detection_using_the_user_agent#browser_name_and_version
  let isSafari = /^(?!.*chrome\/\d+)(?!.*chromium\/\d+).*safari\/\d+/i.test(navigator.userAgent);
  let identityProvider = isSafari ?
    `http://127.0.0.1:4943/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}` :
    `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943/`;

  let authClient = await AuthClient.create();
  await new Promise((resolve) => {
    authClient.login({
      identityProvider: identityProvider,
      onSuccess: resolve,
    });
  });
  // At this point we're authenticated, and we can get the identity from the auth client:
  const identity = authClient.getIdentity();
  // Using the identity obtained from the auth client, we can create an agent to interact with the IC.
  const agent = new HttpAgent({ identity });
  // Using the interface description of our webapp, we create an actor that we use to call the service methods. We override the global actor, such that the other button handler will automatically use the new actor with the Internet Identity provided delegation.
  app_backend_actor = createActor(process.env.CANISTER_ID_APP_BACKEND, {
    agent,
  });
  app_backend_principal = identity.getPrincipal();

  document.getElementById("principal").innerHTML = annotated_principal(app_backend_principal);

  fetched_derived_key_material = null;
  document.getElementById("get_vetkey_result").innerText = "";
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