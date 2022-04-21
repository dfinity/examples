const path = require("path");
const webpack = require("webpack");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

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

const index_html = path.join(__dirname, path.join("webapp", "index.html"));
const index_js = path.join(__dirname, path.join("webapp", "index.js"));

module.exports = {
  entry: { index: index_js },

  mode: "production",

  output: {
    filename: "index.js",
    path: path.join(__dirname, "dist"),
  },

  plugins: [
    // This loads index.html as template, and embeds index.js
    new HtmlWebpackPlugin({
      template: index_html,
      cache: false,
    }),

    new webpack.EnvironmentPlugin({
      WHOAMI_CANISTER_ID: process.env.WHOAMI_CANISTER_ID,
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
    static: "./webapp",
  },
};

