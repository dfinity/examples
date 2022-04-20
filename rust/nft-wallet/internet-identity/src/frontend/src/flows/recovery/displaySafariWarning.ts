import { html, render } from "lit-html";
import { warningIcon } from "../../components/icons";
import { setupRecovery } from "./setupRecovery";
import { IIConnection } from "../../utils/iiConnection";

const pageContent = () => html`
  <style>
    #warningContainer {
      min-height: 15rem;
    }
    .warningIcon {
      align-self: center;
      width: 3rem;
      height: 3rem;
      margin-bottom: 1.5rem;
    }
    #warningHeading {
      text-align: center;
    }
    #warningContainer p {
      font-size: 1.2rem;
    }
    #warningContainer a {
      margin-bottom: 1rem;
    }
    .spacer {
      min-height: 48px;
    }
  </style>
  <div id="warningContainer" class="container">
    ${warningIcon}
    <h1 id="warningHeading">Warning</h1>
    <p>It looks like you are using Safari or iOS.</p>
    <p>
      If you “Clear History and Website Data” via system preferences, all web
      authentication keys will be <em>deleted</em> from this device. This means
      that you will no longer have access to your identity anchor (and all
      associated resources and tokens) with this device.
    </p>
    <p>
      As a best practice, we recommend you assign multiple devices to an
      Identity Anchor and add at least one recovery mechanism such as an
      external key fob or a seedphrase
    </p>
    <button id="displayWarningAddRecovery" class="primary">
      Add a recovery mechanism to an Identity Anchor
    </button>
    <button id="displayWarningRemindLater" class="primary">
      Skip, I understand the risks
    </button>
    <div class="spacer"></div>
  </div>
`;

export const displaySafariWarning = async (
  userNumber: bigint,
  connection: IIConnection
): Promise<void> => {
  const container = document.getElementById("pageContent") as HTMLElement;
  render(pageContent(), container);
  return init(userNumber, connection);
};

const init = (userNumber: bigint, connection: IIConnection): Promise<void> =>
  new Promise((resolve) => {
    const displayWarningAddRecovery = document.getElementById(
      "displayWarningAddRecovery"
    ) as HTMLButtonElement;
    displayWarningAddRecovery.onclick = () => {
      setupRecovery(userNumber, connection).then(() => resolve());
    };
    const displayWarningRemindLater = document.getElementById(
      "displayWarningRemindLater"
    ) as HTMLButtonElement;
    displayWarningRemindLater.onclick = () => {
      resolve();
    };
  });
