# Persistant storage
The example dapp shows how to build a simple dapp in Motoko, which will have persistant storage. The dapp is a simple counter, which will increment a counter, retrieve the counter value and reset the counter value by calling backend functions. The functions are exposed through a Candid interface. 

![Counter Frontend](README_images/candid_ui.png)

## Introduction
The purpuse of this example dapp is to build a simple counter dapp, where the counter value will persist even after the dapp has changed and been re-deployed. 

This example covers:

- Create new canister smart contract using Motoko
- Add backend functions for a counter (increment, get count and reset count)
- Deploy the canister smart contract locally
- Test backend with Candid UI and command line using DFX 

## Installation
This example project can be cloned, installed and deployed locally, for learning and testing purposes. The instructions are based on running the example on either macOS or Linux, but when using WSL2 on Windows, the instructions will be the same.

### Prerequisites
The example project requeres the following installed:

- GitHub
- DFX 

GitHub can be installed from various package managers. DFX can be installed following the instructions [here](https://smartcontracts.org/docs/quickstart/local-quickstart.html#download-and-install).

### Install
Install the example dapp project:

```bash
$ git clone https://github.com/dfinity/examples
$ cd motoko/persistant storage
```

## Documentation
The two main parts of the example dapp is the backend and the Candid interface. This example project does not have a frontend.

### Motoko backend
The backend functions are located in the `src/persistant_storage/main.mo` Motoko file. The backend stores the counter value, and has functions to get, increment and reset the counter value. Furthermore the backend insures the counter value persists upgrades of the dapp. 

#### Counter variable
The current counter value is stored as a number in the actor. 

```javascript
actor {
    stable var counter : Nat = 0;
}
```

#### increment()
The `increment()` function increments the counter variable.

```javascript
public func increment() : async Nat {
    counter += 1;
    return counter;
};
```

The function is returning the incremented counter variable.

#### get()
The `get()` function returns the current counter value.

```javascript
public query func get() : async Nat {
    return counter;
};
```

#### reset()
The `reset()` function resets the counter value to 0 and returns the value.

```javascript
public func reset() : async Nat {
    counter := 0;
    return counter;
};
```

### Candid interface
The Candid interface is automatically created, and it has a convienient UI, which provides an easy, user friendly way to test the backend. The UI is also automatically generated, and the canister ID can be found in the `canister_ids.json` file. 

The localhost version of the `canister_ids.json` file can be found in `.dfx/local/canister_ids.json` and the URL is: 

**http://<candid_canister_id>.localhost:8000/?id=<backend_canister_id>**

## Deployment
The local network is started by running this command:

```bash
$ dfx start --background
```

When the local network is up and running, run this command to deploy the canisters:

```bash
$ dfx deploy
```

## Testing
There are two ways of testing the functionality of this example dapp. One way is by making command line requests using DFX, and the other way is to use the Candid UI. Before the example dapp can be tested, it must be deployed (locally) like described in the above Deployment section. 

### DFX
DFX has a subset of commands for canister operations, and one of them enables calling the public functions added to the `main.mo` file in the previous step. In the following examples the initial value is 0. `increment` will increment value and return 1, `get` will return the current value and `reset` will set the value to 0.

Command usage: `dfx canister call &#60project&#62  &#60function&#62`

```bash
$ dfx canister call persistant_storage increment
(1 : Nat)
```

```bash
$ dfx canister call persistant_storage get
(1 : Nat)
```

```bash
$ dfx canister call persistant_storage reset
(0 : Nat)
```

### Candid UI
The Candid UI provides an easy, user friendly interface for testing the dapp. The UI is automatically generated, and the canister ID can be found in the `canister_ids.json` file. 

The localhost version of the `canister_ids.json` file can be found in `.dfx/local/canister_ids.json` and the URL is: 

**http://<candid_canister_id>.localhost:8000/?id=<backend_canister_id>**

![Candid UI](README_images/candid_ui.png)

## License
This project is licensed under the Apache 2.0 license, see LICENSE.md for details. See CONTRIBUTE.md for details about how to contribute to this project. 
