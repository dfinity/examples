import { html, render } from "lit-html";
import { initLogout, logoutSection } from "../components/logout";
import { IIConnection } from "../utils/iiConnection";
import { renderManage } from "./manage";

const pageContent = (name: string) => html`
  <div class="container">
    <h1>Success!</h1>
    <p>You have successfully added your new device.</p>
    <label>Device name:</label>
    <div class="highlightBox">${name}</div>
    <button id="manageDevicesButton" class="primary">Manage devices</button>
    ${logoutSection()}
  </div>
`;

export const successfullyAddedDevice = async (
  name: string,
  userNumber: bigint,
  connection: IIConnection
): Promise<void> => {
  const container = document.getElementById("pageContent") as HTMLElement;
  render(pageContent(name), container);
  initLogout();
  init(userNumber, connection);
};

const init = async (userNumber: bigint, connection: IIConnection) => {
  const manageDevicesButton = document.getElementById(
    "manageDevicesButton"
  ) as HTMLButtonElement;
  manageDevicesButton.onclick = () => renderManage(userNumber, connection);
};
