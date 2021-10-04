import { Actor, HttpAgent } from '@dfinity/agent';
import { cert_var } from '../../declarations'

document.getElementById("certifyBtn").addEventListener("click", async () => {
  const newVal = BigInt(document.getElementById("newValue").value);
  const _ = await cert_var.set(newVal);
  const resp = await cert_var.get();

  /* to do -- check the certificate in the resp. */

  document.getElementById("var").innerText = "Certified response.";
});
