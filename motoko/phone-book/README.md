# Simple Phonebook

This example project illustrates how to create a simple phonebook web application using the Motoko programming language.

## Source code

This project is implemented using the following Motoko source code files:

- The `index.jsx` file contains the motoko that is compiled into the webapp index.js file loaded by your web browser.

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

1. Take note of the resultant containerId

    ```bash
    Installing code for canister phonebook, with canister_id ic:F5B3FD9D7854D55E3C
    ```

1. Open up your web browser to view the installed phonebook canister web page:

    http://localhost:8000/?canisterId=ic:F5B3FD9D7854D55E3C

1. Enter a Name, Description, Phone and click the Insert or Update button:

    Name: Information
    Description: Local Directory Assitance in the NANPA dialplan
    Phone: 411

1. Enter another Name, Description, Phone and click the Insert or Update button:

    Name: Emergency
    Description: Local Emergency Services in the NANPA dialplan
    Phone: 911

1. Now enter "Information" into the Lookup Name field and click the Lookup button:

    Lookup Name: Information

1. Add additional phonebook records by using the Candid interface to call the `insert` function or by running the following command:

    ```bash
    dfx canister call phonebook insert '("TRS","Assistive Telecommunications Relay Service in the NANPA dialplan",711)'
    ```

1. Verify the updated phonebook checklist by using the Candid interface to call the 'lookup' function or by running the following command:  

    ```bash
    dfx canister call phonebook lookup '("TRS")'
    ```

    The function returns a checklist similar to the following:

    ```bash
    (opt record { 1224700491 = "TRS"; 1595738364 = "Assistive Telecommunications Relay Service in the NANPA dialplan"; 3253977966 = 711; })
    ```
