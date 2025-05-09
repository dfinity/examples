![Group 5981 (1)](https://user-images.githubusercontent.com/73345016/144523306-f6041b24-bd34-4ecf-bc0f-6c96b0c24ca8.png)

## DIP20 - Introduction

Token standards are essential for the Internet Computer ecosystem, especially for the decentralized finance ecosystem (DeFi) system. In this token interface, we implemented an ERC-20 style token standard in both Motoko and Rust, the standard is named DIP20.

This standard allows for a common and familiar interface that not only provides a quick entry point for existing blockchain developers, but future interoperability options between the Internet Computer and Ethereum, through the process of sustaining the same shared interfaces.

You can find the interface descriptions in the [specification file](./spec.md).

[This branch](https://github.com/dfinance-tech/ic-token/tree/templates) contains code of several other token canister templates.


## Development

You need the latest DFINITY Canister SDK to be able to build and deploy a token canister:

```shell
sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
```

Navigate to a the sub directory and start a local development network:

```shell
cd motoko
dfx start --background
```

Create canisters:

```shell
dfx canister create --all
```

Install code for token canister:

```
dfx build

dfx canister install token --argument="(\"<LOGO>\", \"<NAME>\", \"<SYMBOL>\", <DECIMALS>, <TOTAL_SUPPLY>, <YOUR_PRINCIPAL_ID>, <FEE>)"
e.g.:
dfx canister install token --argument="(\"data:image/jpeg;base64,...\", \"DFinance Coin\", \"DFC\", 8, 10000000000000000, principal \"4qehi-lqyo6-afz4c-hwqwo-lubfi-4evgk-5vrn5-rldx2-lheha-xs7a4-gae\", 10000)"
```

Refer to `demo.sh` in the corresponding sub directory for more details.



## Contributing

We'd like to collaborate with the community to provide better token standard implementation for the developers on the IC, if you have some ideas you'd like to discuss, submit an issue, if you want to improve the code or you made a different implementation, make a pull request!

