import { getWebApp, redirectTo, redirectToLogin } from './common.js';

window.onload = async () => {
  const webapp = await getWebApp();
  // Call whoami which returns the principal (user id) of the current user.
  const whoAmI = await webapp.whoami();
  if (whoAmI.isAnonymous()) {
    redirectToLogin();
  }
  else {
    // show the principal on the page
    document.getElementById("principal").innerText = whoAmI;
    // Call get_balance which returns the balance of the current user.
    // This call is very long because it is an update for security reason.
    const balance = await webapp.get_balance();
    // show the balance on the page
    document.getElementById("balance").innerText = Number(balance) / (10 ** 8);
  }
};

document.getElementById("logOutBtn").addEventListener("click", async () => {
  localStorage.clear();
  redirectToLogin();
});

["send", "receive", "history"].forEach(function(page) {
  document.getElementById(page + "Btn").addEventListener("click", async () => {
    redirectTo(page);
  });
});
