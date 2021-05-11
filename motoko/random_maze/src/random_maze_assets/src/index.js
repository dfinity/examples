import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory as random_maze_idl, canisterId as random_maze_id } from 'dfx-generated/random_maze';

const agent = new HttpAgent();
const random_maze = Actor.createActor(random_maze_idl, { agent, canisterId: random_maze_id });

document.getElementById("generateBtn").addEventListener("click", async () => {
  const size = BigInt(document.getElementById("size").value);
  const maze = await random_maze.generate(size);

  document.getElementById("maze").innerText = maze;
});
