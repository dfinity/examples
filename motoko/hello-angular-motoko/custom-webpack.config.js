const path = require("path");
const webpack = require("webpack");

const isDevelopment = process.env.NODE_ENV !== "production";
const localCanisterHost = 'http://127.0.0.1:8000';
const productionCanisterHost = 'https://ic0.app';
const network = process.env.DFX_NETWORK || (process.env.NODE_ENV === "production" ? "ic" : "local");
let localCanisters, prodCanisters, canisters, canisterHost;

console.log("--- starting custom-webpack.config.js ---");

function initCanisterIds() {
  
  try {
    localCanisters = require(path.resolve(".dfx", "local", "canister_ids.json"));
  } catch (error) {
    console.log("No local canister_ids.json found. Continuing production");
  }

  try {
    prodCanisters = require(path.resolve("canister_ids.json"));
  } catch (error) {
    console.log("No production canister_ids.json found. Continuing with local");
  }

  console.log("network = ",network );

  canisters = isDevelopment ? localCanisters : prodCanisters;

  for (const canister in canisters) {
    let currentCanster = canisters[canister][network];
    process.env[canister.toUpperCase() + "_CANISTER_ID"] = currentCanster;
    console.log('canister:', canister, currentCanster);
  }
  
}
initCanisterIds();

function initCanisterHost() {
  canisterHost = isDevelopment ? localCanisterHost : productionCanisterHost;
}
initCanisterHost();
//for @angular-builders/custom-webpack
//just the Plugins and configs needed for the IC build process
module.exports = {
  node: { global: true}, // Fix: "Uncaught ReferenceError: global is not defined".
  plugins: [
    new webpack.EnvironmentPlugin({
      NODE_ENV: 'development',
      MOTOKO_CANISTER_ID: canisters["motoko"],
      MOTOKO_CANISTER_HOST: canisterHost
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
    hot: true
  },
};
