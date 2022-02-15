import { hack } from "../../declarations/hack";
/*
async function load() {
  const counter = await hack.getCount();
  document.getElementById("counter").innerText = "Counter: " + counter;
  console.log(counter);
}

window.onload = load();
*/
document.addEventListener('DOMContentLoaded', async function () {
  const counter = await hack.getCount();
  document.getElementById("counter").innerText = "Counter: " + counter;
})

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const counter = await hack.count();
  console.log(counter);
  document.getElementById("counter").innerText = "Counter: " + counter;
});
