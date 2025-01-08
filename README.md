# Internet Computer sample applications 

Get started building on ICP with the sample applications in this repository. From this repository, you can deploy, download, clone, fork, or share sample projects.

The projects in this repository are not intended to be used as commercial applications and do not provide any explicit or implied support or warranty of any kind.

You can also contribute your own project or suggest updates to published projects using the standard GitHub workflow.

## Sample applications

Code samples are organized by programming language:

- [Motoko](https://github.com/dfinity/examples/tree/master/motoko)
- [Rust](https://github.com/dfinity/examples/tree/master/rust)
- [C](https://github.com/dfinity/examples/tree/master/c)

Some examples include frontends written in a variety of frameworks, such as React, JavaScript, etc. 

Additional frontend samples can be found in the following folders:

- [Svelte](https://github.com/dfinity/examples/tree/master/svelte)
- [HTML](https://github.com/dfinity/examples/tree/master/hosting)
- [Unity](https://github.com/dfinity/examples/tree/master/native-apps)

## Deploying samples 

### GitHub Codespaces or Gitpod

This repo can be opened in a web-based developer environment such as [GitHub Codespaces](https://github.com/codespaces) or [Gitpod](https://www.gitpod.io/), allowing you to edit and deploy the sample projects without downloading any tools or setting up a local environment. 

[Get started with GitHub codespaces](https://internetcomputer.org/docs/current/developer-docs/developer-tools/ide/codespaces).

[Get started with Gitpod](https://internetcomputer.org/docs/current/developer-docs/developer-tools/ide/gitpod).

### Motoko Playground

Motoko Playground is a web-based developer environment for Motoko projects. To use Motoko Playground, navigate to the [playground UI](https://m7sm4-2iaaa-aaaab-qabra-cai.ic0.app/) and select a template to get started, or start a new project.

### dfx 

dfx is a command-line tool used to create, deploy. and manage projects on ICP. To download and use dfx with this examples repo, run the following commands locally (macOS/Linux systems):

```
git clone https://github.com/dfinity/examples.git
cd examples
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```

Then, navigate into the folder of the sample that you want to use and follow the project's README instructions to setup and deploy the sample code.


## Resources 

- [ICP Developer Docs](https://internetcomputer.org/docs/current/home)

- [Overview of ICP](https://internetcomputer.org/docs/current/developer-docs/getting-started/overview-of-icp)

- [Installing dfx](https://internetcomputer.org/docs/current/developer-docs/getting-started/install/)

- [Developer tools](https://internetcomputer.org/docs/current/developer-docs/developer-tools/dev-tools-overview)

## Security considerations and best practices

If you base your application on one of these examples, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. The examples provided here may not implement all the best practices.
