import { html, render } from "lit-html";
import { RouteName } from ".";
import { AuthLoginType, MultiPlatformLoggedInAction } from "../auth";
import { Page, PageOptions } from "../types";

enum LoginPlatform {
  Desktop,
  Mobile,
}

const content = (platform: LoginPlatform) => {
  switch (platform) {
    case LoginPlatform.Mobile:
      return html`<div class="container">
        <button type="button" id="loginButton">continue</button>
      </div>`;
    default:
      return html`<div class="container">
        <h1>Internet Identity Client</h1>
        <h2>You are not authenticated</h2>
        <p>To log in, click this button!</p>
        <button type="button" id="loginButton">Log in</button>
      </div>`;
  }
};

export class LoginPage implements Page {
  async render({ auth, router }: PageOptions): Promise<void> {
    let platform = LoginPlatform.Desktop;
    if (auth.loginType() === AuthLoginType.Mobile) {
      platform = LoginPlatform.Mobile;
    }

    render(
      content(platform),
      document.getElementById("pageContent") as HTMLElement
    );

    (document.getElementById("loginButton") as HTMLButtonElement).onclick =
      async () => {
        auth.login(async () => {
          const action = await auth.handleMultiPlatformLogin();
          if (action === MultiPlatformLoggedInAction.Redirecting) {
            return;
          }

          router.renderPage(RouteName.home, { auth, router });
        });
      };
  }
}
