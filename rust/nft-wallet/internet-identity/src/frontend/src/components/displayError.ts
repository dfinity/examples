import { html, render, TemplateResult } from "lit-html";
import { warningIcon } from "./icons";

export type ErrorOptions = {
  title: string;
  message: string | TemplateResult;
  detail?: string;
  primaryButton: string;
};

const pageContent = (options: ErrorOptions) => html`
  <style>
    #errorContainer {
      min-height: 15rem;
    }
    .displayErrorDetail {
      overflow: auto;
      margin-bottom: 2rem;
      flex-grow: 1;
    }
    .displayErrorMessage {
      flex-grow: 1;
    }
    .warningIcon {
      align-self: center;
      width: 3rem;
      height: 3rem;
      margin-bottom: 1.5rem;
    }
  </style>
  <div id="errorContainer" class="container">
    ${warningIcon}
    <h1>${options.title}</h1>
    <p class="displayErrorMessage">${options.message}</p>
    ${options.detail !== undefined
      ? html` <details class="displayErrorDetail">
          <summary>Error details</summary>
          <pre>${options.detail}</pre>
        </details>`
      : ""}
    <button id="displayErrorPrimary" class="primary">
      ${options.primaryButton}
    </button>
  </div>
`;

export const displayError = async (options: ErrorOptions): Promise<void> => {
  const container = document.getElementById("pageContent") as HTMLElement;
  render(pageContent(options), container);
  return init();
};

const init = (): Promise<void> =>
  new Promise((resolve) => {
    const displayErrorPrimary = document.getElementById(
      "displayErrorPrimary"
    ) as HTMLButtonElement;
    displayErrorPrimary.onclick = () => resolve();
  });
