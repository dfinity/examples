const path = require("path");
const webpack = require("webpack");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

let canisters;

function initCanisterIds() {
  if(dfx_network = process.env.DFX_NETWORK) {
    network = dfx_network;
    console.log(`network was inferred from environment variable DFX_NETWORK`);
  } else {
    network = process.env.NODE_ENV === "production" ? "ic" : "local";
    console.log(`environment variable DFX_NETWORK not set, inferring network from node environment`);
  }

  network_alphanum = network.replace(/[^a-zA-Z0-9]/g, "_"); // replace non-alphanumeric like dfx
  console.log(`network is '${network}' (${network_alphanum})`);

  function getCanisterIds(path) {
    try {
      return require(path);
    } catch (error) {
      console.log(`No canister_ids.json found for network ${network} (${path}), try a different network..`);
      throw error;
    }
  }

  canisters = network === "ic" ?
        getCanisterIds(path.resolve("canister_ids.json")) :
        getCanisterIds(path.resolve(".dfx", network_alphanum, "canister_ids.json"));

  for (const canister in canisters) {
    process.env[canister.toUpperCase() + "_CANISTER_ID"] =
      canisters[canister][network_alphanum];
  }
}
initCanisterIds();

const isDevelopment = process.env.NODE_ENV !== "production";
const asset_entry = path.join(
  "src",
  "testnet_app_assets",
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
    path: path.join(__dirname, "dist", "testnet_app_assets"),
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
    new CopyPlugin({
      patterns: [
        {
          from: path.join(__dirname, "src", "testnet_app_assets", "assets"),
          to: path.join(__dirname, "dist", "testnet_app_assets"),
        },
      ],
    }),
    new webpack.EnvironmentPlugin({
      NODE_ENV: 'development',
      TESTNET_APP_CANISTER_ID: canisters["testnet_app"]
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
    contentBase: path.resolve(__dirname, "./src/testnet_app_assets"),
    watchContentBase: true
  },
};
