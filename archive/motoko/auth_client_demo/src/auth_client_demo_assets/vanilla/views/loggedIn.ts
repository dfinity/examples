import { ActorSubclass } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import { html, render } from "lit-html";
import { renderIndex } from ".";
import { _SERVICE } from "../../../declarations/whoami/whoami.did";

const content = () => html`<div class="container">
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
</div>`;

export const renderLoggedIn = (
  actor: ActorSubclass<_SERVICE>,
  authClient: AuthClient
) => {
  render(content(), document.getElementById("pageContent") as HTMLElement);

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
      await authClient.logout();
      renderIndex(authClient);
    };
};
