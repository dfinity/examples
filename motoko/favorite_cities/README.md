# Favorite cities

This program provides functions that return multiple city names and illustrates how to use a type enclosed by square (`[ ]`) brackets to indicate an *array* of that type.
In this example, the `[Text]` notation indicates an array of a collection of UTF-8 characters, enabling the program to accept and return multiple text strings.

### Prerequisites

You have downloaded and installed the SDK as described in [Getting started](https://sdk.dfinity.org/developers-guide/getting-started.html).

### Demo

Start a local internet computer.

```bash
dfx start
```

Execute the following commands in another tab.

```bash
dfx build
dfx canister install --all
dfx canister call favorite_cities location_pretty '(vec {"San Francisco";"Paris";"Rome"})'
```

Observe the following result.
```bash
("Hello from San Francisco, Paris, Rome, bon voyage!")
```
