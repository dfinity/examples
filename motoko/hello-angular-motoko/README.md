# hello-angular-motoko
This is an example of locally implementing the Internet Computer's motoko project along side Angular

&nbsp;

---


## Getting started

1. [Install DFX](https://sdk.dfinity.org/docs/quickstart/local-quickstart.html). Please keep in mind the dfx cli currently only runs on Linux and Apple based PCs.
1. [Install Angular](https://angular.io/guide/setup-local)
1. Install npm packages from the project root: 

    `npm install`

&nbsp;

---
### Serve Angular and the IC backend server locally with live reloading:
    
    npm run ng-start
    
- Once the job fully starts, your application will be available at `http://localhost:4200`.

- If you make any changes while in development mode, the project will recompile on both the Angular and IC / Motoko side.
    
&nbsp;

---
### Build and Serve the Angular application on the local IC server:

    npm run build-dfx-www

&nbsp;

---

### package.json Scripts
    npm run [script name]
| Name | Description |
| ----------- | ----------- |
| ng-start | Start the Angular and the IC server locally with live reloading |
| ng-serve | Serve and watch both Angular and the IC server for changes |
| ng-start-dfx | Stop IC server when needed and re-start the IC server |
| build-dfx-www | Build and Serve the Angular application on the local IC server
| print-dfx-www | Print the url to the local IC server Angular application
&nbsp;

---

### To learn more about working with dfx, see the following documentation available online:

- [Quick Start](https://sdk.dfinity.org/docs/quickstart/quickstart-intro.html)
- [SDK Developer Tools](https://sdk.dfinity.org/docs/developers-guide/sdk-guide.html)
- [Motoko Programming Language Guide](https://sdk.dfinity.org/docs/language-guide/motoko.html)
- [Motoko Language Quick Reference](https://sdk.dfinity.org/docs/language-guide/language-manual.html)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.ic0.app)
