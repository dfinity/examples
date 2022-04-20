import { html, render } from "lit-html";
import { DeviceData } from "../../../generated/internet_identity_types";
import { securityKeyIcon, seedPhraseIcon } from "../../components/icons";

const pageContent = (devices: DeviceData[]) => html`
  <style>
    #skipRecovery {
      margin-top: 3.5rem;
      font-weight: 600;
      font-size: 1rem;
    }
    .recoveryContainer {
      display: flex;
      gap: 1rem;
      margin-top: 1rem;
    }
    .recoveryOption {
      display: flex;
      flex-direction: column;
      align-items: center;
      border: 1px solid gray;
      border-radius: 4px;
      width: 100%;
      padding: 1rem;
      font-family: "Montserrat", sans-serif;
      font-size: 1.2rem;
      margin-bottom: 2rem;
    }
    .recoveryOption:disabled div {
      color: gray;
    }
    .recoveryOption:disabled svg {
      filter: invert(93%) sepia(0%) saturate(33%) hue-rotate(255deg)
        brightness(94%) contrast(79%);
    }
    .recoveryOption:disabled:hover,
    .recoveryOption:disabled:focus {
      outline: none;
      box-shadow: none;
    }
    .recoveryIcon {
      height: 52px;
    }
    .recoveryTitle {
      font-weight: 500;
      margin: 0.5rem;
    }
    .recoveryDescription {
      text-align: center;
      font-size: 1rem;
    }
  </style>
  <div class="container">
    <h1>Recovery Mechanism</h1>
    <p>
      You can use a recovery mechanism to recover your anchor if your other
      device(s) are lost. We recommend adding one as a seed phrase or portable
      backup now.
    </p>
    <div class="recoveryContainer">
      <button
        ?disabled=${hasRecoveryPhrase(devices)}
        class="recoveryOption"
        id="seedPhrase"
      >
        <span class="recoveryIcon">${seedPhraseIcon}</span>
        <div class="recoveryTitle">Seed Phrase</div>
        <div class="recoveryDescription">Use your own storage</div>
      </button>
      <button
        ?disabled=${hasRecoveryKey(devices)}
        class="recoveryOption"
        id="securityKey"
      >
        <span class="recoveryIcon">${securityKeyIcon}</span>
        <div class="recoveryTitle">Security Key</div>
        <div class="recoveryDescription">Use an extra security key</div>
      </button>
    </div>
    <button id="skipRecovery" class="linkStyle">Add recovery later</button>
  </div>
`;

export type RecoveryMechanism = "securityKey" | "seedPhrase";

export const chooseRecoveryMechanism = async (
  devices: DeviceData[]
): Promise<RecoveryMechanism | null> => {
  const container = document.getElementById("pageContent") as HTMLElement;
  render(pageContent(devices), container);
  return init();
};

const init = (): Promise<RecoveryMechanism | null> =>
  new Promise((resolve) => {
    const securityKey = document.getElementById(
      "securityKey"
    ) as HTMLButtonElement;
    const seedPhrase = document.getElementById(
      "seedPhrase"
    ) as HTMLButtonElement;
    const skipRecovery = document.getElementById(
      "skipRecovery"
    ) as HTMLButtonElement;
    securityKey.onclick = () => resolve("securityKey");
    seedPhrase.onclick = () => resolve("seedPhrase");
    skipRecovery.onclick = () => resolve(null);
  });

const hasRecoveryPhrase = (devices: DeviceData[]): boolean =>
  devices.some((device) => device.alias === "Recovery phrase");
const hasRecoveryKey = (devices: DeviceData[]): boolean =>
  devices.some((device) => device.alias === "Recovery key");
