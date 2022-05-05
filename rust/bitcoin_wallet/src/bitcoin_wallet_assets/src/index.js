import { AuthClient } from "@dfinity/auth-client";
import { iiUrl, getWebApp, redirectToDashboard } from './common.js';

window.onload = async () => {
  const webapp = await getWebApp();
  // Call whoami which returns the principal (user id) of the current user.
  const whoAmI = await webapp.whoami();
  if (!whoAmI.isAnonymous()) {
      redirectToDashboard();
  }
};

document.getElementById("loginBtn").addEventListener("click", async () => {
  // When the user clicks, we start the login process.
  // First we have to create and AuthClient.
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

  redirectToDashboard();
});
