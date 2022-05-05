const path = require("path");
const webpack = require("webpack");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const fs = require('fs');

let canisters;

function initCanisterIds() {
  if ((dfx_network = process.env.DFX_NETWORK)) {
    network = dfx_network;
    console.log(`network was inferred from environment variable DFX_NETWORK`);
  } else {
    network = process.env.NODE_ENV === "production" ? "ic" : "local";
    console.log(
      `environment variable DFX_NETWORK not set, inferring network from node environment`
    );
  }

  network_alphanum = network.replace(/[^a-zA-Z0-9]/g, "_"); // replace non-alphanumeric like dfx
  console.log(`network is '${network}' (${network_alphanum})`);

  function getCanisterIds(path) {
    try {
      return require(path);
    } catch (error) {
      console.log(
        `No canister_ids.json found for network ${network} (${path}), try a different network..`
      );
      throw error;
    }
  }

  canisters =
    network === "ic"
      ? getCanisterIds(path.resolve("canister_ids.json"))
      : getCanisterIds(
          path.resolve(".dfx", network_alphanum, "canister_ids.json")
        );

  for (const canister in canisters) {
    process.env[canister.toUpperCase() + "_CANISTER_ID"] =
      canisters[canister][network_alphanum];
  }
}
initCanisterIds();

const files = ["index", "dashboard", "receive"];

function get_src_path(file) {
  return path.join(__dirname, path.join("src", "bitcoin_wallet_assets", "src", file));
}

var entries = {};

files.forEach(file => {
  entries[file] = get_src_path(file)
});

module.exports = {
  entry: entries,

  mode: "production",

  output: {
    filename: "[name].js",
    path: path.join(__dirname, "dist", "bitcoin_wallet_assets"),
  },

  plugins: [
    // This loads HTML files with common.html as template.
    ...files.map((filename) => {
      const file = filename + ".html";
      return new HtmlWebpackPlugin({
        filename: file,
        template: get_src_path("common.html"),
        body: fs.readFileSync(get_src_path(file)),
        cache: false,
        chunks: [filename]
      })
    }),

    new webpack.EnvironmentPlugin({
      BITCOIN_WALLET_CANISTER_ID: process.env.BITCOIN_WALLET_CANISTER_ID,
      II_CANISTER_ID: process.env.INTERNET_IDENTITY_CANISTER_ID,
      DFX_NETWORK: process.env.DFX_NETWORK || "local",
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
        target: "http://localhost:8000",
        changeOrigin: true,
        pathRewrite: {
          "^/api": "/api",
        },
      },
    },
    hot: true,
    static: "./src/bitcoin_wallet_assets/src/",
  },
};
