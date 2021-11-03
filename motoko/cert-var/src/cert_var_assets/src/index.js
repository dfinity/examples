import { HttpAgent } from '@dfinity/agent';
import { Certificate } from '@dfinity/agent';
import { IDL } from '@dfinity/candid';
import { Principal } from '@dfinity/principal'
import { cert_var, canisterId } from '../../declarations'

const agent = new HttpAgent({});
if (process.env.NODE_ENV !== "production") {
  agent.fetchRootKey();
}

document.getElementById("setBtn").addEventListener("click", async () => {
  const cid = Principal.fromText(canisterId);
  const log = document.getElementById("var");
  const newVal = BigInt(document.getElementById("newValue").value);

  log.innerText = "Setting value " + newVal + " for canister " + cid + "...";

  await cert_var.set(newVal);

  log.innerText = "Getting value " + newVal + " from canister " + cid + "...";

  const resp = await cert_var.get();

  log.innerText = "Verifying gotten value " + newVal + "from " + cid + "...";

  /*
    # Response certification example: Single 32-bit Variable.

    To detect an attacker in the middle between us and the IC (and our
    "true" canister running there), we must either:

    - perform update calls that use "full consensus" (and wait for ~2 sec).
    - perform (fast) query calls whose responses that we, the client, certify,
      using the coordination of the IC and our canister running there.

    This code demonstrates the second approach here, in a minimal setting.

    The full example (beyond this file) consists of a Motoko canister
    (called cert_var here) that holds a single certified variable, as
    a 32-bit number, and an asset canister whose (client-side) code
    (here!) queries and certifies this number's "current certificate".

    The Motoko backend canister prepares for our certification here by
    giving us a "current certificate" within the response; this certificate
    is signed by the entire IC, using a system feature described here:

    https://sdk.dfinity.org/docs/interface-spec/index.html#system-api-certified-data

    Before we trust the response from our apparently "true" canister,
    we interrogate it, and verify its authenticity:

    We do four checks below:

    1. verify system certificate.
    2. check system certificate timestamp is not "too old".
    3. check canister ID in system certificate.
    4. check response matches witness.

    For steps 2, 3 and 4, we access data from the certificate (Blob).
    The Certificate class from the agent-js library provides a way to
    access those items using their paths, like a filesystem, each addressing
    a Blob, encoding something.

    In the case of time and our data, the encodings are each Candid.

    The IC spec represents time using a LEB128 encoding, and certified data
    uses little endian. Ideally, we should use a proper library to decode
    these numbers.  To prevent an extra dependency, we take advantage of the fact
    that the Candid value encoding of Nat and Nat32 happen to use the same
    representation.

    Our data we choose to encode the same as a Candid 32-bit Nat
    (little endian -- see the Motoko canister for details).

    Notably, in an example with more data in the canister than a single number,
    or a more complex query interface, we would generally do more work to
    certify each query response:

    5. use witnesss to re-calculate hash (no witness or hashing needed here.)
    6. check query parameters matches witness (no params, so trivial here.)

    Neither of those steps are needed here, for the reasons given above.
  */

  const readState = { certificate: new Uint8Array(resp.certificate[0]) };
  const cert = new Certificate(readState, agent);

  // Check 1: Certificate verifies.
  if(!(await cert.verify())) {
    log.innerText = "Failure: Certification verification failed.";
    return;
  }
  console.log("Check 1: Verified certificate.", cert);

  const te = new TextEncoder();
  const pathTime = [te.encode('time')];
  const rawTime = cert.lookup(pathTime);
  const idlMessage = new Uint8Array([
      ...new TextEncoder().encode('DIDL\x00\x01\x7d'),
      ...new Uint8Array(rawTime),
  ]);
  const decodedTime = IDL.decode(
    [IDL.Nat], idlMessage
  )[0];
  const time = Number(decodedTime) / 1e9;

  // Check 2: The diff between decoded time and local time is within 5s.
  const now = Date.now() / 1000;
  const diff = Math.abs(time - now);
  if(diff > 5) {
    document.getElementById("var").innerText = "Failure: Timing is wrong.";
    return;
  };
  console.log("Check 2: Timestamp difference seems legit (< 5 sec).", diff);

  // Checks 3 and 4:
  // - Canister ID is correct.
  // - Certified data is correct.
  const pathData = [te.encode('canister'),
                    cid.toUint8Array(),
                    te.encode('certified_data')];
  const rawData = cert.lookup(pathData);
  const decodedData = IDL.decode(
    [IDL.Nat32],
    new Uint8Array([
      ...new TextEncoder().encode('DIDL\x00\x01\x79'),
      ...new Uint8Array(rawData),
    ]),
  )[0];
  const expectedData = resp.value;
  if (expectedData !== decodedData) {
    log.innerText = "Failure: Wrong certified data!";
    return;
  }
  console.log("Check 3: Canister ID is correct.", cid);
  console.log("Check 4: Data is correct.", decodedData);
  console.log("Success.");

  log.innerText = "Success: Fully certified query response.";
});
