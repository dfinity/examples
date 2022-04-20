import { html, render } from "lit-html";
import { FrontendHostname } from "../../generated/internet_identity_types";
import { questions } from "./faq/questions";

const pageContent = (hostName: string, principal: string) => html`
  <style>
    #confirmRedirectHostname {
      font-size: 0.875rem;
      font-weight: 400;
    }

    /* The tooltip */
    .tooltip {
      position: relative;
      display: inline-block;
      /* fake a dotted underline that works across browsers */
      border-bottom: 1px dotted black;
    }
    /* The actual tooltip text */
    .tooltiptext {
      visibility: hidden;
      width: 200px;
      background-color: var(--grey-500);
      color: #fff;
      text-align: center;
      padding: 0.9rem;
      border-radius: 6px;
      font-size: 0.7rem;
      position: absolute;
      z-index: 1;
    }
    .tooltip:hover .tooltiptext {
      visibility: visible;
    }

    #confirmRedirectPrincipal {
      background-color: transparent;
    }

    #confirmRedirectPrincipal > * {
      font-size: 0.7rem;
      font-weight: 400;
      color: var(--grey-500);
    }
  </style>
  <div class="container">
    <h1>Authorize Authentication</h1>
    <p>Proceed to authenticate with:</p>
    <div id="confirmRedirectHostname" class="highlightBox">${hostName}</div>
    <button id="confirmRedirect" class="primary">Proceed</button>
    <button id="cancelRedirect">Cancel</button>
    <div id="confirmRedirectPrincipal" class="highlightBox">
      <a href="/faq#${questions.shareIIAnchor.anchor}" target="_blank">
        Application-specific</a
      >
      <span> ID for ${hostName}:</span>
      <br />
      <p>${principal}</p>
    </div>
  </div>
`;

export const confirmRedirect = async (
  hostName: FrontendHostname,
  userPrincipal: string
): Promise<boolean> => {
  const container = document.getElementById("pageContent") as HTMLElement;
  render(pageContent(hostName, userPrincipal), container);
  return init();
};

const init = (): Promise<boolean> =>
  new Promise((resolve) => {
    const confirmRedirect = document.getElementById(
      "confirmRedirect"
    ) as HTMLButtonElement;
    const cancelRedirect = document.getElementById(
      "cancelRedirect"
    ) as HTMLButtonElement;
    confirmRedirect.onclick = () => resolve(true);
    cancelRedirect.onclick = () => resolve(false);
  });
