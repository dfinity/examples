import { persistent_storage } from "../../declarations/persistent_storage";

document.addEventListener('DOMContentLoaded', async function () {
  const counter = await persistent_storage.get();
  document.getElementById("counter").innerText = "Counter: " + counter;
})

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const counter = await persistent_storage.increment();
  document.getElementById("counter").innerText = "Counter: " + counter;
});