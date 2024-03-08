---
keywords: [beginner, motoko, game of life]
---

# Game of life

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/life)

## Overview

This example contains a series of implementations of Conway's Game of Life.

Its main purpose is to demonstrate state-preserving upgrades using Motoko's stable variables. The implementations are meant to be instructive and are not optimized for efficiency or to hide network latency, which a production implementation would need to consider.

Our `src` directory contains the initial, version v0, implementation but its contents will later be replaced with contents from directories `versions/v1` and `versions/v2`. In a real project, with proper source control, there might be a single src directory, with different versions of code residing in different branches of the repository.

Directory src (version v0) contains an application with a very simple React UI.

Directories `versions/v1` and `versions/v2` contain sequential upgrades to src used to illustrate live upgrades by re-deployment of a deployed canister.

To make upgrades apparent, each version uses a different digit to display live cells (0,1,2).

This is a Motoko example that does not currently have a Rust variant. 


## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Install [Node.js](https://nodejs.org/en/download/).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

Begin by opening a terminal window.

### Step 1: Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the command:

```bash
cd examples/motoko/life
dfx start --background
```

### Step 2: Deploy the canister:

```bash
dfx deploy
```

The deployment step should report a canister ID for the life_assets canister.

Take note of the URL at which the life_assets is accessible using the command:

```bash
echo "http://127.0.0.1:4943/?canisterId=$(dfx canister id life_assets)"
```

### Step 3: Open the frontend in your browser by clicking on the link returned in the output of the previous command.

Click the button **Step**. The grid will advance to the next generation of the Game of Life.

Click the button **Run**. The grid will (slowly) animate sequential generations.

Click the button **Pause**. The animation will pause.

## Upgrading to other versions

Because the v0 implementation makes no provision for upgrades, every time you re-deploy the v0 implementation, the grid will be re-initialized to the same(pseudo-random) initial state.

The state of the grid is represented naively as a nested, mutable array of Boolean values, as described in the `src/State.mo` file:

```motoko
module {

  public type Cell = Bool;
  public type State = [[var Cell]];
  ...
}
```

A `src/life/grid` is represented as a simple class constructed from, and maintaining, state. We omit the details here.

The main actor in `src/life/main` creates a random state and maintains two grid objects, the current and next grid (`cur` and `nxt`). Life's `next()` method advances the Game of Life to the next generation by updating `nxt` from `cur`, using Grid method call `cur.next(nxt)`. The roles of `cur` and `nxt` are then swapped to re-use `cur`'s space for the next generation (a simple application of double-buffering). This logic is described in the `src/life/main.mo` file:

```motoko
import Random = "Random";
import State = "State";
import Grid = "Grid";

actor Life {

  let state = do {
    let rand = Random.new();
    State.new(64, func (i, j) { rand.next() % 2 == 1 });
  };

  var cur = Grid.Grid(state);

  var nxt = Grid.Grid(State.new(cur.size(), func (i, j) { false; }));

  public func next() : async Text {
    cur.next(nxt);
    let temp = cur;
    cur := nxt;
    nxt := temp;
    cur.toText();
  };

  public query func current() : async Text {
    cur.toText()
  };

};
```

Note that none of the variables in this actor are declared stable so their values will not be preserved across upgrade, but re-initialized as on a fresh installation.


### Upgrading to v1
To upgrade to the v1 implementation, issue these commands:

```bash
mv src versions/v0
mv versions/v1 src
dfx deploy
```

Then, return to the same browser tab and refresh (or re-load the link). Note the current grid state is unchanged (thus preserved), apart from changing the display character in grid.
Click the button **Run**, then click the button **Pause** when bored. Open **Details** and click **View State**. Admire the #v1 state on display.

After first upgrading from v0 the state will be random, as on deploying v0. This is because the v0 code did not declare its state variable stable, forcing the upgraded actor to re-initialize state as no previous value for state is available in the retired actor.

However, if you re-deploy the v1 project a second time, perhaps after making a minor edit, you'll see the last state of the grid, before deployment, preserved across the deployment, in a state-preserving upgrade. The random initializer for state is skipped and state just assumes the value it had before the upgrade.


### Upgrading to v2:
To upgrade to the v2 implementation, issue these commands:

```bash
mv src versions/v1
mv versions/v2 src
dfx deploy
```

Return to the same browser tab. Refresh the tab. Note the current grid state is unchanged (thus preserved), apart from changing the display character in the textual display of the grid.

Click the button **Run**, then click the button **Pause** when bored. Open **Details** and click **View State**. Admire the #v2 state on display.

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspect is particularly relevant for this app:
* [Consider using stable memory, version it, test it](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#consider-using-stable-memory-version-it-test-it), since this example is about using stable memory. The best practice focuses on Rust, but are partly also applicable to Motoko. 

