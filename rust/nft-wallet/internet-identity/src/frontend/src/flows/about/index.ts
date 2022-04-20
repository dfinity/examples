import { html, render } from "lit-html";
import { compatibilityData } from "../../components/compatibilityChart";

// The About page
const pageContent = html`
  <div class="about__container">
    <h1 class="about__title">About</h1>
    <div class="about__section">
      <h2 class="about__subtitle">Internet Identity</h2>
      <div class="about__section-title-underline"></div>
      <p>
        Internet Identity is the identity provider for the Internet Computer: A
        dapp facilitating authentication on the Internet Computer.
        <br />
        <br />

        For frequently asked questions, check the
        <a href="/faq" title="Go to the Internet Identity FAQ page">FAQ page</a
        >.
      </p>
    </div>
    <div class="about__section">
      <h2 class="about__subtitle">Compatibility</h2>
      <div class="about__section-title-underline"></div>
      <p>${compatibilityData.note}</p>

      <div class="about__compatibility-flexparent">
        <div class="about__compatibility-flexchild">
          <h3 class="about__compatibility-type">For Desktop</h3>
          <ul class="about__compatibility-list">
            ${compatibilityData.desktop.map(
              (browser) =>
                html`<li class="about__compatibility-list-item">${browser}</li>`
            )}
          </ul>
        </div>
        <div class="about__compatibility-flexchild">
          <h3 class="about__compatibility-type">For Mobile</h3>
          <ul class="about__compatibility-list">
            ${compatibilityData.mobile.map(
              (browser) =>
                html`<li class="about__compatibility-list-item">${browser}</li>`
            )}
          </ul>
        </div>
      </div>
    </div>
  </div>
`;

export const aboutView = (): void => {
  document.title = "About | Internet Identity";
  const container = document.getElementById("pageContent") as HTMLElement;
  render(pageContent, container);
};
