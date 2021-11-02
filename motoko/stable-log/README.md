# Stable log

![Compatibility](https://img.shields.io/badge/compatibility-0.7.0-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-stable-log-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-stable-log-example)

The example demonstrates the use of a stable log of time-coded text entries that:
 - survives canister upgrades, 
 - but without using extra time or space, unlike Motoko's current implementation of `stable var`s.
  
 To accomplish these criteria, we demonstrate a low-level, experimental stable memory API.

## Background: Stable memory in Motoko

- [High-level `stable var` feature is **not yet** efficient enough,](https://sdk.dfinity.org/docs/language-guide/upgrades.html)
- [So there is a low-level base library module which is experimental.](https://github.com/dfinity/motoko-base/blob/doc-pages/modules/base-libraries/pages/ExperimentalStableMemory.adoc)

This example shows the latter, and verifies its correctness by comparing to the former.

## Prerequisites

Verify the following before running this demo:

*  You have downloaded and installed the [DFINITY Canister
   SDK](https://sdk.dfinity.org).

*  You have stopped any Internet Computer or other network process that would
   create a port conflict on 8000.

## Demo playbook

1. Start a local internet computer.

   ```text
   dfx start
   ```

3. Build and deploy your canisters.

   ```text
   dfx deploy --no-wallet
   ```

4. Run CI tests, locally.

   To do.

## Demo explanation

In a nut shell, this example code demonstrates a log that grows forever,
that always gives sublinear access to existing log entries, and 
sublinear update time for adding new entries.  On upgrade, the log
survives in stable memory, without any additional copying or processing.

To do -- finish.

###


## More info

Sequences as (Cartesian) trees:

- [Wikipedia article](https://en.wikipedia.org/wiki/Cartesian_tree)
- [Motoko package](https://github.com/matthewhammer/motoko-sequence)

Stable memory in Motoko:

- [High-level `stable var` feature (not yet efficient enough)](https://sdk.dfinity.org/docs/language-guide/upgrades.html)
- [Doc for low-level base library module (experimental)](https://github.com/dfinity/motoko-base/blob/doc-pages/modules/base-libraries/pages/ExperimentalStableMemory.adoc)


General background:

- [Manage Canisters](https://sdk.dfinity.org/docs/developers-guide/working-with-canisters.html)
- [Quick  Start](https://sdk.dfinity.org/developers-guide/quickstart.html)
- [Developer's Guide](https://sdk.dfinity.org/developers-guide)
- [Language Reference](https://sdk.dfinity.org/language-guide)
