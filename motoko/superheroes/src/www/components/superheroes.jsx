import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory as superheroes_idl, canisterId as superheroes_id } from 'dfx-generated/superheroes';

const agent = new HttpAgent();
const Superheroes = Actor.createActor(superheroes_idl, { agent, canisterId: superheroes_id });

export default Superheroes;
