Backend blackbox testing
========================

This directory contains a test suite that tests the backend of the Internet
Identity. It uses the [`ic-hs`](https://github.com/dfinity/ic-hs) library to
run a simulated IC environment in a process.

Setup
-----

 * Install ghc-8.8.4 and `cabal` (e.g. using https://www.haskell.org/ghcup/)
 * Run `cabal build`

Running
-------


 * Build the top-level directory, build the backend canister (`dfx build internet_identity`)
 * In the present directory, run
   ```
   cabal run -v0 backend-tests --
   ```

Options
-------

By default, this tests the wasm file in

    ../target/wasm32-unknown-unknown/release/internet_identity.wasm

to use a different one, pass the `--wasm` flag to `backend-tests`. The tests
use a preset CAPTCHA value, so you will need to build with the following
environment variable: `USE_DUMMY_CAPTCHA=1`.

You can select tests to run using `-p`, e.g.

    cabal run -v0 backend-tests -- -p 'get multiple delegations and expire'

See `--help` for more options.

Developing
----------

The simplest way to get a good developer experience is running `ghcid`

    ghcid -c 'cabal repl backend-tests'

while editing, and saving the code to reload in `ghci`. You can also use

    ghcid -c 'cabal repl backend-tests' -T Main.main --setup ":set args -p \"delegation\""

to run some tests after each save, for a quick development iteration
