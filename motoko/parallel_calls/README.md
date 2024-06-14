---
keywords: [advanced, motoko, defi]
---

# Parallel inter-canister calls

[View this sample code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/parallel_calls).

## Overview

This example demonstrates how to implement inter-canister calls that run in parallel in Motoko, and highlights some differences between parallel and sequential calls. Running independent calls in parallel can lower the latency, especially when messages are sent across subnets. For example, a canister that swaps two tokens might want to launch both token transfer operations in parallel.

## Architecture

The sample code revolves around two simple canisters, `caller` and `callee`. `Caller` has three endpoints:
1. `setup_callee`, to set the ID of the callee canister.
2. `sequential_calls` and `parallel_calls`, which both take a number `n` and issue `n` calls to the callee, returning the number of successful calls. The former performs calls sequentially, the latter in parallel.

The callee exposes a simple `ping` endpoint that takes no parameters and returns nothing.

## Prerequisites

To run this example you should:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

## Running the example

Begin by opening a terminal window.

### Step 1: Navigate into the example folder and start a local Internet Computer replica

```bash
cd examples/motoko/parallel_calls
dfx start --background
```

### Step 2: Deploy the canister

```bash
dfx deploy
```

### Step 3: Set up the caller canister

We now provide the ID of the callee to the caller, such that the caller can initiate calls.
```
dfx canister call caller setup_callee "(principal \"`dfx canister id callee`\")"
```

### Step 4: Invoke sequential and parallel calls

Let's first call the different endpoints of the `caller` canister using `dfx`

```bash
dfx canister call caller sequential_calls 100
```

This should output:
```bash
(100 : nat64)
```

And the other endpoint: 

```bash
dfx canister call caller parallel_calls 100
```

which outputs:

```bash
(100 : nat64)
```

The results are identical: all calls succeed. There also isn't a large difference in performance between these calls:

```bash
time dfx canister call caller sequential_calls 100
(100 : nat64)
dfx canister call caller sequential_calls 100  0.11s user 0.03s system 7% cpu 1.848 total
time dfx canister call caller parallel_calls 100
(100 : nat64)
dfx canister call caller parallel_calls 100  0.11s user 0.03s system 8% cpu 1.728 total
```

The reason why the performance is similar is because the local replica has only a single subnet. Inter-canister calls normally have almost no latency on a single subnet, so it doesn't matter much if we run them sequentially or in parallel.

However, once we increase the number of calls, we observe a difference in both the results and performance.

```bash
time dfx canister call caller sequential_calls 2000
(2_000 : nat64)
dfx canister call caller sequential_calls 2000  0.18s user 0.03s system 1% cpu 15.587 total
time dfx canister call caller parallel_calls 2000
(500 : nat64)
dfx canister call caller parallel_calls 2000  0.11s user 0.03s system 4% cpu 3.524 total
```

All the sequential calls succeed, but most parallel calls fail. The reason is that the replica imposes a limit on the number of in-flight calls a canister can make (in particular, to a different canister). Doing the calls sequentially yields only one in-flight call at a time. However, too many parallel calls exceed the limit, after which the calls start failing. Note that it's also possible to hit this limit with sequential calls under high load (if `sequential_call` was itself called many times in parallel). If such limits are hit, immediate retries will also fail; retries should be done in a timer or a heartbeat instead.

Lastly, the parallel calls here complete sooner -- because most of them fail!

### Step 5: Multi-subnet setting

Parallel calls are a lot more useful in multi-subnet settings. We can create such a setting locally using Pocket IC. 

First, follow the [installation instructions](https://github.com/dfinity/pocketic) to install `pocket-ic` in the `parallel_calls` directory. 

Then, run the pre-made test, which now installs the `caller` and `callee` canisters on different subnets, and then runs 90 calls sequentially/in parallel.

```bash
CALLER_WASM=.dfx/local/canisters/caller/caller.wasm CALLEE_WASM=.dfx/local/canisters/callee/callee.wasm cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/multi_subnet
Sequential calls: 90/90 successful calls in 599.863583ms
Parallel calls: 90/90 successful calls in 296.402ms
```

As you can see, parallel calls run a lot faster than sequential calls here. The difference on the IC mainnet would be significantly larger still, as Pocket IC executes rounds much faster than the IC mainnet.
