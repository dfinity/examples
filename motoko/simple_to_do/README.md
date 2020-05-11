# Simple To-do checklist

The sample project illustrates how to create a simple to-do checklist application using the Motoko programming language.

## Source code

This project is implemented using the following Motoko source code files:

- The `utils.mo` file contains the core functions for adding, completing, and removing to-do items.

- The `types.mo` file defines the properties for ToDo items as a custom data type definition.

- The `main.mo` file contains the actor definition and public functions that the compiled canister exposes.

## Prerequisites

You have downloaded and installed the SDK as described in [Getting started](https://sdk.dfinity.org/developers-guide/getting-started.html).

## Demo

1. Start a local internet computer replica by running the following command:

    ```bash
    dfx start
    ```

1. Open a new terminal shell and navigate to the project directory.

1. Build the project by running the following command:

    ```bash
    dfx build
    ```

1. Deploy the project by running the following command:

    ```bash
    dfx canister install --all
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