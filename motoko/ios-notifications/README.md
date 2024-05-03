---
keywords: [advanced, motoko, ios, ios notifications]
---

# iOS integration

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/ios-notifications)


## Overview
[iOS integration](https://github.com/dfinity/examples/tree/master/motoko/ios-notifications) is an experimental dapp with a native app integration that showcases a possible solution for integrating a dapp hosted in the Internet Computer with multiple platforms. For this example, we've created an iOS app.

This is to create an example of a simple integration of a dapp running purely on ICP and using [Internet Identity](https://internetcomputer.org/docs/current/docs/current/references/ii-spec) with a native iOS application that can let the user have a native feel such as authenticating and receive push notifications.

## Architecture 

The basic functionality of the IOS integration consists of four main components:

- First, create a dapp that is integrated with [Internet Identity](https://internetcomputer.org/docs/current/docs/current/references/ii-spec) and has a basic routing functionality. While the user is not authenticated it can only see the login page and when authenticated can navigate between the about and home page.

- Second, create a new IOS native application that serves as a wrapper for the dapp and creates a native feel for the user.

- Third, a proxy page was added in the dapp to enable the user to securely authenticate using [Internet Identity](https://internetcomputer.org/docs/current/docs/current/references/ii-spec) and keep the authenticated session in the webview until it expires, even when the user exits the app and re-opens it the session persists.

- Fourth, the dapp is configured to receive push notifications from the system and open a specified URL, this allows for notifications to be sent serving as a mechanism to deep link into a specific section of the dapp. 

## Prerequisites
- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Install [Node.js](https://nodejs.org/en/download/).
- [x] [xcode](https://apps.apple.com/us/app/xcode/id497799835).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

### Step 1: Local development.

To get started, start a local `dfx` development environment with the following steps:

```bash
cd examples/motoko/ios-notifications/dapp-demo
dfx start --background --clean
```

### Step 2: Install the dependency packages:

```bash
npm install
```

### Step 3: Deploy your canisters:

```bash
dfx deploy
```

### Step 4: Start the frontend:

```bash
npm start
```

You can now access the dapp at `http://localhost:4943/?canisterId={YOUR_LOCAL_CANISTER_ID}`.

> `YOUR_LOCAL_CANISTER_ID` will be made available to you in the output of the `dfx deploy` command.

### Step 5: Using Internet Identity.

The integration of this dapp with the [Internet Identity](https://internetcomputer.org/docs/current/developer-docs/integrations/internet-identity/integrate-identity) enables authentication. To support the IOS integration,  it uses the `delegation` and `key` made available in the browser IndexedDB. 

The steps for iOS authentication are:

1. User clicks to authenticate (this triggers the `window.open` to be called).
2. The dapp intercepts the request and opens a new [ASWebAuthenticationSession](https://developer.apple.com/documentation/authenticationservices/aswebauthenticationsession).
    - This shows a confirmation dialog, informing the user that the dapp would like to authenticate using the Internet Identity domain.
3. After authentication happens, a local callback that only happens inside the device with the [universal link](https://developer.apple.com/documentation/xcode/supporting-universal-links-in-your-app) is made.
4. The dapp receives this callback and injects the `delegation` and `key` into the local [WKWebView](https://developer.apple.com/documentation/webkit/wkwebview).
5. The webview reloads and the user is now authenticated, since authentication uses IndexedDB, it continues to work after the user closes the dapp (expiration time of the session is kept, max is 30 days).

#### Example of how this can be handled:

```ts
async handleMultiPlatformLogin(): Promise<void> {
    const key = await this.storage.get(KEY_STORAGE_KEY) ?? undefined;
    const delegation = await this.storage.get(KEY_STORAGE_DELEGATION) ?? undefined;
    const identityParam: IdentityParam = { key, delegation };
    const preloadParam = Buffer.from(JSON.stringify(identityParam), "ascii").toString("base64");
    const url = Auth.currentURL();

    switch(this.loginType()) {
        case AuthLoginType.Ios:
            const iosCallback = new URL(url.searchParams.get(AuthLoginType.Ios) ?? "");
            iosCallback.searchParams.append(Auth.identityPreloadProp, preloadParam);
            // the redirect here triggers the app universal link
            // such as dappexample://auth?_identity=...
            // and this is what the app intercepts and handles
            window.location.href = iosCallback.toString();
            break;
        default:
            // desktop is enabled by default and doesn't need a special condition
            break;
    }
}
```

### Step 6: Notifications

The IOS app is prepared to receive notifications from remote APN servers. For the scope of this example, you haven't setup your own notification server. Instead, you can use the `send-notification.sh` script to trigger the notification with your own Apple developer keys.

These are the steps to show an IOS notification:

1. When the app starts use UNUserNotificationCenter to request the user for push notification permissions.
2. With granted permissions a request to register for remote notifications is made.
3. A device ID is made available with the remote call.
    - For development purposes, you print this value to the xcode console.
4. Execute the `send-notification.sh` script with the correct `env` variables and the notification will appear in your device.
    - A physical IOS device is required for this step since the simulator can't register remotely.
5. By clicking the notification the dapp will open on the about page.

## Security considerations

- When integrating with Internet Identity make sure to set up and use universal links, other forms of passing the delegation such as a custom app scheme are known to have security risks such as allowing others to impersonate your dapp.
-  For this example, you are generating the key pair in the native app but the private key is still available there since the signing of the request happens on the dapp, in your production application make sure this is generated by your native app and stored in a [secure enclave](https://developer.apple.com/documentation/security/certificate_key_and_trust_services/keys/protecting_keys_with_the_secure_enclave), and expose a `sign` method to the dapp through webkit, this will ensure that your dapp does not have access to your private key.

## References

For further details, please refer to the [README file](https://github.com/dfinity/examples/blob/master/motoko/ios-notifications/README.md).


## Disclaimer

This is an example dapp that demonstrates the potential of integrating a dapp with native apps. 

Please be mindful when considering this code for production and be mindful of:

1. The integration with II in this example is using universal links which is known to be safer than custom app schemes that are not bound to the appID. To enable this in your production code you should look at setting up [universal links](https://developer.apple.com/documentation/xcode/supporting-universal-links-in-your-app).
2. APN Certificate Key provided by Apple needs to be safely stored to prevent a malicious actor from being able to send notifications to your app users.
