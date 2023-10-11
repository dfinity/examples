import { redirectToDashboard, redirectToLoginIfUnauthenticated, getWebApp } from './common.js';

window.onload = async () => {
  const webapp = await getWebApp();
  // If the user isn't authenticated, then he is redirected to the login webpage.
  redirectToLoginIfUnauthenticated(webapp);
  // Calls `get_user_address_str` which returns the Bitcoin address of the current user.
  // This call takes time because it is an update call for security reason.
  const address = (await webapp.get_user_address_str());
  // Shows the Bitcoin address on the page.
  document.getElementById("address").innerText = address;
};

// If the user clicks the "Back to dashboard" button, then he is redirected to the dashboard webpage.
document.getElementById("backToDashboardBtn").addEventListener("click", async () => {
  redirectToDashboard();
});

// If the user clicks the "Copy" button, then his Bitcoin address replaces his clipboard.
document.getElementById("copyBtn").addEventListener("click", async () => {
  const address = document.getElementById("address").innerText;
  navigator.clipboard.writeText(address);
});
