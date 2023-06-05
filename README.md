# DFINITY examples for the Internet Computer

This repository provides access to sample code, applications, and microservices that run on the Internet Computer platform.

From this repository, you can download, clone, fork, or share sample projects.
<!-- You will also be able to contribute your own project or suggest updates to published projects using the standard Git work flow. --> 

Sample projects provide a way for you to experiment and collaborate with other developers while exploring different use-cases or workflows.
The projects and sample code are not, however, intended to be used as commercial applications and do not provide any explicit or implied support or warranty of any kind.

To get started:

1. Open a terminal shell on your local computer.

1. Download the DFINITY IC SDK, if needed:

    ```
    sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
    ```

1. Clone this repository to download the `examples` repo to your local workspace.

1. Select a language—for example, `motoko` or `rust`—to explore the examples available in the language of your choice.

1. Open the project folder for the example you want to use.

1. Start a local Internet Computer replica by running the following command:

    ```
    dfx start
    ```

1. Then, open a new terminal shell on your local computer.

1. Build the sample project by running the following command:

    ```
    dfx deploy
    ```

## Security considerations and security best practices

If you base your application on one of these examples, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. The examples provided here may not implement all the best practices.
