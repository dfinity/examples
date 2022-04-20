import { render, html } from "lit-html";
import { IIConnection } from "../../utils/iiConnection";
import { parseUserNumber, setUserNumber } from "../../utils/userNumber";
import { withLoader } from "../../components/loader";
import { register } from "../register";
import { icLogo } from "../../components/icons";
import { addDeviceUserNumber } from "../addDeviceUserNumber";
import { navbar } from "../../components/navbar";
import { footer } from "../../components/footer";
import { UserIntent, authenticateUnknownIntent } from "../../utils/userIntent";
import { useRecovery } from "../recovery/useRecovery";
import { registerDisabled } from "../registerDisabled";
import { apiResultToLoginFlowResult, LoginFlowResult } from "./flowResult";

const pageContent = (userIntent: UserIntent) => html` <style>
    #registerUserNumber:focus {
      box-sizing: border-box;
      border-style: double;
      border-width: 2px;
      border-radius: 4px;
      border-image-slice: 1;
      outline: none;
      border-image-source: linear-gradient(
        270.05deg,
        #29abe2 10.78%,
        #522785 22.2%,
        #ed1e79 42.46%,
        #f15a24 59.41%,
        #fbb03b 77.09%
      );
    }

    #registerSection {
      margin-top: 4rem;
    }

    .spacer {
      height: 2rem;
    }
  </style>
  <div class="container">
    ${icLogo}
    <h2 id="loginWelcome">Welcome to<br />Internet Identity</h2>
    <p>
      Provide an Identity Anchor to
      authenticate${authenticateUnknownIntent(userIntent)}.
    </p>
    <input
      type="text"
      id="registerUserNumber"
      placeholder="Enter Identity Anchor"
    />
    <button type="button" id="loginButton" class="primary">Authenticate</button>
    ${userIntent.kind === "addDevice"
      ? html`<div class="spacer"></div>`
      : html`<div class="textLink" id="registerSection">
            New?
            <button id="registerButton" class="linkStyle">
              Create an Internet Identity Anchor.
            </button>
          </div>
          <div class="textLink">
            Already have an anchor
            <button id="addNewDeviceButton" class="linkStyle">
              but using a new device?
            </button>
          </div>
          <div class="textLink">
            Lost access
            <button id="recoverButton" class="linkStyle">
              and want to recover?
            </button>
          </div>`}
    ${navbar}
  </div>
  ${footer}`;

export const loginUnknownAnchor = async (
  userIntent: UserIntent
): Promise<LoginFlowResult> => {
  const container = document.getElementById("pageContent") as HTMLElement;
  render(pageContent(userIntent), container);
  return new Promise((resolve, reject) => {
    initLogin(resolve);
    if (userIntent.kind !== "addDevice") {
      initLinkDevice();
      initRegister(resolve, reject);
      initRecovery();
    }
  });
};

/** Check that the current origin is not the explicit canister id or a raw url.
 *  Explanation why we need to do this:
 *  https://forum.dfinity.org/t/internet-identity-deprecation-of-account-creation-on-all-origins-other-than-https-identity-ic0-app/9694
 **/
function isRegistrationAllowed() {
  return !/(^https:\/\/rdmx6-jaaaa-aaaaa-aaadq-cai\.ic0\.app$)|(.+\.raw\..+)/.test(
    window.origin
  );
}

const initRegister = (
  resolve: (res: LoginFlowResult) => void,
  reject: (err: Error) => void
) => {
  const registerButton = document.getElementById(
    "registerButton"
  ) as HTMLButtonElement;
  registerButton.onclick = () => {
    const result = isRegistrationAllowed() ? register() : registerDisabled();
    result
      .then((res) => {
        if (res === null) {
          window.location.reload();
        } else {
          resolve(res);
        }
      })
      .catch(reject);
  };
};

const initRecovery = () => {
  const recoverButton = document.getElementById(
    "recoverButton"
  ) as HTMLButtonElement;
  recoverButton.onclick = () => useRecovery();
};

const initLogin = (resolve: (res: LoginFlowResult) => void) => {
  const userNumberInput = document.getElementById(
    "registerUserNumber"
  ) as HTMLInputElement;
  const loginButton = document.getElementById(
    "loginButton"
  ) as HTMLButtonElement;

  userNumberInput.onkeypress = (e) => {
    // submit if user hits enter
    if (e.key === "Enter") {
      e.preventDefault();
      loginButton.click();
    }
  };

  loginButton.onclick = async () => {
    const userNumber = parseUserNumber(userNumberInput.value);
    if (userNumber === null) {
      return resolve({
        tag: "err",
        title: "Please enter a valid Identity Anchor",
        message: `${userNumber} doesn't parse as a number`,
      });
    }
    const result = await withLoader(() => IIConnection.login(userNumber));
    if (result.kind === "loginSuccess") {
      setUserNumber(userNumber);
    }
    resolve(apiResultToLoginFlowResult(result));
  };
};

const initLinkDevice = () => {
  const addNewDeviceButton = document.getElementById(
    "addNewDeviceButton"
  ) as HTMLButtonElement;

  addNewDeviceButton.onclick = () => {
    const userNumberInput = document.getElementById(
      "registerUserNumber"
    ) as HTMLInputElement;

    const userNumber = parseUserNumber(userNumberInput.value);
    addDeviceUserNumber(userNumber);
  };
};
