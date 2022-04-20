import { html, render } from "lit-html";
import { compatibilityChart } from "../components/compatibilityChart";

// Taken from: https://caniuse.com/?search=PublicKeyCredential
const pageContent = html`
  <style>
    ul {
      list-style-type: none;
    }
  </style>
  <div class="container" id="about">
    <h1>About Internet Identity</h1>
    <p>
      Internet Identity is the identity provider for the Internet Computer: A
      dapp facilitating authentication on the Internet Computer.
    </p>
    <p>
      Internet Identity protects your privacy. When you create an Identity
      Anchor, you will not be asked to provide any personal information.
      Internet Identity will create a different, random pseudonym for each dapp
      you use with the same Identity Anchor and Identity Anchors are hidden from
      dapps.
    </p>
    <p>
      Internet Identity works without passwords! You can use your devices with
      their associated authentication methods.
    </p>
    <p>
      If you have questions about how to get started with Internet Identity, you
      can consult the
      <a href="http://sdk.dfinity.org/docs/ic-identity-guide/auth-how-to.html"
        >Internet Identity guide</a
      >
      for step-by-step directions.
    </p>
    <h2>Compatibility</h2>
    ${compatibilityChart}
  </div>
`;

export const aboutView = (): void => {
  const container = document.getElementById("pageContent") as HTMLElement;
  render(pageContent, container);
};
