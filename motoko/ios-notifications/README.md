# IOS DApp integration

This example project exists to showcase a possible solution for integrating a DApp hosted in the [Internet Computer](https://internetcomputer.org/) with multiple platforms, for this example we've created a native ios app.

## Table of contents

- [About](#about)
    - [Demo](#demo)
- [Development](#development)
    - [Setting up for local development](#setting-up-for-local-development)
- [Internet identity](#internet-identity)
- [Notifications](#notifications)
- [References](#references)
- [Disclaimer](#disclaimer)

## About

The example dapp used for this example is hosted under [https://ptf55-faaaa-aaaag-qbd6q-cai.ic0.app](https://ptf55-faaaa-aaaag-qbd6q-cai.ic0.app) and has the following features:

- Show's a login screen if the user is not authenticated with it's [internet identity](https://internetcomputer.org/docs/current/developer-docs/integrations/internet-identity/integrate-identity).
- A user can authenticate with internet identity both within the browser or within the native ios app integration.
- The app accepts multiple routes for navigation. We've only included a `home` page and `about` for this purpose.
- A notification can be sent to the ios app that will load the specified `url` in the notification inside the app. 

### Demo

https://user-images.githubusercontent.com/119848388/208689656-02a3e25e-260d-4c00-b2b0-553afa0fdfea.mov

## Development

To get started, you might want to explore the project directory structure and the default [dfx configuration file](dapp-demo/dfx.json). Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with it, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/ic-overview)
- [Developer Tools](https://internetcomputer.org/tooling)
- [Motoko Programming Language Guide](https://internetcomputer.org/docs/current/developer-docs/build/cdks/motoko-dfinity/motoko)

### Setting up for local development

Before you start make sure the requirements are meet.

#### Requirements
- [nodejs](https://nodejs.org/en/download/)
- [xcode](https://apps.apple.com/us/app/xcode/id497799835)
- [dfx](https://internetcomputer.org/docs/current/developer-docs/ic-overview)

#### Local development

To get started, start a local dfx development environment with the following steps:

```bash
cd dapp-demo
dfx start --background --clean
dfx deploy
```

You can now access the app at `http://localhost:4943/?canisterId={YOUR_LOCAL_CANISTER_ID}`.

> `YOUR_LOCAL_CANISTER_ID` will be made available to you after `dfx deploy`

## Internet Identity

The integration of this dapp with the [internet identity](https://internetcomputer.org/docs/current/developer-docs/integrations/internet-identity/integrate-identity) enables authentication. 

To support the IOS integration it uses the `delegation` and `key` made available in the browser IndexedDB. 

The steps for IOS authentication are:

1. User clicks to authenticate (this triggers the window.open to be called)
1. App intercepts the request and opens a new [ASWebAuthenticationSession](https://developer.apple.com/documentation/authenticationservices/aswebauthenticationsession)
    1. This show's a confirmation dialog, informing the user that the app would like to authenticate using the internet identity domain  
1. After authentication happens a local callback that only happens inside the device with the custom [app scheme](https://developer.apple.com/documentation/xcode/defining-a-custom-url-scheme-for-your-app) is made
1. App receives this callback and injects the `delegation` and `key` into the local [WKWebView](https://developer.apple.com/documentation/webkit/wkwebview) 
1. The webview reloads and the user is now authenticated, since authentication uses indexeddb it continues to work after the user closes the app (expiration time of the session is kept, max is 30 days)

**Example of how this can be handled:**

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
            // the redirect here triggers the custom app scheme
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

## Notifications

The ios app is prepared to receive notifications from remote APN servers. For the scope of this example we haven't setup our own notification server, instead, you can use the [send-notification.sh](send-notification.sh) script to trigger the notification with your own apple developer keys.

These are the steps to show an IOS notification:

1. When the app starts we use UNUserNotificationCenter to request the user for push notification permissions 
1. With granted permissions a request to register for remote notifications is made
1. A device id is made available with the remote call
    1. for development purposes we print this value to the xcode console
1. Execute the [send-notification.sh](send-notification.sh) script with the correct env variables and the notification will appear in your device
    1. A physical ios device is required for this step since the simulator can't register remotely
1. By clicking the notification the app will open in the about page

## References

For specific implementation details defer to:

- [Readme: dapp-demo](dapp-demo/README.md)
- [Readme: ios-dapp-demo](ios-dapp-demo/README.md) 

## Disclaimer

This is an example dapp that demonstrates the potential of integrating a dapp with native apps. 

Please be mindful when considering this code for production and be mindful of:

1. The integration with II in this example is using a universal links which is known to be safer than custom app schemes that are not bound to the appID. To enable this in your production code you should look at setting up [universal links](https://developer.apple.com/documentation/xcode/supporting-universal-links-in-your-app).
1. APN Certificate Key provided by apple needs to be safely stored to avoid a malicious actor from being able to send notitications to your app users.
