const path = require("path");

module.exports = {
  entry: "./src/icpAgent.ts",
  output: {
    path: path.resolve(__dirname, "wwwroot"),
    filename: "icpAgent.js",
    library: {
      name: "IcpAgent",
      type: "var",
    },
    globalObject: "window",
  },
  resolve: {
    extensions: [".ts", ".js"],
  },
  module: {
    rules: [{ test: /\.ts$/, use: "ts-loader" }],
  },
  mode: "production",
};
