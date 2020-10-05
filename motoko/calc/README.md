# Calc

The program uses the `+cell+` variable to contain an integer number that represents the current result of a calculator operation.

This program supports the following types of function calls:

* The `add` function call accepts input and performs addition.
* The `sub` function call accepts input and performs subtraction.
* The `mul` function call accepts input and performs multiplication.
* The `div` function call accepts input and performs division.
* The `clearall` function clears the `cell` value stored as the result of previous operations, resetting the `cell` value to zero.

The `div` function also includes code to prevent the program from attempting to divide by zero.

### Prerequisites

Before building the example application, verify the following:

* You have downloaded and installed the DFINITY Canister SDK as described in [Download and install](https://sdk.dfinity.org/docs/quickstart/quickstart.html#download-and-install).
* You have stopped any Internet Computer network processes running on the local computer.

### Demo

Start a local internet computer.

```bash
dfx start
```

Execute the following commands in another tab.

```bash
dfx canister call calc add '(10)'
```

Observe the following result.

```bash
(+10)
```
