import {AuthClient} from "@dfinity/auth-client"
import {SignIdentity} from "@dfinity/agent";
import {DelegationIdentity, Ed25519PublicKey } from "@dfinity/identity";

// An imcomplete Ed25519KeyIdentity with only the public key provided.
class ImcompleteEd25519KeyIdentity extends SignIdentity {
    constructor(publicKey) {
        super();
        this._publicKey = publicKey;
    }

    getPublicKey () {
        return this._publicKey;
    }
}

function fromHexString(hexString) {
    return new Uint8Array((hexString.match(/.{1,2}/g) ?? []).map(byte => parseInt(byte, 16))).buffer;
}

let myKeyIdentity;
let sessionKeyIndex = -1;

var url = window.location.href;
sessionKeyIndex = url.indexOf("sessionkey=");
if (sessionKeyIndex !== -1) {
    // Parse the public session key and instantiate an ImcompleteEd25519KeyIdentity.
    var sessionkey = url.substring(sessionKeyIndex + "sessionkey=".length);

    var publicKey = Ed25519PublicKey.fromDer(fromHexString(sessionkey));
    myKeyIdentity = new ImcompleteEd25519KeyIdentity(publicKey);
} else {
    // TODO: initialize an Ed25519KeyIdentity();
}

let delegationIdentity;

const loginButton = document.getElementById("login");
loginButton.onclick = async (e) => {
    e.preventDefault();

    // Create an auth client.
    let authClient = await AuthClient.create({
        identity: myKeyIdentity,
    });

    // Start the login process and wait for it to finish.
    await new Promise((resolve) => {
        authClient.login({
            identityProvider: "https://identity.ic0.app/#authorize",
            onSuccess: resolve,
        });
    });

    // At this point we're authenticated, and we can get the identity from the auth client.
    const identity = authClient.getIdentity();
    if (identity instanceof DelegationIdentity) {
        delegationIdentity = identity;
    }

    return false;
};

const openButton = document.getElementById("open");
openButton.onclick = async (e) => {
    e.preventDefault();

    // if (sessionKeyIndex === -1) {
    //     // TODO: warning for not login from a game.
    //     return false;
    // }
    
    var url = "https://6x7nu-oaaaa-aaaan-qdaua-cai.icp0.io/authorize?";
    if (delegationIdentity != null) {
        var delegationString = JSON.stringify(delegationIdentity.getDelegation().toJSON());
        console.log(delegationString);
        url = url + "delegation=" + encodeURIComponent(delegationString);
    }

    window.open(url, "_self");

    return false;
};
