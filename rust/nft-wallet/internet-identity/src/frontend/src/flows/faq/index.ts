import { html, render } from "lit-html";

import { questionsArray } from "./questions";
import type { Question, Link } from "./questions";

// re-export for ease of use
export { questions } from "./questions";

// The rendered (list item) question
function renderQuestion(faq: Question) {
  return html`<li
    class="faq__question"
  >
    <details
    id=${faq.anchor} >
    <summary class="faq__question-summary">
      ${faq.question}
    <div class="faq__question-underline"></div>
    </summary>
    <div>
      <p class="faq__answer">${faq.answer}</p>
      ${faq.links.length > 0 ? renderFaqLinks(faq.links) : ""}
    </div>
  </li>`;
}

function renderFaqLinks(links: Link[]) {
  return html` <ul class="faq__answer-links">
    ${Object.values(links)
      .sort((a, b) => {
        return a.link < b.link ? -1 : 1;
      })
      .map(
        (link) =>
          html`<li>
            &middot;
            <a
              class="faq__answer-link"
              rel="noopener noreferrer"
              href="${link.link}"
              target="_blank"
              >${link.name} &#8599;</a
            >
          </li>`
      )}
  </ul>`;
}

// The FAQ page
const pageContent = html`
  <style>
    /* briefly flash the question when redirected to a particular question */
    @keyframes flash-question {
      0% {
        background-color: transparent;
      }
      50% {
        background-color: var(--rainbow-orange);
        border-radius: 0.3em;
      }
      100% {
        background-color: transparent;
      }
    }
    :target {
      animation-name: flash-question;
      animation-duration: 600ms;
    }
  </style>
  <div class="faq__container">
    <h1 class="faq__title">FAQ</h1>
    <ul class="faq__questions">
      ${questionsArray.map((faq) => renderQuestion(faq))}
    </ul>
  </div>
`;

// Open the anchor with id="foo" if the page hash is "#foo"
const openAnchor = (): void => {
  const hash = location.hash.substring(1);

  if (hash !== "") {
    const details = document.getElementById(hash);
    console.log(details);

    if (details) {
      details.setAttribute("open", "");
    }
  }
};

export const faqView = (): void => {
  document.title = "FAQ | Internet Identity";
  const container = document.getElementById("pageContent") as HTMLElement;
  render(pageContent, container);
  openAnchor(); // needs to happen after DOM was rendered
};
