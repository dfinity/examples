import {AuthClient} from "@dfinity/auth-client"
import {DelegationIdentity, Ed25519PublicKey, Ed25519KeyIdentity, DelegationChain} from "@dfinity/identity";

function fromHexString(hexString) {
    return new Uint8Array((hexString.match(/.{1,2}/g) ?? []).map(byte => parseInt(byte, 16))).buffer;
}

let appPublicKey;

var url = window.location.href;
var publicKeyIndex = url.indexOf("sessionkey=");
if (publicKeyIndex !== -1) {
    // Parse the public key.
    var publicKeyString = url.substring(publicKeyIndex + "sessionkey=".length);
    appPublicKey = Ed25519PublicKey.fromDer(fromHexString(publicKeyString));
    // console.log(appPublicKey);
}

let delegationChain;

const loginButton = document.getElementById("login");
loginButton.onclick = async (e) => {
    e.preventDefault();

    // Create an auth client.
    var middleKeyIdentity = Ed25519KeyIdentity.generate();
    let authClient = await AuthClient.create({
        identity: middleKeyIdentity,
    });

    // Start the login process and wait for it to finish.
    await new Promise((resolve) => {
        authClient.login({
            identityProvider: "https://identity.ic0.app/#authorize",
            onSuccess: resolve,
        });
    });

    // At this point we're authenticated, and we can get the identity from the auth client.
    const middleIdentity = authClient.getIdentity();
    // console.log(middleIdentity);

    // Chain the app key.
    if (appPublicKey != null && middleIdentity instanceof DelegationIdentity ) {
        let middleToApp = await DelegationChain.create(
            middleKeyIdentity,
            appPublicKey,
            new Date(Date.now() + 15 * 60 * 1000),
            { previous: middleIdentity.getDelegation() },
        );
        // console.log(middleToApp);

        delegationChain = middleToApp;
    }

    return false;
};

const openButton = document.getElementById("open");
openButton.onclick = async (e) => {
    e.preventDefault();

    if (delegationChain == null){
        console.log("Invalid delegation chain.");
        return false;
    }

    var url = "internetidentity://authorize?";
    var delegationString = JSON.stringify(delegationChain.toJSON());
    url = url + "delegation=" + encodeURIComponent(delegationString);
    //console.log(url);

    window.open(url, "_self");

    return false;
};
