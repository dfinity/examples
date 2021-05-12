# Random Maze

![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-random_maze-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-random_maze-example)

The example generates a random maze using cryptographic randomness.

It illustrates:

* Importing library `Random.mo` to use cryptographic randomness;
* Make asynchronous requests for initial and additional entropy using
  shared function `Random.blob()`; and
* generating bounded, discrete random numbers using helper
  class `Random.Finite()` that consumes its supplied entropy to
  sample from various distributions.

## Introduction

The application is built from the following Motoko source code file:

*  [main.mo](./src/random_maze/main.mo), which contains the actor definition and methods exposed by this canister.

This actor use Motoko's 'Random.mo' library to generate a cryptographically
random maze of user-specified space.

Each call to library function `Random.blob()` asynchronously
obtains 256-bits of raw entropy from the Internet Computer.
The random bits of this blob are then  used to generate
samples from a variety of discrete distributions using
the other classes and functions of library `Random.mo`.


## Prerequisites

Verify the following before running this demo:

*  You have downloaded and installed the [DFINITY Canister
   SDK](https://sdk.dfinity.org).

*  You have stopped any Internet Computer or other network process that would
   create a port conflict on 8000.

## Demo

1. Start a local internet computer.

   ```text
   dfx start
   ```

1. Open a new terminal window.

2. Install the front-end dependencies:

   ```text
   npm install
   ```

3. Build and deploy your canisters.

   ```text
   dfx deploy
   ```

4. Take note of the URL at which the user interface is accessible.

   ```text
   echo "http://localhost:8000/?canisterId=$(dfx canister id random_maze_assets)"
   ```

5. Alternatively, generate two random mazes of sizes 9x9 and 65x65:

   ```text
   dfx canister call random_maze generate '(9)'
   dfx canister call random_maze generate '(65)'
   ```

## More info

Specific links:

- [Random Library](https://sdk.dfinity.org/docs/base-libraries/random)
- [Maze Generation](https://en.wikipedia.org/wiki/Maze_generation_algorithm#Iterative_implementation)

General background:

- [Manage Canisters](https://sdk.dfinity.org/docs/developers-guide/working-with-canisters.html)
- [Quick  Start](https://sdk.dfinity.org/developers-guide/quickstart.html)
- [Developer's Guide](https://sdk.dfinity.org/developers-guide)
- [Language Reference](https://sdk.dfinity.org/language-guide)

