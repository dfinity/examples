import { canisterId, createActor } from "../../declarations/seller";
import { Secp256k1KeyIdentity } from "@dfinity/identity";

const base = new TextEncoder().encode("lorem-ipsum-dolor-sit-amet");
const seed = new Uint8Array([...base, ...new Uint8Array(32 - base.byteLength)]);

const userFromSeed = Secp256k1KeyIdentity.generate(seed);

const actor = createActor(canisterId, {
  agentOptions: {
    identity: userFromSeed,
  },
});

export class InvoiceHandler extends HTMLElement {
  constructor() {
    // establish prototype chain
    super();

    const shadow = this.attachShadow({ mode: "open" });

    // creating a container for the editable-list component
    const container = document.createElement("div");

    // adding a class to our container for the sake of clarity
    container.classList.add("license-badge");

    // creating the inner HTML of the editable list element
    container.innerHTML = `
          <style>
          </style>
          <div> license badge </div>
        `;

    // binding methods
    this.checkStatus = this.checkStatus.bind(this);

    // appending the container to the shadow DOM
    shadow.appendChild(container);
    this.checkStatus();
  }

  async checkStatus() {
    const status = await actor.check_license_status();
    console.log(status);
  }

  // fires after the element has been attached to the DOM
  connectedCallback() {}
}

customElements.define("license-badge", InvoiceHandler);
