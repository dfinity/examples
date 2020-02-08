const dfxJson = require("./dfx.json");
const fs = require('fs');
const path = require("path");
const TerserPlugin = require("terser-webpack-plugin");

// Identify dfx output directory.
const output = ["defaults", "build", "output"].reduce(function (accum, x) {
  return accum && accum[x] ? accum[x] : null
}, dfxJson) || "build";
console.log("dfx output directory = " + output);

// Identify dfx version.
const versions = fs.readdirSync(
  path.join(process.env["HOME"], ".cache/dfinity/versions")
);
const custom = process.env["DFX_VERSION"];
const latest = versions.map(function (version) {
  const chunks = version.split('-');
  const prefix = chunks[0].split('.').map(s => parseInt(s));
  const suffix = chunks[1] == null ? 0 : parseInt(chunks[1]);
  return [prefix.concat(suffix).map(n => 1000000 + n).join(), version];
}).sort().slice(-1)[0][1];
const version = custom ? custom : latest;
console.log("dfx version = " + version);

// List of all aliases for canisters. This creates the module alias for
// the `import ... from "ic:canisters/xyz"` where xyz is the name of a
// canister.
const aliases = Object.entries(dfxJson.canisters).reduce((acc, [name,]) => {
  const outputRoot = path.join(__dirname, output, name);
  return {
    ...acc,
    ["ic:canisters/" + name]: path.join(outputRoot, "main.js"),
    ["ic:idl/" + name]: path.join(outputRoot, "main.did.js"),
  };
}, {
  // This will later point to the userlib from npm, when we publish the userlib.
  "ic:userlib": path.join(
    process.env["HOME"],
    ".cache/dfinity/versions",
    version,
    "js-user-library/dist/lib.prod.js",
  ),
});

/**
 * Generate a webpack configuration for a canister.
 */
function generateWebpackConfigForCanister(name, info) {
  if (typeof info.frontend !== 'object') {
    return;
  }
  const outputRoot = path.join(__dirname, output, name);
  const inputRoot = __dirname;
  const entry = path.join(inputRoot, info.frontend.entrypoint);
  return {
    mode: "production",
    entry,
    devtool: "source-map",
    optimization: {
      minimize: true,
      minimizer: [new TerserPlugin()],
    },
    resolve: {
      alias: aliases,
    },
    module: {
      rules: [
        { test: /\.(js|ts)x?$/, loader: "ts-loader" }
      ]
    },  
    output: {
      filename: "index.js",
      path: path.join(outputRoot, "assets"),
    },
    plugins: [
    ],
  };
}

// If you have webpack configurations you want to build as part of this
// config, add them here.
module.exports = [
  ...Object.entries(dfxJson.canisters).map(([name, info]) => {
    return generateWebpackConfigForCanister(name, info);
  }).filter(x => !!x),
];
