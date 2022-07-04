import { redirectToDashboard, redirectToLoginIfUnauthenticated, getWebApp } from './common.js';

window.onload = async () => {
  window.webapp = await getWebApp();
  redirectToLoginIfUnauthenticated(window.webapp);
  window.balance = await window.webapp.get_balance();
  window.balance = Number(window.balance) / (10 ** 8);
  document.getElementById("amount").max = window.balance;
  window.fees = await window.webapp.get_fees();
  refreshFeeRate();
};

function refreshFeeRate() {
  const feeLevel = document.querySelector('input[name="feeLevel"]:checked').value;
  var feeLevelIndex;
  switch (feeLevel) {
    case 'high':
      feeLevelIndex = 2;
      break;
    case 'std':
      feeLevelIndex = 1;
      break;
    default:
      feeLevelIndex = 0;
  }

  const fee = window.fees[feeLevelIndex];
  document.getElementById("feeMSatB").value = fee;
}

document.getElementById("backToDashboardBtn").addEventListener("click", async () => {
  redirectToDashboard();
});

document.getElementById("setMaxAmountBtn").addEventListener("click", async () => {
  document.getElementById("amount").value = window.balance;
});

["low", "std", "high"].forEach(function(feeLevel) {
  document.getElementById(feeLevel).addEventListener("click", async () => {
    refreshFeeRate();
  });
});

document.getElementById("sendBtn").addEventListener("click", async () => {
  const address = document.getElementById("address").value;
  const amount = parseFloat(document.getElementById("amount").value) * (10 ** 8);
  const fee = parseInt(document.getElementById("feeMSatB").value) / (10 ** 3);
  const allowRBF = document.getElementById("allowRBF").checked;
  const transferResult = await window.webapp.transfer(address, amount, fee, allowRBF);
  //alert(JSON.stringify(transferResult));
  alert(JSON.stringify(transferResult, (key, value) =>
    typeof value === 'bigint'
      ? value.toString()
      : value // return everything else unchanged
  ));
});
