import { Actor, HttpAgent } from '@dfinity/agent';
import { Certificate, makeNonceTransform } from '@dfinity/agent';
import { cert_var } from '../../declarations'

document.getElementById("certifyBtn").addEventListener("click", async () => {
  const newVal = BigInt(document.getElementById("newValue").value);
  const _ = await cert_var.set(newVal);
  const resp = await cert_var.get();

  // to do -- From where to get the port and host, in general?
  const port = 8000;
  const agent = await Promise.resolve(new HttpAgent({ host: 'http://127.0.0.1:' + port })).then(
      async agent => {
          await agent.fetchRootKey();
          agent.addTransform(makeNonceTransform());
          return agent;
      },
  );
  const now = Date.now() / 1000;

  const cert = new Certificate(resp, agent);

  // Check: Certificate verifies.
  if(!(await cert.verify())) {
    document.getElementById("var").innerText = "Verification failed.";
    return
  }

  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  const te = new TextEncoder();
  const pathTime = [te.encode('time')];
  const rawTime = cert.lookup(pathTime);
  const decodedTime = IDL.decode(
    [IDL.Nat],
    new Uint8Array([
      ...new TextEncoder().encode('DIDL\x00\x01\x7d'),
      ...(new Uint8Array(rawTime) || []),
    ]),
  )[0];
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const time = decodedTime / 1e9;

  // Check: The diff between decoded time and local time is within 5s.
  if(Math.abs(time - now) > 5) {
    document.getElementById("var").innerText = "Timing is wrong.";
    return
  };

  // Checks:
  // - Canister ID is correct.
  // - Certified data is correct.
  const pathData = [te.encode('canister'),
                    cert_var.toText(),
                    te.encode('certified_data')];
  const rawData = cert.lookup(pathData);
  const decodedData = IDL.decode(
    [IDL.Nat32],
    new Uint8Array([
      ...new TextEncoder().encode('DIDL\x00\x01\x7d'),
      ...(new Uint8Array(rawData) || []),s
    ]),
  )[0];
  const expectedData = IDL.Nat32.encodeValue(resp.value);
  expect(decodedData == expectedData).toBe(true);

  document.getElementById("var").innerText = "Certified response.";
});
