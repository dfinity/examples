import { render, html } from "lit-html";
import { displayError } from "../components/displayError";
import { warningIcon } from "../components/icons";
import { withLoader } from "../components/loader";
import { initLogout, logoutSection } from "../components/logout";
import { IIConnection } from "../utils/iiConnection";
import { parseUserNumber } from "../utils/userNumber";
import { unknownToString } from "../utils/utils";
import { pickDeviceAlias } from "./addDevicePickAlias";
import { successfullyAddedDevice } from "./successfulDeviceAddition";
import { DerEncodedPublicKey } from "@dfinity/agent";
import { fromHexString } from "@dfinity/identity/lib/cjs/buffer";

const pageContent = (userNumber: bigint) => html`
  <div class="container">
    <h1>New device</h1>
    <label>Identity Anchor:</label>
    <div class="highlightBox">${userNumber}</div>
    <div class="warningBox">
      <span class="warningIcon">${warningIcon}</span>
      <div class="warningMessage">
        Click the button below only if you have transferred the link from your
        other device.
      </div>
    </div>
    <button type="button" class="primary" id="addDevice">
      Yes, add new device
    </button>
    <button type="button" id="cancelAdd">Cancel</button>
    ${logoutSection()}
  </div>
`;

export const addDevice = (
  userNumber: bigint,
  connection: IIConnection
): void => {
  const container = document.getElementById("pageContent") as HTMLElement;
  render(pageContent(userNumber), container);
  init(userNumber, connection);
};

const init = (userNumber: bigint, connection: IIConnection) => {
  initLogout();
  const addDeviceButton = document.querySelector(
    "#addDevice"
  ) as HTMLButtonElement;
  const cancelAdd = document.querySelector("#cancelAdd") as HTMLButtonElement;
  cancelAdd.onclick = () => {
    clearHash();
    window.location.reload();
  };
  addDeviceButton.onclick = async () => {
    // Check URL if user has pasted in an Add Identity link
    const url = new URL(document.URL);
    const parsedParams = parseNewDeviceParam(url.hash?.split("device=")[1]);

    if (parsedParams) {
      const { userNumber: expectedUserNumber, publicKey, rawId } = parsedParams;
      if (expectedUserNumber !== userNumber) {
        // Here we're adding a device to our userNumber that was supposed to be added to a different one.
        await displayError({
          title: "Wrong Identity Anchor",
          message: `We're expecting to add a device to the Identity Anchor ${expectedUserNumber}, but you're logged in as ${userNumber}. Please choose the correct Identity Anchor when creating the add device link, or log in with the expected Identity Anchor.`,
          primaryButton: "Back to Authenticate",
        });
        window.location.reload();
      }
      console.log("Adding new device with:", parsedParams);
      try {
        const deviceName = await pickDeviceAlias();
        if (deviceName === null) {
          clearHash();
          return window.location.reload();
        }
        await withLoader(() =>
          connection.add(
            userNumber,
            deviceName,
            { unknown: null },
            { authentication: null },
            publicKey,
            rawId
          )
        );
        clearHash();
        successfullyAddedDevice(deviceName, userNumber, connection);
      } catch (error: unknown) {
        // If anything goes wrong, or the user cancels we do _not_ want to add the device.
        await displayError({
          title: "Failed to add the device",
          message:
            "Something went wrong when adding the new device. Please try again",
          detail: unknownToString(error, "Unknown error type"),
          primaryButton: "Back to Authenticate",
        });
        window.location.reload();
      }
    } else {
      await displayError({
        title: "Not a valid link",
        message:
          "We failed to recognize your add device link. Please make sure you copy the entire link, and only use links you created yourself.",
        primaryButton: "Back to Authenticate",
      });
      clearHash();
      window.location.reload();
    }
  };
};

const parseNewDeviceParam = (
  param: string
): {
  userNumber: bigint;
  publicKey: DerEncodedPublicKey;
  rawId?: ArrayBuffer;
} | null => {
  const segments = param.split(";");
  if (!(segments.length === 2 || segments.length === 3)) {
    return null;
  }
  const userNumber = parseUserNumber(segments[0]);
  if (userNumber === null) {
    return null;
  }
  const publicKey = fromHexString(segments[1]) as DerEncodedPublicKey;
  const rawId = segments[2] !== "" ? fromHexString(segments[2]) : undefined;
  return { userNumber, publicKey, rawId };
};

export const clearHash = (): void => {
  history.pushState(
    // Preserve the #authorize hash if it's present.
    /authorize/.test(window.location.hash) ? "authorize" : "",
    document.title,
    window.location.pathname + window.location.search
  );
};
