# Motoko examples

## Core Motoko

These examples show the Motoko language as a backend language for the IC, operating directly with clients, without a front end component.

### Basic

- [`actor_reference`](https://github.com/dfinity/examples/tree/master/motoko/actor_reference) -- IC management canister as an actor (reference).
- [`counter`](https://github.com/dfinity/examples/tree/master/motoko/counter) -- basic (stable) variable demo.
- [`echo`](https://github.com/dfinity/examples/tree/master/motoko/echo) -- basic query function demo.
- [`factorial`](https://github.com/dfinity/examples/tree/master/motoko/factorial) -- basic factorial demo.
- [`hello-world`](https://github.com/dfinity/examples/tree/master/motoko/hello-world) -- basic hello world demo.
- [`hello_cycles`](https://github.com/dfinity/examples/tree/master/motoko/hello_cycles) -- basic cycles demo.
- [`persistent-storage`](https://github.com/dfinity/examples/tree/master/motoko/persistent-storage) -- basic stable var demo.
- [`whoami`](https://github.com/dfinity/examples/tree/master/motoko/whoami) -- basic caller identification demo.

### Intermediate

- [`classes`](https://github.com/dfinity/examples/tree/master/motoko/classes) -- dynamic actor (class) instantiation.
- [`pub-sub`](https://github.com/dfinity/examples/tree/master/motoko/pub-sub) -- multiple canisters, with publisher-subscriber inter-canister calls.
- [`quicksort`](https://github.com/dfinity/examples/tree/master/motoko/quicksort) -- sorting an array, via Quick Sort, in Motoko.
- [`simple-to-do`](https://github.com/dfinity/examples/tree/master/motoko/simple-to-do) -- CRUD-like demo service, sans a front end; see also: `phone-book` and `superheroes`.
- [`calc`](https://github.com/dfinity/examples/tree/master/motoko/calc) -- more advanced version of `counter` demo.
- [`icrc2-swap`](https://github.com/dfinity/examples/tree/master/motoko/icrc2-swap) -- deposit, swap, and withdraw two ICRC-2 tokens.

## Minimal front end.

These examples use a minimal front end component.

- [`random_maze`](https://github.com/dfinity/examples/tree/master/motoko/random_maze) -- random maze generation, with IC-based randomness.
- [`cert_var`](https://github.com/dfinity/examples/tree/master/motoko/cert-var) -- simple certified variable (a single 32-bit number), with client-side certificate validation.
- [`minimal-counter-dapp`](https://github.com/dfinity/examples/tree/master/motoko/minimal-counter-dapp) -- counter dapp with minimal front end.


## Full stack.

These examples use a "conventional" front end component (via `React.Component`).

- [`life`](https://github.com/dfinity/examples/tree/master/motoko/life) -- demonstrates upgrades among three versions and stable state migration.
- [`phone-book`](https://github.com/dfinity/examples/tree/master/motoko/phone-book) -- CRUD-like demo service.
- [`superheroes`](https://github.com/dfinity/examples/tree/master/motoko/superheroes) -- CRUD-like demo service.

## Security Considerations and Security Best Practices

If you base your application on one of these examples, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. The examples provided here may not implement all the best practices.
