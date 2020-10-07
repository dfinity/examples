![Compatibility](https://img.shields.io/badge/compatibility-0.6.10-blue)

## Simple Phonebook

This example project illustrates how to create a simple phonebook web application using the Motoko programming language.

## Source code

This project is implemented using the following source code files:

- The `main.mo` file contains the actor definition and public functions that the compiled canister exposes.

- The `index.jsx` file contains the JavaScript, React, and HTML used to generate the front-end user interface for the application when it is launched in a web browser.


## Prerequisites

Before building the example application, verify the following:

* You have downloaded and installed the DFINITY Canister SDK as described in [Download and install](https://sdk.dfinity.org/docs/quickstart/quickstart.html#download-and-install).
* You have stopped any Internet Computer network processes running on the local computer.

## Demo

1. Start a local internet computer replica by running the following command:

    ```bash
    dfx start
    ```

1. Open a new terminal shell and navigate to the project directory.

1. Build the project by running the following command:

    ```bash
    dfx canister create --all
    dfx build
    ```

1. Deploy the project by running the following command:

    ```bash
    dfx canister install --all
    ```

1. Take note of the resultant canisterId

    ```bash
    Installing code for canister phonebook_ui, with canister_id cxeji-wacaa-aaaaa-aaaaa-aaaaa-aaaaa-a
    ```

1. Open up your web browser to view the installed phonebook canister web page:

    http://localhost:8000/?canisterId=cxeji-wacaa-aaaaa-aaaaa-aaaaa-aaaaa-a

1. Enter a Name, Description, Phone and click the Insert or Update button:

    Name: Information
    Description: Local Directory Assistance in the NANPA dialplan
    Phone: 411

    NOTE: the Name and Description are Text, and the Phone number is a Nat (number). Entering a non-number into the Phone field will cause the application to silently fail on the browser side. You will only see this failure in your browser's javascript console.

1. Enter another Name, Description, Phone and click the Insert or Update button:

    Name: Emergency
    Description: Local Emergency Services in the NANPA dialplan
    Phone: 911

1. Now enter "Information" into the Lookup Name field and click the Lookup button:

    Lookup Name: Information

1. Add additional phonebook records by calling the `insert` function using the Candid endpoint or by running the following command:

    ```bash
    dfx canister call phonebook insert '("TRS","Assistive Telecommunications Relay Service in the NANPA dialplan",711)'
    ```

1. Verify the updated phonebook checklist by calling the 'lookup' function using the Candid endpoint or by running the following command:

    ```bash
    dfx canister call phonebook lookup '("TRS")'
    ```

    The function returns output similar to the following:

    ```bash
        (
        opt record {
            name = "TRS";
            description = "Assistive Telecommunications Relay Service in the NANPA dialplan";
            phone = 711;
        },
        )
    ```
