import { random_maze } from "../../declarations/random_maze";

document.getElementById("generateBtn").addEventListener("click", async () => {
  const size = BigInt(document.getElementById("size").value);
  const maze = await random_maze.generate(size);

  document.getElementById("maze").innerText = maze;
});
