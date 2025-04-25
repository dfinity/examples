---
keywords: [static website, basic website, html, host a website, beginner]
---

# Static website

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/hosting/static-website)

The example shows how to deploy a simple, static website hosted on ICP. The website is very simple; it just displays the DFINITY logo. While the website in this example is very simple, the method would be the same for a more advanced static website, e.g., based on popular static site generators.

![Website](README_images/website.png)

## Prerequisites

This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/).

- [x] Download and install [git](https://git-scm.com/downloads).

- [x] Download the example project: `git clone https://github.com/dfinity/examples && cd examples/hosting/static-website`

## Website

The website consists of an HTML file, a CSS file, and a PNG file.

```html
<!doctype html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width">
        <title>Static Website</title>
        <base href="/">
        <link type="text/css" rel="stylesheet" href="styles.css" />
    </head>
    <body>
        <img src="logo.png" alt="DFINITY logo" />
    </body>
</html>

```

The only styling done in the CSS file is aligning the logo image:

```css
img {
    max-width: 50vw;
    max-height: 25vw;
    display: block;
    margin: auto;
}
```

The project folder will then look like this:

```
  static-website
  ├── dfx.json
  └── frontend
      ├── assets
      │   ├── logo.png
      │   └── main.css
      └── src
          └── index.html
```
## `dfx.json`

The `dfx.json` file is a configuration file that specifies the canister used for the dapp. In this case only one canister is needed. Besides the canister configuration, `dfx.json` also includes information about the project's build settings.

```json
{
    "canisters": {
        "frontend": {
            "frontend": {
                "entrypoint": "frontend/src/index.html"
            },
            "source": [
                "frontend/assets",
                "frontend/src"
            ],
            "type": "assets"
        }
    },
    "defaults": {
        "build": {
            "args": "",
            "packtool": ""
        }
    },
    "version": 1
}
```

This is all needed for creating a canister smart contract for hosting a static website on ICP.

## Local deployment

The local replica is started by running:

```bash

dfx start --background

```

When the local replica is up and running, run this command to deploy the canisters:

```bash

dfx deploy

```

The URL for the frontend depends on the canister ID.  Local canister IDs can be obtained using `dfx canister id`, in this case `dfx canister id www`. When deployed, the URL will look like this:

```

http://{canister_id}.localhost:4943

```

![Candid UI](README_images/website.png)

## Deploying to the mainnet 

Deploy your static website dapp to the mainnet with the command:

```

dfx deploy --network=ic

```

Deploying to the mainnet will cost cycles. [Learn more about how to get cycles.](https://internetcomputer.org/docs/building-apps/getting-started/tokens-and-cycles)

You can set your website to be hosted at a custom domain, such as `https://mywebsite.com`. [Learn more about custom domains.](https://internetcomputer.org/docs/building-apps/frontends/custom-domains/using-custom-domains)

## License

This project is licensed under the Apache 2.0 license, see `LICENSE.md` for details. See `CONTRIBUTE.md` for details about how to contribute to this project. 

