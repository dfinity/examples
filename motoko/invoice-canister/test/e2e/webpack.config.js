const path = require("path");

module.exports = {
  entry: "./src/index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bundle.js",
  },

  devServer: {
    publicPath: "",
    contentBase: path.resolve(__dirname, "dist"),
    watchContentBase: true,
    compress: true,
    port: 8080,
    open: true,
  },
};
