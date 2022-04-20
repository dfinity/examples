import { html } from "lit-html";
import { aboutLink } from "../components/aboutLink";
import { faqLink } from "../components/faqLink";

export const navbar = html`<nav>
  <div class="nav-links">${aboutLink} &middot; ${faqLink}</div>
</nav>`;
