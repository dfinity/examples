import { WebAuthnIdentity } from "@dfinity/identity";
import { html, render } from "lit-html";
import { DeviceData } from "../../generated/internet_identity_types";
import { displayError } from "../components/displayError";
import {
  bufferEqual,
  creationOptions,
  IIConnection,
} from "../utils/iiConnection";
import { unknownToString } from "../utils/utils";
import { parseUserNumber, setUserNumber } from "../utils/userNumber";
import { displayAddDeviceLink } from "./displayAddDeviceLink";
import { toHexString } from "@dfinity/identity/lib/cjs/buffer";

const pageContent = (userNumber: bigint | null) => html`
  <div class="container">
    <h1>New device</h1>
    <p>
      Please provide the Identity Anchor to which you want to add your device.
    </p>
    <input
      type="text"
      id="addDeviceUserNumber"
      placeholder="Enter Identity Anchor"
      value=${userNumber ?? ""}
    />
    <button id="addDeviceUserNumberContinue" class="primary">Continue</button>
  </div>
`;

export const addDeviceUserNumber = async (
  userNumber: bigint | null
): Promise<void> => {
  const container = document.getElementById("pageContent") as HTMLElement;
  render(pageContent(userNumber), container);
  return init();
};

const init = () => {
  const addDeviceUserNumberContinue = document.getElementById(
    "addDeviceUserNumberContinue"
  ) as HTMLButtonElement;
  const userNumberInput = document.getElementById(
    "addDeviceUserNumber"
  ) as HTMLInputElement;

  userNumberInput.onkeypress = (e) => {
    // submit if user hits enter
    if (e.key === "Enter") {
      e.preventDefault();
      addDeviceUserNumberContinue.click();
    }
  };

  addDeviceUserNumberContinue.onclick = async () => {
    let loginInterval: number;

    const userNumber = parseUserNumber(userNumberInput.value);
    if (userNumber !== null) {
      userNumberInput.classList.toggle("errored", false);
      let existingDevices: DeviceData[];
      try {
        existingDevices = await IIConnection.lookupAll(userNumber);
      } catch (err) {
        console.log(
          "Failed to fetch devices for Identity Anchor:",
          userNumber,
          err
        );
        existingDevices = [];
      }
      let identity: WebAuthnIdentity;
      try {
        identity = await WebAuthnIdentity.create({
          publicKey: creationOptions(existingDevices),
        });
      } catch (error: unknown) {
        let message: string;
        if (error instanceof Error && "message" in error) {
          message = error.message;
        } else {
          message = unknownToString(error, "Unknown error");
        }
        await displayError({
          title: "Failed to authenticate",
          message:
            "We failed to collect the necessary information from your security device.",
          detail: message,
          primaryButton: "Try again",
        });
        return addDeviceUserNumber(userNumber);
      }
      const publicKey = identity.getPublicKey().toDer();
      const rawId = toHexString(identity.rawId);

      const url = new URL(location.toString());
      url.pathname = "/";
      url.hash = `#device=${userNumber};${toHexString(publicKey)};${rawId}`;
      const link = encodeURI(url.toString());

      displayAddDeviceLink(link);
      loginInterval = window.setInterval(async () => {
        console.log("checking if authenticated");
        try {
          const devices = await IIConnection.lookupAuthenticators(userNumber);
          const matchedDevice = devices.find((deviceData) =>
            bufferEqual(new Uint8Array(deviceData.pubkey).buffer, publicKey)
          );
          if (matchedDevice !== undefined) {
            window.clearInterval(loginInterval);
            setUserNumber(userNumber);
            window.location.reload();
          }
        } catch (error) {
          console.error(error);
        }
      }, 2500);
    } else {
      userNumberInput.classList.toggle("errored", true);
      userNumberInput.placeholder = "Please enter your Identity Anchor first";
    }
  };
};
