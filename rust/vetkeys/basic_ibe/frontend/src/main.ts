import "./style.css";
import { Principal } from "@icp-sdk/core/principal";
import {
    TransportSecretKey,
    DerivedPublicKey,
    EncryptedVetKey,
    VetKey,
    IbeCiphertext,
    IbeIdentity,
    IbeSeed,
} from "@icp-sdk/vetkeys";
import { createActor, type Backend, type Inbox } from "./declarations/basic_ibe/backend";
import { AuthClient } from "@icp-sdk/auth/client";
import { HttpAgent } from "@icp-sdk/core/agent";
import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";

const canisterEnv = safeGetCanisterEnv<{
    "PUBLIC_CANISTER_ID:basic_ibe": string;
}>();

let ibePrivateKey: VetKey | undefined = undefined;
let ibePublicKey: DerivedPublicKey | undefined = undefined;
let myPrincipal: Principal | undefined = undefined;
let authClient: AuthClient | undefined;
let basicIbeActor: Backend | undefined;

async function getBasicIbeActor(): Promise<Backend> {
    if (basicIbeActor) return basicIbeActor;
    const canisterId = canisterEnv?.["PUBLIC_CANISTER_ID:basic_ibe"];
    if (!canisterId) {
        throw Error("Canister ID for basic_ibe is not set");
    }
    if (!authClient) {
        throw Error("Auth client is not initialized");
    }

    const agent = await HttpAgent.create({
        identity: await authClient.getIdentity(),
        host: window.location.origin,
        ...(canisterEnv?.IC_ROOT_KEY
            ? { rootKey: canisterEnv.IC_ROOT_KEY }
            : {}),
    });
    basicIbeActor = createActor(canisterId, { agent });

    return basicIbeActor;
}

async function getIbePublicKey(): Promise<DerivedPublicKey> {
    if (ibePublicKey) return ibePublicKey;
    const actor = await getBasicIbeActor();
    ibePublicKey = DerivedPublicKey.deserialize(
        new Uint8Array(await actor.get_ibe_public_key()),
    );
    return ibePublicKey;
}

async function encrypt(
    cleartext: Uint8Array,
    receiver: Principal,
): Promise<Uint8Array> {
    const publicKey = await getIbePublicKey();
    const ciphertext = IbeCiphertext.encrypt(
        publicKey,
        IbeIdentity.fromPrincipal(receiver),
        cleartext,
        IbeSeed.random(),
    );
    return ciphertext.serialize();
}

async function getMyIbePrivateKey(): Promise<VetKey> {
    if (ibePrivateKey) return ibePrivateKey;

    if (!myPrincipal) {
        throw Error("My principal is not set");
    } else {
        const transportSecretKey = TransportSecretKey.random();
        const actor = await getBasicIbeActor();
        const encryptedKey = Uint8Array.from(
            await actor.get_my_encrypted_ibe_key(
                transportSecretKey.publicKeyBytes(),
            ),
        );
        ibePrivateKey = EncryptedVetKey.deserialize(
            encryptedKey,
        ).decryptAndVerify(
            transportSecretKey,
            await getIbePublicKey(),
            new Uint8Array(myPrincipal.toUint8Array()),
        );
        return ibePrivateKey;
    }
}

async function decryptMessage(encryptedMessage: Uint8Array): Promise<string> {
    const ibeKey = await getMyIbePrivateKey();
    const ciphertext = IbeCiphertext.deserialize(encryptedMessage);
    const plaintext = ciphertext.decrypt(ibeKey);
    return new TextDecoder().decode(plaintext);
}

async function sendMessage() {
    const message = prompt("Enter your message:");
    if (!message) throw Error("Message is required");

    const receiver = prompt("Enter receiver principal:");
    if (!receiver) throw Error("Receiver is required");

    const receiverPrincipal = Principal.fromText(receiver);

    try {
        const encryptedMessage = await encrypt(
            new TextEncoder().encode(message),
            receiverPrincipal,
        );

        const actor = await getBasicIbeActor();
        const result = await actor.send_message({
            encrypted_message: encryptedMessage,
            receiver: receiverPrincipal,
        });

        if ("Err" in result) {
            console.error("Error sending message:", result.Err);
            alert("Error sending message: " + result.Err);
        } else {
            alert("Message sent successfully!");
        }
    } catch (error) {
        console.error("Error sending message:", error);
        alert("Error sending message: " + (error as Error).message);
    }
}

async function showMessages() {
    const actor = await getBasicIbeActor();
    const inbox = await actor.get_my_messages();
    await displayMessages(inbox);
}

function createMessageElement(
    sender: Principal,
    timestamp: bigint,
    plaintextString: string,
    index: number,
): HTMLDivElement {
    const messageElement = document.createElement("div");
    messageElement.className = "message";

    const messageContent = document.createElement("div");
    messageContent.className = "message-content";

    const messageText = document.createElement("div");
    messageText.className = "message-text";
    messageText.textContent = plaintextString;

    const messageInfo = document.createElement("div");
    messageInfo.className = "message-info";

    const senderInfo = document.createElement("div");
    senderInfo.className = "sender";
    senderInfo.textContent = `From: ${sender.toString()}`;

    const timestampInfo = document.createElement("div");
    timestampInfo.className = "timestamp";
    const date = new Date(Number(timestamp) / 1_000_000);
    timestampInfo.textContent = `Sent: ${date.toLocaleString()}`;

    const messageActions = document.createElement("div");
    messageActions.className = "message-actions";

    const deleteButton = document.createElement("button");
    deleteButton.className = "delete-button";
    deleteButton.textContent = "Delete";
    deleteButton.dataset.index = index.toString();

    messageActions.appendChild(deleteButton);
    messageInfo.appendChild(senderInfo);
    messageInfo.appendChild(timestampInfo);
    messageContent.appendChild(messageText);
    messageContent.appendChild(messageInfo);
    messageContent.appendChild(messageActions);
    messageElement.appendChild(messageContent);

    return messageElement;
}

