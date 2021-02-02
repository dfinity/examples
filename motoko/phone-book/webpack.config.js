const Config = require("./dfx.json");
const Path = require("path");
const TerserPlugin = require("terser-webpack-plugin");

// Identify build output directory.
const output = ["defaults", "build", "output"].reduce((accum, x) => {
  return accum && accum[x] ? accum[x] : null;
}, Config) || Path.join(".dfx", "local", "canisters");

// Identify canisters aliases.
const aliases = Object.entries(Config.canisters).reduce((accum, [name,]) => {
  const outputRoot = Path.join(__dirname, output, name);
  return {
    ...accum,
    ["ic:canisters/" + name]: Path.join(outputRoot, name + ".js"),
    ["ic:idl/" + name]: Path.join(outputRoot, name + ".did.js"),
  };
}, {});

// Generate webpack configuration.
const generate = (name, info) => {
  if (typeof info.frontend !== 'object') {
    return;
  };
  const inputRoot = __dirname;
  const outputRoot = Path.join(__dirname, output, name);
  return {
    entry: Path.join(inputRoot, info.frontend.entrypoint),
    mode: "production",
    module: {
      rules: [
        {
          loader: "ts-loader",
          test: /\.(js|ts)x?$/,
        },
      ],
    },
    optimization: {
      minimize: true,
      minimizer: [
        new TerserPlugin(),
      ],
    },
    output: {
      filename: "index.js",
      path: Path.join(outputRoot, "assets"),
    },
    resolve: {
      alias: aliases,
    },
  };
};

module.exports = [
  ...Object.entries(Config.canisters).map(([name, info]) => {
    return generate(name, info);
  }).filter(x => !!x),
];
