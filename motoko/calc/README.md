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

You have downloaded and installed the SDK as described in [Getting started](https://sdk.dfinity.org/docs/developers-guide/getting-started.html).

### Demo

Start a local internet computer.

```bash
dfx start
```

Execute the following commands in another tab.

```bash
fx canister call calc add '(10)'
```

Observe the following result.

```bash
(+10)
```
