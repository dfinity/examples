const cliProgress = require("cli-progress");
import { Principal } from "@dfinity/principal";
const identityUtils = require("./utils/identity");

const { 
  defaultActor, 
  defaultIdentity, 
  balanceHolder, 
  delegatedAdministrator, 
  getRandomPrincipal,
  getActorByPrincipal
} = identityUtils;

const MAX_CAPACITY_ALLOWED_CREATOR_LIST = 256;
const MAX_SMALL_CONTENT = 256;
const MAX_LARGE_CONTENT = 32_000;
const MAX_INVOICES_NUMBER = 30_000; // ~33h @ 4s/invoice

let args = process.argv.slice(2);

const encoder = new TextEncoder();

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

const bar1 = new cliProgress.SingleBar({}, cliProgress.Presets.shades_classic);

const run = async () => {
  bar1.start(7_500, 0);

  try {
    let result = await delegatedAdministrator.get_allowed_creators_list();
    if (result?.ok) {
      for (let p of result.ok.allowed) {
        await delegatedAdministrator.remove_allowed_creator({ who: p });
      }
    } else {
      throw new Error("Could prepare for max capacity test--delegated administrator could not access creator allow list");
    }
    // load up our invoice creator actors and give them permission
    let actos = [];
    for (let i = 0; i < MAX_CAPACITY_ALLOWED_CREATOR_LIST; ++i) {
      bar1.increment();
      let p = getRandomPrincipal();
      // assuming there's not going to be a collision
      actos.push(getActorByPrincipal(p));
      result = await delegatedAdministrator.add_allowed_creator({ who: p });
      if (result?.err) {
        bar1.stop;
        throw new Error("Could prepare for max capacity test--delegated administrator could not add principal to creator allow list");
      }
    } 
    // now verify one more can't be added to the list
    result = await delegatedAdministrator.add_allowed_creator({ who: getRandomPrincipal() });
    let allGood = false;
    if (result?.err) {
      if (Object.keys(result.err.kind)[0] === 'MaxAllowed') {
        allGood === true;
      }
    }
    if (!allGood) {
      bar1.stop;
      throw new Error("Maxing out creators allowed list did not work as intended");
    }
    bar1.update(0);
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
          let ra = actos[Math.floor(Math.random() * (actos.length))];
          promises.push(ra.create_invoice(maxCapacity));
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
  } catch (e) {
    console.error(`Could not complete max capacity test due to ${e}`);
  } 
};
run();
