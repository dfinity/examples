![Compatibility](https://img.shields.io/badge/compatibility-0.6.10-blue)

# Simple To-do checklist

The sample project illustrates how to create a simple to-do checklist application using the Motoko programming language.

## Source code

This project is implemented using the following Motoko source code files:

- The `utils.mo` file contains the core functions for adding, completing, and removing to-do items.

- The `types.mo` file defines the properties for ToDo items as a custom data type definition.

- The `main.mo` file contains the actor definition and public functions that the compiled canister exposes.

## Prerequisites

Before building the example application, verify the following:

* You have downloaded and installed the DFINITY Canister SDK as described in [Download and install](https://sdk.dfinity.org/docs/quickstart/quickstart.html#download-and-install).
* You have stopped any Internet Computer network processes running on the local computer.

## Demo

1. Start the Internet Computer network locally by running the following command:

    ```bash
    dfx start
    ```

1. Open a new terminal shell and navigate to the project directory.

1. Register identifiers for the project by running the following command:

    ```bash
    dfx canister create simple_to_do
    ```

1. Build the project by running the following command:

    ```bash
    dfx build
    ```

1. Deploy the project by running the following command:

    ```bash
    dfx canister install simple_to_do
    ```

1. Create a to-do checklist by using the Candid interface to call the `addTodo` function or by running the following commands:

    ```bash
    dfx canister call simple_to_do addTodo '("Create a project")'
    dfx canister call simple_to_do addTodo '("Build the project")'
    dfx canister call simple_to_do addTodo '("Deploy the project")'
    ```

1. Display the to-do checklist by using the Candid interface to call the `showTodos` function or by running the following command:

    ```bash
    dfx canister call simple_to_do showTodos
    ```

1. Verify the function returns a checklist similar to the following:

    ```bash
    ("
    ___TO-DOs___
    (1) Create a project
    (2) Build the project
    (3) Deploy the project")
    ```

1. Complete a checklist item by using the Candid interface to call the `completeTodo` function or by running the following command:

    ```bash
    dfx canister call simple_to_do completeTodo '(1)'
    ```

1. Verify the updated to-do checklist by running the following command:

    ```bash
    dfx canister call simple_to_do showTodos
    ```

    The function returns a checklist similar to the following:

    ```bash
    ("
    ___TO-DOs___
    (1) Create a project ✔
    (2) Build the project
    (3) Deploy the project")
    ```
