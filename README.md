## DFINITY Examples for the Internet Computer

This repository provides access to sample code, applications, and microservices that run on the Internet Computer platform.

From this repository, you can download, clone, fork, or share sample projects.
<!-- You will also be able to contribute your own project or suggest updates to published projects using the standard Git work flow. --> 

Sample projects provide a way for you to experiment and collaborate with other developers.
The projects and sample code are not, however, intended to be used as commercial applications and do not provide any explicit or implied support or warranty of any kind.

To get started:

1. Open a terminal shell on your local computer.

1. Download the DFINITY Canister SDK, if needed:

    ```
    sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
    ```

1. Select a language—for example, `c` or `motoko`—to explore the examples available in the language of your choice.

1. Clone the repository to download the `examples` repo to your local workspace.

1. Open the project folder for the example you want to use.

1. Start a local internet computer replica by running the following command:

    ```
    dfx start
    ```

1. Open a new terminal shell on your local computer.

1. Build the sample project by running the following command:

    ```
    dfx build
    ```

## Security Considerations and Security Best Practices

If you base your application on one of these examples, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. The examples provided here may not implement all the best practices.