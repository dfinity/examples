import { AuthClient } from "@dfinity/auth-client";
import { iiUrl, getWebApp, redirectToDashboard } from './common.js';

window.onload = async () => {
  const webapp = await getWebApp();
  // Call whoami which returns the principal (user id) of the current user.
  const whoAmI = await webapp.whoami();
  // If the user is already authenticated, then he is redirected to the dashboard webpage.
  if (!whoAmI.isAnonymous()) {
      redirectToDashboard();
  }
};

// When the user clicks the "Login" button, we start the login process.
document.getElementById("loginBtn").addEventListener("click", async () => {
  // First we have to create an AuthClient.
  const authClient = await AuthClient.create();

  // Call authClient.login(...) to login with Internet Identity. This will open a new tab
  // with the login prompt. The code has to wait for the login process to complete.
  // We can either use the callback functions directly or wrap in a promise.
  await new Promise((resolve, reject) => {
    authClient.login({
      identityProvider: iiUrl,
      onSuccess: resolve,
      onError: reject,
    });
  });

  // Once authenticated, the user is redirected to the dashboard webpage.
  redirectToDashboard();
});
