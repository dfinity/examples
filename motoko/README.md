# Motoko examples

## Core Motoko

These examples show the Motoko language as a backend language for the IC, operating directly with clients, without a front end component.

### Basic

- `actor_reference` -- IC management canister as an actor (reference).
- `counter` -- basic (stable) variable demo.
- `echo` -- basic query function demo.
- `factorial` -- basic factorial demo.
- `hello-world` -- basic hello world demo.
- `hello_cycles` -- basic cycles demo.
- `whoami` -- basic caller identification demo.

### Intermediate

- `classes` -- dynamic actor (class) instantiation.
- `pub-sub` -- multiple canisters, with publisher-subscriber inter-canister calls.
- `quicksort` -- sorting an array, via Quick Sort, in Motoko.
- `simple-to-do` -- CRUD-like demo service, sans a front end; see also: `phone-book` and `superheroes`.
- `calc` -- more advanced version of `counter` demo.

## Minimal front end.

These examples use a minimal front end component.

- `random_maze` -- random maze generation, with IC-based randomness.

## Full stack.

These examples use a "conventional" front end component (via `React.Component`).

- `life` -- demonstrates upgrades among three-versions and stable state migration.
- `phone-book` -- CRUD-like demo service.
- `superheroes` -- CRUD-like demo service.
