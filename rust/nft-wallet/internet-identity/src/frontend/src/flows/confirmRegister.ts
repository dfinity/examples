import { html, render } from "lit-html";
import { displayUserNumber } from "./displayUserNumber";
import { displayError } from "../components/displayError";
import { setUserNumber } from "../utils/userNumber";
import {
  apiResultToLoginFlowResult,
  LoginFlowResult,
} from "./login/flowResult";
import { Challenge } from "../../generated/internet_identity_types";
import { WebAuthnIdentity } from "@dfinity/identity";
import getProofOfWork from "../crypto/pow";
import { Principal } from "@dfinity/principal";
import { withLoader } from "../components/loader";
import {
  IIConnection,
  canisterIdPrincipal,
  ChallengeResult,
} from "../utils/iiConnection";

const pageContent = html`
  <div class="container">
    <h1>Confirm new device</h1>
    <form id="confirmForm">
      <p class="captcha-status-text">…</p>
      <img id="captchaImg" />
      <input id="captchaInput" />
      <p class="confirm-paragraph">Please confirm to add your device.</p>
      <button type="submit" class="primary" id="confirmRegisterButton" disabled>
        Confirm
      </button>
      <button type="button" id="cancelButton">Cancel</button>
    </form>
  </div>
`;

export const confirmRegister = (
  captcha: Promise<Challenge>,
  identity: WebAuthnIdentity,
  alias: string
): Promise<LoginFlowResult | null> => {
  const container = document.getElementById("pageContent") as HTMLElement;
  render(pageContent, container);
  return init(canisterIdPrincipal, identity, alias, captcha);
};

const tryRegister = (
  identity: WebAuthnIdentity,
  alias: string,
  challengeResult: ChallengeResult,
  func: (result: LoginFlowResult) => void
) => {
  withLoader(async () => {
    return IIConnection.register(identity, alias, challengeResult);
  }).then((result) => {
    if (result.kind == "loginSuccess") {
      // Write user number to storage
      setUserNumber(result.userNumber);

      // Congratulate user
      displayUserNumber(result.userNumber).then(() => {
        func(apiResultToLoginFlowResult(result));
      });
    } else if (result.kind == "badChallenge") {
      const confirmParagraph = document.querySelector(
        ".confirm-paragraph"
      ) as HTMLElement;
      confirmParagraph.innerHTML =
        "The value you entered is incorrect. A new challenge is generated.";
      requestCaptcha();
    } else if (result.kind == "registerNoSpace") {
      // Currently, if something goes wrong we only tell the user that
      // something went wrong and then reload the page.
      displayError({
        title: "No Space Left",
        message:
          "We could not create an identity anchor because Internet Identity is at maximum capacity. Click 'ok' to reload.",
        primaryButton: "Ok",
      }).then(() => {
        window.location.reload();
      });
    } else {
      displayError({
        title: "Something went wrong",
        message:
          "We could not create an identity anchor. You will find the full error message below. Click 'ok' to reload.",
        primaryButton: "Ok",
        detail: JSON.stringify(
          result.error,
          Object.getOwnPropertyNames(result.error)
        ),
      }).then(() => {
        window.location.reload();
      });
    }
  });
};

// Request a captcha and, when received, update the DOM elements accordingly.
const requestCaptcha = (captcha?: Promise<Challenge>): void => {
  const form = document.getElementById("confirmForm") as HTMLFormElement;
  const captchaStatusText = document.querySelector(
    ".captcha-status-text"
  ) as HTMLElement;
  captchaStatusText.innerHTML = "Creating CAPTCHA challenge…";

  const captchaInput = document.querySelector(
    "#captchaInput"
  ) as HTMLFormElement;
  captchaInput.disabled = true;

  const confirmRegisterButton = form.querySelector(
    "#confirmRegisterButton"
  ) as HTMLFormElement;
  confirmRegisterButton.disabled = true;

  captcha = captcha || makeCaptcha();

  captcha.then((captchaResp) => {
    const captchaImg = document.querySelector("#captchaImg");
    if (captchaImg) {
      captchaImg.setAttribute(
        "src",
        `data:image/png;base64, ${captchaResp.png_base64}`
      );
      confirmRegisterButton.setAttribute(
        "data-captcha-key",
        `${captchaResp.challenge_key}`
      );
      captchaStatusText.innerHTML = "Please type in the characters you see.";
      confirmRegisterButton.disabled = false;
      captchaInput.disabled = false;
      captchaInput.value = "";
    }
  });
};

// This computes a PoW and requests a challenge from the II backend.
// NOTE: The Proof-of-Work is computed in one go (one run of the event loop) so
// nothing else will happen during that time. Better have a loading screen
// shown to the user, or have all buttons disabled, because no other javascript
// will run for a few seconds.
export const makeCaptcha = (): Promise<Challenge> =>
  new Promise((resolve) => {
    setTimeout(() => {
      const now_in_ns = BigInt(Date.now()) * BigInt(1000000);
      const pow = getProofOfWork(now_in_ns, canisterIdPrincipal);
      IIConnection.createChallenge(pow).then((cha) => {
        resolve(cha);
      });
    });
  });

const init = (
  canisterIdPrincipal: Principal,
  identity: WebAuthnIdentity,
  alias: string,
  captcha: Promise<Challenge>
): Promise<LoginFlowResult | null> => {
  requestCaptcha(captcha);

  // since the index expects to regain control we unfortunately have to wrap
  // this whole logic in a promise that then resolves (giving control back to
  // the caller)
  return new Promise((resolve) => {
    const confirmRegisterButton = document.querySelector(
      "#confirmRegisterButton"
    ) as HTMLFormElement;
    const captchaInput = document.querySelector(
      "#captchaInput"
    ) as HTMLFormElement;
    const cancelButton = document.querySelector(
      "#cancelButton"
    ) as HTMLButtonElement;

    cancelButton.onclick = () => {
      resolve(null);
    };

    confirmRegisterButton.onclick = (e) => {
      e.preventDefault();
      e.stopPropagation();

      const captchaStatusText = document.querySelector(
        ".captcha-status-text"
      ) as HTMLElement;
      captchaStatusText.innerHTML = "Checking CAPTCHA challenge…";
      confirmRegisterButton.disabled = true;

      const captchaChars = captchaInput.value;
      const captchaKey = confirmRegisterButton.dataset.captchaKey;

      if (captchaKey === undefined) {
        console.log("Something went wrong: no captcha key found");
        requestCaptcha();
        return;
      }

      const challengeResult: ChallengeResult = {
        key: captchaKey,
        chars: captchaChars,
      };

      tryRegister(identity, alias, challengeResult, resolve);
    };
  });
};
