import { html, render } from "lit-html";
import { Page, PageOptions } from "../types";

const content = (homeUrl: string) => html`<div class="container">
  <h2>About</h2>
  <p>
  This dapp is one of the examples available on possible integrations with the internet computer, 
  for more examples check the <a href="https://github.com/dfinity/examples">examples repository</a>.
  </p>
  <br /><br />
  <a href="${homeUrl}">home</a>
</div>`;

export class AboutPage implements Page {
  async render({ auth, router }: PageOptions): Promise<void> {
    const homeUrl = new URL(window.location.href);
    homeUrl.searchParams.set("route", "home");

    render(content(homeUrl.toString()), document.getElementById("pageContent") as HTMLElement);
  }
}
