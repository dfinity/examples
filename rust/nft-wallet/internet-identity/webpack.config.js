const path = require("path");
const webpack = require("webpack");
const CopyPlugin = require("copy-webpack-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const CompressionPlugin = require("compression-webpack-plugin");
const dfxJson = require("./dfx.json");
require("dotenv").config();

let localCanister;

try {
  localCanister = require("./.dfx/local/canister_ids.json").internet_identity.local;
} catch {}

/**
 * Generate a webpack configuration for a canister.
 */
function generateWebpackConfigForCanister(name, info) {
  const isProduction = process.env.NODE_ENV === "production";
  const devtool = isProduction ? undefined : "source-map";

  return {
    mode: isProduction ? "production" : "development",
    entry: {
      index: path.join(__dirname, "src", "frontend", "src", "index"),
    },
    devtool,
    optimization: {
      minimize: isProduction,
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
      filename: "[name].js",
      path: path.join(__dirname, "dist"),
    },
    devServer: {
      port: 8080,
      proxy: {
        "/api": "http://localhost:8000",
        "/authorize": "http://localhost:8081",
      },
      allowedHosts: [".localhost", ".local", ".ngrok.io"],
      historyApiFallback: true, // makes sure our index is served on all endpoints, e.g. `/faq`
    },

    // Depending in the language or framework you are using for
    // front-end development, add module loaders to the default
    // webpack configuration. For example, if you are using React
    // modules and CSS as described in the "Adding a stylesheet"
    // tutorial, uncomment the following lines:
    module: {
      rules: [
        { test: /\.(ts|tsx)$/, loader: "ts-loader" },
        { test: /\.css$/, use: ["style-loader", "css-loader"] },
        {
          test: /\.(png|jpg|gif)$/i,
          type: "asset/resource",
        },
      ],
    },
    plugins: [
      new CopyPlugin({
        patterns: [
          {
            from: path.join(__dirname, "src", "frontend", "assets"),
            to: path.join(__dirname, "dist"),
          },
        ],
      }),
      new webpack.ProvidePlugin({
        Buffer: [require.resolve("buffer/"), "Buffer"],
        process: require.resolve("process/browser"),
      }),
      new webpack.EnvironmentPlugin({
        "CANISTER_ID": localCanister,
        "II_ENV": "production"
      }),
      new CompressionPlugin({
        test: /\.js(\?.*)?$/i,
      }),
      new webpack.IgnorePlugin(/^\.\/wordlists\/(?!english)/, /bip39\/src$/),
    ],
  };
}

// If you have additional webpack configurations you want to build
//  as part of this configuration, add them to the section below.
module.exports = [
  ...Object.entries(dfxJson.canisters)
    .map(([name, info]) => {
      return generateWebpackConfigForCanister(name, info);
    })
    .filter((x) => !!x),
];
