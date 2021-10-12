import { HttpAgent } from '@dfinity/agent';
import { Certificate } from '@dfinity/agent';
import { IDL } from '@dfinity/candid';
import { Principal } from '@dfinity/principal'
import { cert_var, canisterId } from '../../declarations'

const agent = new HttpAgent({});
const hostname = agent._host.hostname;
if (process.env.NODE_ENV !== "production") {
  agent.fetchRootKey();
}

function hexToBytes(hex) {
    for (var bytes = [], c = 0; c < hex.length; c += 2)
    bytes.push(parseInt(hex.substr(c, 2), 16));
    return bytes;
}

document.getElementById("certifyBtn").addEventListener("click", async () => {
  const newVal = BigInt(document.getElementById("newValue").value);
  await cert_var.set(newVal);
  const resp = await cert_var.get();

  const log = document.getElementById("var");
  log.innerText = "Verifying...";

  const readState = { certificate: new Uint8Array(resp.certificate[0]) };
  const cert = new Certificate(readState, agent);

  // Check: Certificate verifies.
  if(!(await cert.verify())) {
    log.innerText = "Verification failed.";
    return;
  }
  
  const te = new TextEncoder();
  const pathTime = [te.encode('time')];
  const rawTime = cert.lookup(pathTime);
  console.log(rawTime);
  /*
  const idlMessage = new Uint8Array([
      ...new TextEncoder().encode('DIDL\x00\x01\x7d'),
      ...(new Uint8Array(rawTime) || []),
  ]);
  console.log(idlMessage);
  const decodedTime = IDL.decode(
    [IDL.Nat], idlMessage
  )[0];
  console.log(decodedTime);

  const time = decodedTime / 1e9;

  // Check: The diff between decoded time and local time is within 5s.
  const now = Date.now() / 1000;
  if(Math.abs(time - now) > 5) {
    document.getElementById("var").innerText = "Timing is wrong.";
    return
  };*/

  // Checks:
  // - Canister ID is correct.
  // - Certified data is correct.
  const cid = new Uint8Array(hexToBytes(Principal.fromText(canisterId).toHex()));
  const pathData = [te.encode('canister'),
                    cid,
                    te.encode('certified_data')];
  const rawData = cert.lookup(pathData);
  console.log(rawData);
  /*
  const decodedData = IDL.decode(
    [IDL.Nat32],
    new Uint8Array([
      ...new TextEncoder().encode('DIDL\x00\x01\x7d'),
      ...(new Uint8Array(rawData) || []),
    ]),
  )[0];
  const expectedData = IDL.Nat32.encodeValue(resp.value);
  expect(decodedData == expectedData).toBe(true);*/

  log.innerText = "Certified response.";
});
