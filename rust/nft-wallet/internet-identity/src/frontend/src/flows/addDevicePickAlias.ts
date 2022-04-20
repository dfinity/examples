import { html, render } from "lit-html";
import { initLogout, logoutSection } from "../components/logout";

const pageContent = () => html`
  <div class="container">
    <h1>New Device</h1>
    <p>Please provide a name for your new device</p>
    <input id="deviceAlias" placeholder="Device alias" />
    <button id="deviceAliasContinue" class="primary">Add Device</button>
    <button id="deviceAliasCancel">Cancel</button>
    ${logoutSection()}
  </div>
`;

export const pickDeviceAlias = async (): Promise<string | null> => {
  const container = document.getElementById("pageContent") as HTMLElement;
  render(pageContent(), container);
  return init();
};

const init = (): Promise<string | null> =>
  new Promise((resolve) => {
    initLogout();
    const deviceAlias = document.getElementById(
      "deviceAlias"
    ) as HTMLInputElement;
    const deviceAliasContinue = document.getElementById(
      "deviceAliasContinue"
    ) as HTMLButtonElement;
    const deviceAliasCancel = document.getElementById(
      "deviceAliasCancel"
    ) as HTMLButtonElement;
    deviceAliasCancel.onclick = () => {
      resolve(null);
    };
    deviceAliasContinue.onclick = () => {
      resolve(deviceAlias.value);
    };
  });
