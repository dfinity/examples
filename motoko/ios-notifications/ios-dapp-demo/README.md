# IOS DApp Example

This is an example ios integration with a dapp, intended to demonstrate how an app developer might integrate with an [Internet Identity](https://identity.ic0.app) and send notifications.

## Setting up for local development

To get started, start your `xcode` with the `ios-dapp-demo` project folder and ideally run the app with a physical device so that notifications are made available.

**Important:** By default the ios app points to the remote dapp url, to update this to your local dapp you can change the configuration within [DappConfig.swift](DAppExample/DappConfig.swift). When changing the url, you also need to update the [property list](DAppExample-Info.plist) with the new list of bound domains.
