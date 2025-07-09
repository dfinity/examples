import { Identity } from "@dfinity/agent";
import { html, render } from "lit-html";
import { canisterId, createActor } from "../../../declarations/ios_notifications_api";
import { Page, PageOptions } from "../types";
import { RouteName } from ".";

const content = (aboutUrl: string) => html`<div class="container">
  <style>
    #whoami {
      border: 1px solid #1a1a1a;
      margin-bottom: 1rem;
    }
  </style>
  <h1>Internet Identity Client</h1>
  <h2>You are authenticated!</h2>
  <p>To see how a canister views you, click this button!</p>
  <button type="button" id="whoamiButton" class="primary">Who am I?</button>
  <input type="text" readonly id="whoami" placeholder="your Identity" />
  <button id="logout">log out</button>

  <br /><br />
  <a href="${aboutUrl}">about</a>
</div>`;

export class HomePage implements Page {
  async render({ auth, router }: PageOptions): Promise<void> {
    const aboutUrl = new URL(window.location.href);
    aboutUrl.searchParams.set("route", "about");

    render(content(aboutUrl.toString()), document.getElementById("pageContent") as HTMLElement);

    const identity = (await auth.client().getIdentity()) as unknown as Identity;

    const actor = createActor(canisterId as string, {
      agentOptions: {
        identity,
      },
    });

    (document.getElementById("whoamiButton") as HTMLButtonElement).onclick = 
      async () => {
        try {
          const response = await actor.whoami();
          (document.getElementById("whoami") as HTMLInputElement).value =
            response.toString();
        } catch (error) {
          console.error(error);
        }
      };

    (document.getElementById("logout") as HTMLButtonElement).onclick =
      async () => {
        await auth.client().logout();
        router.renderPage(RouteName.login, { auth, router });
      };
  }
}
