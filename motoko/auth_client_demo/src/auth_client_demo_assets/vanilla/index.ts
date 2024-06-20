import { AuthClient } from "@dfinity/auth-client";
import { handleAuthenticated, renderIndex } from "./views";

// One day in nanoseconds
const days = BigInt(1);
const hours = BigInt(24);
const nanoseconds = BigInt(3600000000000);

export const getIdentityProvider = () => {
  let idpProvider;
  // Safeguard against server rendering
  if (typeof window !== "undefined") {
    const isLocal = process.env.DFX_NETWORK !== "ic";
    // Safari does not support localhost subdomains
    const isSafari = /^((?!chrome|android).)*safari/i.test(navigator.userAgent);
    if (isLocal && isSafari) {
      idpProvider = `http://localhost:4943/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}`;
    } else if (isLocal) {
      idpProvider = `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943`;
    }
  }
  return idpProvider;
};

export const defaultOptions = {
  /**
   *  @type {import("@dfinity/auth-client").AuthClientCreateOptions}
   */
  createOptions: {
    idleOptions: {
      // Set to true if you do not want idle functionality
      disableIdle: true,
    },
  },
  /**
   * @type {import("@dfinity/auth-client").AuthClientLoginOptions}
   */
  loginOptions: {
    identityProvider: getIdentityProvider(),
  },
};

const init = async () => {
  const authClient = await AuthClient.create(defaultOptions.createOptions);

  if (await authClient.isAuthenticated()) {
    handleAuthenticated(authClient);
  }
  renderIndex();
  setupToast();
};

async function setupToast() {
  const status = document.getElementById("status");
  const closeButton = status?.querySelector("button");
  closeButton?.addEventListener("click", () => {
    status?.classList.add("hidden");
  });
}

init();
