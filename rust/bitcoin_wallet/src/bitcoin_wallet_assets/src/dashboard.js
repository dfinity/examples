import { getWebApp, redirectTo, redirectToLogin } from './common.js';

window.onload = async () => {
  const webapp = await getWebApp();
  // Call `whoami` which returns the principal (user id) of the current user.
  const whoAmI = await webapp.whoami();
  // If the user isn't authenticated, then he is redirected to the login webpage.
  if (whoAmI.isAnonymous()) {
    redirectToLogin();
  }
  else {
    // Shows the principal on the page.
    document.getElementById("principal").innerText = whoAmI;
    // Calls `get_balance` which returns the Bitcoin balance of the current user.
    // This call takes time because it is an update call for security reason.
    const balance = await webapp.get_balance();
    // Shows the Bitcoin balance on the page.
    document.getElementById("balance").innerText = Number(balance) / (10 ** 8);
  }
};

// If the user clicks on the "Log out" button, his Internet Identity authentication
// is cleared and he is redirected to the login webpage.
document.getElementById("logOutBtn").addEventListener("click", async () => {
  localStorage.clear();
  redirectToLogin();
});

// If the user clicks on "Send", "Receive" or "History" button, he is redirected accordingly.
["send", "receive", "history"].forEach(function(page) {
  document.getElementById(page + "Btn").addEventListener("click", async () => {
    redirectTo(page);
  });
});
