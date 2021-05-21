# Random Maze

![Compatibility](https://img.shields.io/badge/compatibility-0.7.0-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-random_maze-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-random_maze-example)

The example generates a random maze using cryptographic randomness.

It illustrates:

* Importing library [Random](https://sdk.dfinity.org/docs/base-libraries/random) to use cryptographic randomness;
* Make asynchronous requests for entropy using
  shared function [Random.blob()](https://sdk.dfinity.org/docs/base-libraries/random#blob); and
* generating bounded, discrete random numbers using helper
  class [Random.Finite(entropy: blob)](https://sdk.dfinity.org/docs/base-libraries/random#type.Finite). Each instance, `f`, of this class consumes its initially supplied entropy as it is called to
  sample from various distributions. Calls to, for example `f.coin()` can fail by returning `null`, requiring `f` to be discarded in favour of a fresh instance of the `Finite` class, constructed from a fresh blob of entropy obtained from a new call to `Random.blob()` (for example `f := Finite(await Random.blob())`. 

## Introduction

The application is built from the following Motoko source code file:

*  [main.mo](./src/random_maze/main.mo), which contains the actor definition and methods exposed by this canister.

This actor use Motoko's `Random` library to generate a cryptographically
random maze of user-specified size.

Function `generate`, calls library function `Random.blob()` asynchronously to
obtain 256-bits of raw entropy (256 random bits as 32 bytes) from the Internet Computer. It makes these calls on demand as it is constructing a maze.
The bits of these blobs are consumed to generate
samples from a variety of discrete distributions using some of
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
