import { Actor, HttpAgent } from '@dfinity/agent';
import { Certificate } from '@dfinity/agent';
import { cert_var } from '../../declarations'

document.getElementById("certifyBtn").addEventListener("click", async () => {
  const newVal = BigInt(document.getElementById("newValue").value);
  const _ = await cert_var.set(newVal);
  const resp = await cert_var.get();

  const agent = await Promise.resolve(new HttpAgent({ host: 'http://127.0.0.1:' + port, identity })).then(
      async agent => {
          await agent.fetchRootKey();
          agent.addTransform(makeNonceTransform());
          return agent;
      },
  );
  const now = Date.now() / 1000;

  const cert = new Certificate(resp.certificate, agent);
  expect(await cert.verify()).toBe(true);

  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  const te = new TextEncoder();
  const pathTime = [te.encode('time')];
  const rawTime = cert.lookup(pathTime)!;
  const decodedTime = IDL.decode(
    [IDL.Nat],
    new Uint8Array([
      ...new TextEncoder().encode('DIDL\x00\x01\x7d'),
      ...(new Uint8Array(rawTime) || []),
    ]),
  )[0];
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const time = Number(decoded as any) / 1e9;
  // The diff between decoded time and local time is within 5s
  expect(Math.abs(time - now) < 5).toBe(true);

  const pathData = [te.encode('canister'),
                    te.encode("to do -- canister id"),
                    te.encode('certified_data')];
  const rawData = cert.lookup(pathData)!;
  const decodedData = IDL.decode(
    [IDL.Nat32],
    new Uint8Array([
      ...new TextEncoder().encode('DIDL\x00\x01\x7d'),
      ...(new Uint8Array(rawData) || []),s
    ]),
  )[0];
  // the certified data should be the same as the value
  // (to do -- modulo the byte ordering of how we represent it...?)
  expect(decodedData == resp.value).toBe(true);

  document.getElementById("var").innerText = "Certified response.";
});
