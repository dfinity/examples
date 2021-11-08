# Certified Variable

![Compatibility](https://img.shields.io/badge/compatibility-0.7.0-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-cert-var-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-cert-var-example)

The example demonstrates the use of a single cryptographically certified variable, as supported by the Internet Computer.



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

1. Open a new terminal window.

2. Install the front-end dependencies:

   ```text
   npm install
   ```

3. Build and deploy your canisters.

   ```text
   dfx deploy
   ```

4. Start a local web server hosting the front end.

   ```text
   npm start
   ```

5. Visit the frontend, and do the demo there:

   http://localhost:8080/

   Should present an entry for "New value of variable",
   and a button to "Set and get!".

   Enter a number and click the button.

   The canister updates its certificate, and the frontend checks it.

   The developer console contains some additional comments about each step.

## Demo explanation

In a nut shell, this example code demonstrates "response certification" for
a canister that holds a single 32-bit variable.  It has two sides:

- Backend (BE) canister logic in Motoko (main.mo)
- Frontend (FE) logic in JS (index.js)

To detect an attacker in the middle between the FE and the IC and our
"true" BE canister running there, we must either:

- perform update calls that use "full consensus" (and wait for ~2 sec).
- perform (fast) query calls whose responses that we, the client, certify,
  using the coordination of the IC and our canister running there.

The FE and BE code demonstrates the second approach here, in a minimal setting.

The BE holds a single certified variable, as a 32-bit number, and the
FE code queries and certifies this number's "current certificate".

The BE prepares for the FE certification by giving the FE a "current
certificate" within the response; this certificate is signed by the
entire IC, using a [special system feature](https://sdk.dfinity.org/docs/interface-spec/index.html#system-api-certified-data).

Before the FE trusts the response from the apparent BE canister,
it interrogates it, and verifies its authenticity:

The FE does four checks:

1. verify system certificate.
2. check system certificate timestamp is not "too old".
3. check canister ID in system certificate.
4. check response matches witness.

For steps 2, 3 and 4, the FE accesses data from the certificate (`Blob`).

The `Certificate` class from the agent-js library provides a way to
access those items using their paths, like a filesystem, each addressing
a `Blob`, encoding something.

In the case of time and our data, the encodings are each Candid.

The IC spec represents time using a [LEB128 encoding](https://en.wikipedia.org/wiki/LEB128),
and certified data uses little endian.

Ideally, we should use a proper library to decode these numbers.  To
prevent an extra dependency, we take advantage of the fact that the
Candid value encoding of Nat and Nat32 happen to use the same
representation.

Our data we choose to encode the same as a Candid 32-bit Nat
(little endian -- see the Motoko canister for details).

Notably, in an example with more data in the canister than a single number,
or a more complex query interface, we would generally do more work to
certify each query response:

5. use witnesss to re-calculate hash (no witness or hashing needed here.)
6. check query parameters matches witness (no params, so trivial here.)

Neither of those steps are needed here, for the reasons given above.

## More info

General background:

- [Manage Canisters](https://sdk.dfinity.org/docs/developers-guide/working-with-canisters.html)
- [Quick  Start](https://sdk.dfinity.org/developers-guide/quickstart.html)
- [Developer's Guide](https://sdk.dfinity.org/developers-guide)
- [Language Reference](https://sdk.dfinity.org/language-guide)
