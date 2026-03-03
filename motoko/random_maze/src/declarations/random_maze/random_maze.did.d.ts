import type { Principal } from '@icp-sdk/core/principal';
import type { ActorMethod } from '@icp-sdk/core/agent';
import type { IDL } from '@icp-sdk/core/candid';

export interface _SERVICE {
  /**
   * / Given n, returns a maze of n * n cells,
   * / separated by n + 1 partial walls.
   * /
   * / https://en.wikipedia.org/wiki/Maze_generation_algorithm
   * / https://en.wikipedia.org/wiki/Maze_generation_algorithm#Iterative_implementation
   */
  'generate' : ActorMethod<[bigint], string>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
