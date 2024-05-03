---
keywords: [intermediate, motoko, cert var, certified variables]
---

# Certified variables

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/cert-var)

## Overview
This example demonstrates the use of a single cryptographically certified variable, as supported by the Internet Computer.

In a nutshell, this example code demonstrates "response certification" for a canister that holds a single 32-bit variable. It has two sides:

- Backend (BE) canister logic in Motoko (`main.mo`).
- Frontend (FE) logic in JS (`index.js`).

To detect an attacker in the middle between the FE and the IC and our "true" BE canister running there, we must either:

- Perform update calls that use "full consensus" (and wait for ~2 sec).
- Perform (fast) query calls whose responses that we, the client, certify, using the coordination of the IC and our canister running there.

The FE and BE code demonstrates the second approach in a minimal setting. The BE holds a single certified variable, as a 32-bit number, and the FE code queries and certifies this number's "current certificate". The BE prepares for the FE certification by giving the FE a "current certificate" within the response; this certificate is signed by the entire IC, using a special system feature.

Before the FE trusts the response from the apparent BE canister, it interrogates it and verifies its authenticity, the FE does four checks:

- Verify system certificate.
- Check system certificate timestamp is not "too old".
- Check the canister ID in the system certificate.
- Check that the response matches the witness.

For steps 2, 3, and 4, the FE accesses data from the certificate (Blob).

The `Certificate` class from the `agent-js` library provides a way to access those items using their paths, like a filesystem, each addressing a Blob, encoding something.

In the case of time and our data, the encodings are each Candid. The IC spec represents time using a LEB128 encoding, and certified data uses little endian.

Ideally, we should use a proper library to decode these numbers. To prevent an extra dependency, we take advantage of the fact that the Candid value encoding of Nat and Nat32 use the same representation.

The data encoded is the same as a Candid 32-bit Nat (little endian -- see the Motoko canister for details).

Notably, in an example with more data in the canister than a single number, or a more complex query interface, we would generally do more work to certify each query response:

- Use witnesses to re-calculate hash (no witness or hashing needed here.)
- Check query parameters match witness (no params, so trivial here.)
- Neither of those steps is needed here, for the reasons given above.

This is a Motoko example that does not currently have a Rust variant. 

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Download [npm](https://nodejs.org/en/download/).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

Begin by opening a terminal window.

### Step 1: Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the command:

```bash
cd examples/motoko/cert-var
dfx start --background
```

### Step 2: Install the front-end dependencies:

```bash
npm install
```

### Step 3: Deploy the canister:

```bash
dfx deploy
```

### Step 4: Next, open the `webpack.config.js` file and replace the contents with the following:

```javascript
const path = require("path");
const webpack = require("webpack");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const TerserPlugin = require("terser-webpack-plugin");

let localCanisters, prodCanisters, canisters;

try {
  localCanisters = require(path.resolve(".dfx", "local", "canister_ids.json"));
} catch (error) {
  console.log("No local canister_ids.json found. Continuing production");
}

function initCanisterIds() {
  try {
    prodCanisters = require(path.resolve("canister_ids.json"));
  } catch (error) {
    console.log("No production canister_ids.json found. Continuing with local");
  }

  const network =
    process.env.DFX_NETWORK ||
    (process.env.NODE_ENV === "production" ? "ic" : "local");

  canisters = network === "local" ? localCanisters : prodCanisters;

  for (const canister in canisters) {
    process.env[canister.toUpperCase() + "_CANISTER_ID"] =
      canisters[canister][network];
  }
}
initCanisterIds();

const isDevelopment = process.env.NODE_ENV !== "production";
const asset_entry = path.join(
  "src",
  "cert_var_assets",
  "src",
  "index.html"
);

module.exports = {
  target: "web",
  mode: isDevelopment ? "development" : "production",
  entry: {
    // The frontend.entrypoint points to the HTML file for this build, so we need
    // to replace the extension to `.js`.
    index: path.join(__dirname, asset_entry).replace(/\.html$/, ".js"),
  },
  devtool: isDevelopment ? "source-map" : false,
  optimization: {
    minimize: !isDevelopment,
    minimizer: [new TerserPlugin()],
  },
  resolve: {
    extensions: [".js", ".ts", ".jsx", ".tsx"],
    fallback: {
      assert: require.resolve("assert/"),
      buffer: require.resolve("buffer/"),
      events: require.resolve("events/"),
      stream: require.resolve("stream-browserify/"),
      util: require.resolve("util/"),
    },
  },
  output: {
    filename: "index.js",
    path: path.join(__dirname, "dist", "cert_var_assets"),
  },

  // Depending in the language or framework you are using for
  // front-end development, add module loaders to the default
  // webpack configuration. For example, if you are using React
  // modules and CSS as described in the "Adding a stylesheet"
  // tutorial, uncomment the following lines:
  // module: {
  //  rules: [
  //    { test: /\.(ts|tsx|jsx)$/, loader: "ts-loader" },
  //    { test: /\.css$/, use: ['style-loader','css-loader'] }
  //  ]
  // },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.join(__dirname, asset_entry),
      cache: false
    }),
    new webpack.EnvironmentPlugin({
      NODE_ENV: 'development',
      CERT_VAR_CANISTER_ID: canisters["cert_var"]
    }),
    new webpack.ProvidePlugin({
      Buffer: [require.resolve("buffer/"), "Buffer"],
      process: require.resolve("process/browser"),
    }),
  ],
  // proxy /api to port 8000 during development
  devServer: {
    proxy: {
      "/api": {
	static: './',
        target: "http://localhost:8000",
        changeOrigin: true,
        pathRewrite: {
          "^/api": "/api",
        },
      },
    },
    hot: true,
  },
};
```

### Step 5: Create a new file called `server.js` with the following content:

```javascript
var express = require('express');
var app = express();

app.get('/', function (req, res) {
  res.send('Hello World!'); // This will serve your request to '/'.
});

app.listen(8000, function () {
  console.log('Example app listening on port 8000!');
 });
```

### Step 6: Replace the content of the `src/cert_var_assets/src/index.html` with the following content:

```html
<!doctype html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width">
    <title>Certified Variables</title>
    <base href="/">

    <link type="text/css" rel="stylesheet" href="main.css" />
</head>
<body style="background-color:powderblue; text-align:center;">
  <div>
    <img src="https://global.discourse-cdn.com/business4/uploads/dfn/original/1X/a6d6c5b4e246cd075a009424601bc981b3086fb4.png" alt="DFINITY logo" />
      <div> <label for="name">New value of variable</label>
        <input id="newValue" alt="New Value" type="number" />
        <button id="setBtn">Set and get!</button>
      </div>
      <pre id="var" style="line-height:1;"></pre>
    </div>
   </div>
</body>
</html>
```

### Step 7: Start a local web server that hosts the frontend.

```bash
npm start
```


### Step 8: Visit the frontend, and interact with the demo there:

```bash
http://localhost:8080/
```

This should present an entry for "New value of variable", and a button to "Set and get!".

Enter a number and click the button.

The canister updates its certificate, and the frontend checks it. The developer console contains some additional comments about each step.


## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:
* [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since this app is all about response certification!
* [Validate Inputs](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices/#validate-inputs), since for incrementing the nat32 variable, the argument the inc call may be too big for the addition to be possible. 