async function displayMessages(inbox: Inbox) {
    const messagesDiv = document.getElementById("messages")!;
    messagesDiv.innerHTML = "";

    if (inbox.messages.length === 0) {
        const noMessagesDiv = document.createElement("div");
        noMessagesDiv.className = "no-messages";
        noMessagesDiv.textContent = "No messages in the inbox.";
        messagesDiv.appendChild(noMessagesDiv);
        return;
    }

    // Iterate through messages in reverse order
    for (let i = inbox.messages.length - 1; i >= 0; i--) {
        const message = inbox.messages[i];
        const plaintextString = await decryptMessage(
            new Uint8Array(message.encrypted_message),
        );

        const messageElement = createMessageElement(
            message.sender,
            message.timestamp,
            plaintextString,
            i,
        );
        messagesDiv.appendChild(messageElement);
    }

    // Add event listeners to delete buttons
    const deleteButtons = document.querySelectorAll(".delete-button");
    deleteButtons.forEach((button) => {
        button.addEventListener("click", (e) => {
            const target = e.target as HTMLButtonElement;
            const index = parseInt(target.dataset.index!);

            // Disable all delete buttons
            deleteButtons.forEach(
                (btn) => ((btn as HTMLButtonElement).disabled = true),
            );

            void (async () => {
                try {
                    const actor = await getBasicIbeActor();
                    const result = await actor.remove_my_message_by_index(
                        BigInt(index),
                    );
                    if ("Err" in result) {
                        console.error("Error deleting message:", result.Err);
                        alert("Error deleting message: " + result.Err);
                    } else {
                        // Re-load all messages to refresh message indices
                        await showMessages();
                    }
                } catch (error) {
                    console.error("Error deleting message:", error);
                    alert(
                        "Error deleting message: " + (error as Error).message,
                    );
                }
            })();
        });
    });
}

export async function login(client: AuthClient): Promise<void> {
    try {
        const identity = await client.signIn({
            maxTimeToLive: BigInt(1800) * BigInt(1_000_000_000),
        });
        myPrincipal = identity.getPrincipal();
        updateUI(true);
    } catch (error: unknown) {
        console.error("Authentication failed:", error);
        alert("Authentication failed: " + error);
    }
}

export function logout() {
    void authClient?.signOut();
    const messagesDiv = document.getElementById("messages")!;
    messagesDiv.innerHTML = "";
    ibePrivateKey = undefined;
    myPrincipal = undefined;
    basicIbeActor = undefined;
    updateUI(false);
}

async function initAuth() {
    const isLocal =
        window.location.hostname === "localhost" ||
        window.location.hostname.endsWith(".localhost");
    authClient = new AuthClient({
        identityProvider: isLocal
            ? "http://id.ai.localhost:8000"
            : "https://id.ai",
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
    const messageButtons = document.getElementById("messageButtons")!;
    const principalDisplay = document.getElementById("principalDisplay")!;
    const logoutButton = document.getElementById("logoutButton")!;

    loginButton.classList.toggle("hidden", isAuthenticated);
    messageButtons.classList.toggle("hidden", !isAuthenticated);
    principalDisplay.classList.toggle("hidden", !isAuthenticated);
    logoutButton.classList.toggle("hidden", !isAuthenticated);

    if (isAuthenticated && myPrincipal) {
        principalDisplay.textContent = `Principal: ${myPrincipal.toString()}`;
    }
}

function handleLogin() {
    if (!authClient) {
        console.error("Auth client not initialized");
        alert("Auth client not initialized");
        return;
    }

    void login(authClient);
}

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
    <h1>Basic IBE Message System with VetKeys</h1>
    <div class="principal-container">
      <div id="principalDisplay" class="principal-display"></div>
      <button id="logoutButton">Logout</button>
    </div>
    <div class="login-container">
      <button id="loginButton">Login</button>
    </div>
    <div id="messageButtons" class="buttons">
      <button id="sendMessage">Send Message</button>
      <button id="showMessages">Show My Messages</button>
    </div>
    <div id="messages"></div>
  </div>
`;

// Add event listeners
document.getElementById("loginButton")!.addEventListener("click", handleLogin);
document.getElementById("logoutButton")!.addEventListener("click", logout);
document.getElementById("sendMessage")!.addEventListener("click", () => {
    void (async () => {
        try {
            await sendMessage();
        } catch (error: unknown) {
            const msg = (error as Error).message ?? String(error);
            console.error("Error in sendMessage:", error);
            alert(msg);
        }
    })();
});
document.getElementById("showMessages")!.addEventListener("click", () => {
    void (async () => {
        try {
            await showMessages();
        } catch (error: unknown) {
            console.error("Error in showMessages:", error);
            alert("Error loading messages: " + (error as Error).message);
        }
    })();
});

// Initialize auth
void initAuth();
