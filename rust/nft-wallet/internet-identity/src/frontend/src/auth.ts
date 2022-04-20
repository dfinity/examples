import { Principal } from "@dfinity/principal";
import {
  FrontendHostname,
  PublicKey,
  SignedDelegation,
  UserNumber,
} from "../generated/internet_identity_types";
import { withLoader } from "./components/loader";
import { confirmRedirect } from "./flows/confirmRedirect";
import { IIConnection } from "./utils/iiConnection";
import { hasOwnProperty } from "./utils/utils";

interface AuthRequest {
  kind: "authorize-client";
  sessionPublicKey: Uint8Array;
  maxTimeToLive?: bigint;
}

interface AuthResponseSuccess {
  kind: "authorize-client-success";
  delegations: {
    delegation: {
      pubkey: Uint8Array;
      expiration: bigint;
      targets?: Principal[];
    };
    signature: Uint8Array;
  }[];
  userPublicKey: Uint8Array;
}

interface AuthResponseFailure {
  kind: "authorize-client-failure";
  text: string;
}

type AuthResponse = AuthResponseSuccess | AuthResponseFailure;

// A message to signal that the II is ready to receive authorization requests.
const READY_MESSAGE = {
  kind: "authorize-ready",
};

/**
 * Setup an event listener to listen to authorize requests from the client.
 *
 * This method expects to be called after the login flow.
 */
export default async function setup(
  userNumber: UserNumber,
  connection: IIConnection
): Promise<void> {
  // Set up an event listener for receiving messages from the client.
  window.addEventListener("message", async (event) => {
    const message = event.data;
    if (message.kind === "authorize-client") {
      console.log("Handling authorize-client request.");
      const response = await handleAuthRequest(
        connection,
        userNumber,
        message,
        event.origin
      );
      (event.source as Window).postMessage(response, event.origin);
    } else {
      console.log(`Message of unknown kind received: ${message}`);
    }
  });

  // Send a message to indicate we're ready.
  // NOTE: Because `window.opener.origin` cannot be accessed, this message
  // is sent with "*" as the target origin. This is safe as no sensitive
  // information is being communicated here.
  if (window.opener !== null) {
    window.opener.postMessage(READY_MESSAGE, "*");
  } else {
    // If there's no `window.opener` a user has manually navigated to "/#authorize". Let's
    // redirect them to the non-hash version.
    window.location.hash = "";
    window.location.reload();
  }
}

async function handleAuthRequest(
  connection: IIConnection,
  userNumber: UserNumber,
  request: AuthRequest,
  hostname: FrontendHostname
): Promise<AuthResponse> {
  const userPrincipal = await withLoader(() =>
    connection.getPrincipal(userNumber, hostname)
  );

  if (!(await confirmRedirect(hostname, userPrincipal.toString()))) {
    return {
      kind: "authorize-client-failure",
      text: `User did not grant access to ${hostname}.`,
    };
  }

  return await withLoader(async () => {
    const sessionKey = Array.from(request.sessionPublicKey);
    const prepRes = await connection.prepareDelegation(
      userNumber,
      hostname,
      sessionKey,
      request.maxTimeToLive
    );
    if (prepRes.length !== 2) {
      throw new Error(
        `Error preparing the delegation. Result received: ${prepRes}`
      );
    }

    const [userKey, timestamp] = prepRes;

    // TODO: Signal failure to retrieve the delegation. Error page, or maybe redirect back with error?
    const signed_delegation = await retryGetDelegation(
      connection,
      userNumber,
      hostname,
      sessionKey,
      timestamp
    );

    // Parse the candid SignedDelegation into a format that `DelegationChain` understands.
    const parsed_signed_delegation = {
      delegation: {
        pubkey: Uint8Array.from(signed_delegation.delegation.pubkey),
        expiration: BigInt(signed_delegation.delegation.expiration),
        targets: undefined,
      },
      signature: Uint8Array.from(signed_delegation.signature),
    };

    return {
      kind: "authorize-client-success",
      delegations: [parsed_signed_delegation],
      userPublicKey: Uint8Array.from(userKey),
    };
  });
}

const retryGetDelegation = async (
  connection: IIConnection,
  userNumber: bigint,
  hostname: string,
  sessionKey: PublicKey,
  timestamp: bigint,
  maxRetries = 5
): Promise<SignedDelegation> => {
  for (let i = 0; i < maxRetries; i++) {
    // Linear backoff
    await new Promise((resolve) => {
      setInterval(resolve, 1000 * i);
    });
    const res = await connection.getDelegation(
      userNumber,
      hostname,
      sessionKey,
      timestamp
    );
    if (hasOwnProperty(res, "signed_delegation")) {
      return res.signed_delegation;
    }
  }
  throw new Error(
    `Failed to retrieve a delegation after ${maxRetries} retries.`
  );
};
