import { Principal } from "@dfinity/principal";
import { Ed25519KeyIdentity, Secp256k1KeyIdentity } from "@dfinity/identity";
import { createActor } from "./declarations/invoice";
const fetch = require("isomorphic-fetch");
const identityUtils = require("./utils/identity");
const cliProgress = require("cli-progress");
const { defaultActor, defaultIdentity, balanceHolder } = identityUtils;

const bar1 = new cliProgress.SingleBar({}, cliProgress.Presets.shades_classic);
const encoder = new TextEncoder();

const { exec } = require("child_process");
const run = async () => {
  const canisterId = await new Promise((resolve, reject) => {
    exec("dfx canister id invoice", (err, result) => {
      if (err) {
        reject(err);
      }
      resolve(result.trim());
    });
  });
  // const canisterId = "r7inp-6aaaa-aaaaa-aaabq-cai";
  const randomActor = async () => {
    const identity = Secp256k1KeyIdentity.generate();
    const actor = createActor(canisterId, {
      agentOptions: {
        identity,
        fetch: fetch,
        host: "http://localhost:8000",
      },
    });
    return actor;
  };

  const excessiveCanGet = {
    amount: 1_000_000n,
    token: {
      symbol: "ICP",
    },
    details: [
      {
        description: new Array(256).fill("a").join(""),
        meta: new Array(320).fill(0),
      },
    ],
    permissions: [
      {
        canGet: new Array(256).fill(Principal.fromText("aaaaa-aa")),
        canVerify: [],
      },
    ],
  };
  const maxCapacity = {
    amount: 1_000_000n,
    token: {
      symbol: "ICP",
    },
    details: [
      {
        description: new Array(256).fill("a").join(""),
        meta: new Array(32_000).fill(0),
      },
    ],
    permissions: [
      {
        canGet: new Array(256).fill(Principal.fromText("aaaaa-aa")),
        canVerify: new Array(256).fill(Principal.fromText("aaaaa-aa")),
      },
    ],
  };

  bar1.start(7_500, 0);

  let count = 0;
  // Save one batch for the end
  for (let index = 0; index < 749; index++) {
    // let result = defaultActor.create_invoice(maxCapacity);
    // count += 1;
    // bar1.update(count);
    // if (!result.ok) {
    //   break;
    // }
    try {
      let promises = [];
      for (let i = 0; i < 10; i++) {
        promises.push((await randomActor()).create_invoice(maxCapacity));
      }
      let resolved = await Promise.all(promises);
      let foundError = resolved.find((item) => {
        return !!item.err;
      });
      if (foundError) {
        bar1.stop;
        console.error(foundError);
        break;
      }
      count += 10;
      bar1.update(count);
    } catch (error) {
      console.error(error);
    }
  }

  bar1.stop();
  console.log("Nearly complete. Creating one more invoice to verify");
  const lastInvoice = await defaultActor.create_invoice(maxCapacity);
  console.log(lastInvoice);
};
run();
