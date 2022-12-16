import { html, render } from "lit-html";
import { Page, PageOptions } from "../types";

const content = (homeUrl: string) => html`<div class="container">
  <h2>You are in the about page!</h2>
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
