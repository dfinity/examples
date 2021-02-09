# Simple To-Do

![Compatibility](https://img.shields.io/badge/compatibility-0.6.23-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-simple-to-do-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-simple-to-do-example)

This example illustrates how to create a simple to-do checklist application.

## Introduction

The application is built from the following Motoko source code files:

*  `Utils.mo`, which contains the core functions for adding, completing, and
   removing to-do checklist items;

*  `Types.mo`, which contains the type definition of a to-do checklist item;
   and

*  `Main.mo`, which contains the actor definition and methods exposed by this
   canister.

## Prerequisites

Verify the following before running this demo:

*  You have downloaded and installed the [DFINITY Canister
   SDK](https://sdk.dfinity.org).

*  You have stopped any Internet Computer or other network process that would
   create a port conflict on 8000.

## Demo

1. Start a local internet computer.

   ```text
   dfx start
   ```

1. Open a new terminal window.

1. Reserve an identifier for your canister.

   ```text
   dfx canister create simple_to_do
   ```

1. Build your canister.

   ```text
   dfx build
   ```

1. Deploy your canister.

   ```text
   dfx canister install simple_to_do
   ```

1. Create a to-do checklist by invoking the `addTodo` method.

   ```text
   dfx canister call simple_to_do addTodo '("Create a project")'
   dfx canister call simple_to_do addTodo '("Build the project")'
   dfx canister call simple_to_do addTodo '("Deploy the project")'
   ```

1. Display the to-do checklist by invoking the `showTodos` method.

   ```text
   dfx canister call simple_to_do showTodos
   ```

1. Verify the return value matches what you would expect.

   ```text
   ("
   ___TO-DOs___
   (1) Create a project
   (2) Build the project
   (3) Deploy the project")
   ```

1. Complete a to-do checklist item by invoking the `completeTodo` method.

   ```text
   dfx canister call simple_to_do completeTodo '(1)'
   ```

1. Display the to-do checklist by invoking the `showTodos` method.

   ```text
   dfx canister call simple_to_do showTodos
   ```

1. Verify the return value matches what you would expect.

   ```text
   ("
   ___TO-DOs___
   (1) Create a project ✔
   (2) Build the project
   (3) Deploy the project")
   ```
