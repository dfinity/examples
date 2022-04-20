import ClipboardJS from "clipboard";
import { html, render } from "lit-html";
import { checkmarkIcon, warningIcon } from "../../components/icons";

const pageContent = (seedPhrase: string) => html`
  <style>
    #seedPhrase {
      font-size: 1rem;
    }
  </style>
  <div class="container">
    <h1>Seedphrase</h1>
    <p>Your seed phrase makes it easy to recover this Identity Anchor.</p>
    <div class="warningBox">
      <span class="warningIcon">${warningIcon}</span>
      <div class="warningMessage">
        Do <b>NOT</b> forget to save this seed phrase. Save a backup on a
        storage medium and write it down.<br />
        Keep it secret &mdash; knowledge of the seed phrase will enable access
        to this Identity Anchor!
      </div>
    </div>
    <label>Your seed phrase</label>
    <div id="seedPhrase" translate="no" class="highlightBox">${seedPhrase}</div>
    <button id="seedCopy" data-clipboard-target="#seedPhrase">Copy</button>
    <button id="displaySeedPhraseContinue" class="primary hidden">
      Continue
    </button>
  </div>
`;

export const displaySeedPhrase = async (seedPhrase: string): Promise<void> => {
  const container = document.getElementById("pageContent") as HTMLElement;
  render(pageContent(seedPhrase), container);
  return init();
};

const init = (): Promise<void> =>
  new Promise((resolve) => {
    const displaySeedPhraseContinue = document.getElementById(
      "displaySeedPhraseContinue"
    ) as HTMLButtonElement;
    displaySeedPhraseContinue.onclick = () => resolve();

    const seedCopy = document.getElementById("seedCopy") as HTMLButtonElement;
    new ClipboardJS(seedCopy).on("success", () => {
      const seedCopy = document.getElementById("seedCopy") as HTMLButtonElement;
      displaySeedPhraseContinue.classList.toggle("hidden", false);
      render(checkmarkIcon, seedCopy);
    });
  });
