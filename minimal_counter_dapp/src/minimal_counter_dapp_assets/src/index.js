import { minimal_counter_dapp } from "../../declarations/minimal_counter_dapp";

document.addEventListener('DOMContentLoaded', async function () {
  const counter = await minimal_counter_dapp.get();
  document.getElementById("counter").innerText = "Counter: " + counter;
})

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const counter = await minimal_counter_dapp.increment();
  document.getElementById("counter").innerText = "Counter: " + counter;
});