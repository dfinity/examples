import React from "react";
import { Button, defaultTheme, Provider } from "@adobe/react-spectrum";
import Status from "./Status";
import InvoiceManager from "./InvoiceManager";
import { get, clear } from "local-storage";
import { sellerActor } from "../identity";

/* If you want to make it easier to debug:
(console.log(JSON.stringify(BigInt will throw error otherwise)))
BigInt.prototype.toJSON = function () {
  return this.toString();
};
*/

const App = () => {
  const [status, setStatus] = React.useState(null);
  React.useEffect(() => {
    const savedId = get("invoice-id");
    if (savedId)
      sellerActor.check_license_status().then((result) => {
        setStatus(result);
      });
    else {
      setStatus(false);
    }
  }, []);

  const reset = async () => {
    await sellerActor.reset_license();
    clear();
    location.reload();
  };

  return (
    <Provider theme={defaultTheme}>
      <main>
        <div>
          <h1>Invoice Payment Flow</h1>
          <p>
            This dapp illustrates a basic flow for a canister selling a simple
            license, in exchange for payment in ICP or a token using the ICRC1 standard.
          </p>
          <Status status={status} />
          <InvoiceManager status={status} setStatus={setStatus} />
        </div>
        <section id="reset">
          <Button variant="negative" type="reset" onPress={reset}>
            Reset License
          </Button>
        </section>
      </main>
      <footer>
        <section id="credits">
          <img
            src="ic-badge-powered-by-crypto_bg-dark.svg"
            alt="powered by crypto"
          />
          Credit to <a href="https://pokedstudio.com">PokedStudio</a> for use of
          the verified/unverified icons
        </section>
      </footer>
      <></>
    </Provider>
  );
};

export default App;
