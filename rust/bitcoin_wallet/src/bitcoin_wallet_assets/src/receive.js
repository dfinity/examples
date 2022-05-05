import { redirectToDashboard, redirectToLoginIfUnauthenticated, getWebApp } from './common.js';

window.onload = async () => {
  const webapp = await getWebApp();
  redirectToLoginIfUnauthenticated(webapp);
  const address = (await webapp.get_principal_address_str());
  document.getElementById("address").innerText = address;
};

document.getElementById("backToDashboardBtn").addEventListener("click", async () => {
  redirectToDashboard();
});

document.getElementById("copyBtn").addEventListener("click", async () => {
  const address = document.getElementById("address").innerText;
  navigator.clipboard.writeText(address);
});
