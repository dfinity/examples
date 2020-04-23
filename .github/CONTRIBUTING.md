# Contributing

Thank you for your interest in contributing to documentation for the Internet Computer.
By participating in this project, you agree to abide by our [Code of Conduct](./CODE_OF_CONDUCT.md).

As a member of the community, you are invited and encouraged to contribute by submitting issues, offering suggestions for improvements, adding review comments to existing pull requests, or creating new pull requests to fix issues.

All contributions to DFINITY documentation and the developer community are respected and appreciated.
Your participation is an important factor in the success of the Internet Computer.

## Contents of this repository

This repository contains the documentation source files for the DFINITY Canister Software Development Kit (SDK).
The files use AsciiDoctor markup language.

This repository also contains the `antora.yml` configuration file that lists the documentation components (modules) and versions included in the [documentation website](https://sdk.dfinity.org).

The navigation for all modules—including documentation for modules located in other repositories—is stored in the under the ROOT module in the `nav.adoc` file.

The [documentation website](https://sdk.dfinity.org) is generated using the [Antora](https://docs.antora.org/antora/2.2/install/install-antora/) static site generator.

The `antora-playbook.yml` identifies the repository branches and UI bundle to use in building the documentation site.
The playbook is located in a separate repository.

To build the documentation site locally, you must have:

- Antora installed locally.
- The sources files in this directory
- Either the `antora-playbook.yml` playbook or a locally-defined playbook.
- Either the UI bundle defined in the `antora-playbook.yml` playbook, the default Antora UI bundle, or a locally-defined UI bundle.

For details about [installing Antora](https://docs.antora.org/antora/2.2/install/install-antora/) and using the [default Antora UI](https://docs.antora.org/antora/2.2/playbook/configure-ui/), see the Antora documentation.

## Before you contribute

Before contributing, please take a few minutes to review these contributor guidelines.
The contributor guidelines are intended to make the contribution process easy and effective for everyone involved in addressing your issue, assessing changes, and finalizing your pull requests.

Before contributing, consider the following:

- If you want to report an issue or request help, click **Issues**.

    You can also post a message to the [community forum](https://forum.dfinity.org/) or submit a [support request](mailto://support@dfinity.org).

- If you are reporting a bug, provide as much information about the problem
as possible, including the SDK version.

- If you want to contribute directly to this repository, typical fixes might include any of the following:

    - Spelling, grammar, or typo fixes.
    - Code indentation, white space, or formatting changes.
    - Broken or missing links.

    Note that any contribution to this repository must be submitted in the form of a **pull request**.

- If you are creating a pull request, be sure that the pull request only implements one bug fix.

If you are new to working with GitHub repositories and creating pull requests, consider exploring [First Contributions](https://github.com/firstcontributions/first-contributions) or [How to Contribute to an Open Source Project on GitHub](https://egghead.io/courses/how-to-contribute-to-an-open-source-project-on-github).

# How to make a contribution

Here's a summary of what you need to do:

1. Make sure you have a GitHub account, an internet connection, and access to a terminal shell or GitHub Desktop application for running commands.

1. Navigate to the DFINITY public repository in a web browser.

1. Click **Fork** to create a copy the repository associated with the issue you want to address under your GitHub account or organization name.

1. Clone the repository to your local machine.

1. Create a new branch for your fix by running a command similar to the following:

    ```
    git checkout -b my-branch-name-here
    ```

1. Open the file you want to fix in a text editor and make the appropriate changes for the issue you are trying to address.

1. Add the file contents of the changed files to the index `git` uses to manage the state of the project by running a command similar to the following:

    ```
    git add path-to-changed-file
    ```
1. Commit your changes to store the contents you added to the index along with a descriptive message by running a command similar to the following:

    ```
    git commit -m "Description of the fix being committed."
    ```

1. Push the changes to the remote repository by running a command similar to the following:

    ```
    git push origin my-branch-name-here
    ```

1. Create a new pull request for the branch you pushed to the upstream GitHub repository.

    Provide a title that includes a short description of the changes made.

1. Wait for the pull request to be reviewed.

1. Make changes to the pull request, if requested.

1. Celebrate your success after your pull request is merged!

## Tips for contributing to documentation

Depending on the type of contribution you want to make, you might follow a different workflow.
For example, if you are only interested in reporting an issue, there's no need to clone repository or set up a documentation environment.

This section describes the most common workflow scenarios:

- Reporting an issue
- Making simple edits
- Making large or ongoing contributions

### Reporting an issue

To open a new issue:

1. Click **Issues**.

1. Click **New Issue**.

1. Click **Open a blank issue**.

1. Type a title and description, then click **Submit new issue**.

    Be as clear and descriptive as possible.

    For any problem, describe it in detail, including details about the version of the code you are using, the results you expected, and how the actual results differed from your expectations.

### Making simple edits

For simple changes, like fixing a typo or making minor changes to a sentence:

1. Click the **Edit** icon on the top right of the documentation page.

1. Edit the page in GitHub.

1. Replace the default commit message with a short description of your change, then click **Commit Changes**.

1. Follow the instructions to open a pull request.

### Making large or ongoing contributions

To make large or ongoing contributions to the documentation, you will mostly likely need to set up a complete documentation environment.

To set up a documentation environment:

1. Create a GitHub account.
1. Download and install a Git client
1. Install an AsciiDoc or AsciiDoctor editor.
1. Fork then clone repository.
1. Connect your forked local repository to the upstream repository.

    ```
    git remote add upstream git@github.com:dfinity/docs.git
    ```

1. Download upstream changes.

    ```
    git fetch upstream
    ```

1. Create a working branch.
1. Fix the issue.
1. Install Antora.
1. Create a local copy of the `antora-playbook.yml` file.
1. Configure the playbook `sources.url` setting to use your forked local repository and your working branch or `HEAD`.
1. Run Antora with your local playbook to generate the site locally.

    ```
    antora local-antora-playbook.yml
    ```

1. Open the site directly from the default output directory by running the following command:

    ```
    open build/site/docs/index.html
    ```
