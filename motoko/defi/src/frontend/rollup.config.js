import svelte from "rollup-plugin-svelte";
import commonjs from "@rollup/plugin-commonjs";
import resolve from "@rollup/plugin-node-resolve";
import livereload from "rollup-plugin-livereload";
import { terser } from "rollup-plugin-terser";
import css from "rollup-plugin-css-only";
import replace from "@rollup/plugin-replace";
import inject from "rollup-plugin-inject";
import json from "@rollup/plugin-json";

const production = !process.env.ROLLUP_WATCH;

const path = require("path");

function initCanisterIds() {
  let localCanisters, prodCanisters, canisters;
  try {
    localCanisters = require(path.resolve(
      "..",
      "..",
      ".dfx",
      "local",
      "canister_ids.json"
    ));
  } catch (error) {
    console.log("No local canister_ids.json found. Continuing production");
  }
  try {
    prodCanisters = require(path.resolve("..", "..", "canister_ids.json"));
  } catch (error) {
    console.log("No production canister_ids.json found. Continuing with local");
  }

  const network =
    process.env.DFX_NETWORK ||
    (process.env.NODE_ENV === "production" ? "ic" : "local");

  console.log(network);

  const canisterIds = network === "local" ? localCanisters : prodCanisters;

  return { canisterIds, network };
}
const { canisterIds, network } = initCanisterIds();
console.log(canisterIds);

function serve() {
  let server;

  function toExit() {
    if (server) server.kill(0);
  }

  return {
    writeBundle() {
      if (server) return;
      server = require("child_process").spawn(
        "npm",
        ["run", "start", "--", "--dev"],
        {
          stdio: ["ignore", "inherit", "inherit"],
          shell: true,
        }
      );

      process.on("SIGTERM", toExit);
      process.on("exit", toExit);
    },
  };
}

export default {
  input: "src/main.js",
  output: {
    sourcemap: true,
    format: "iife",
    name: "app",
    file: "../frontend_assets/build/main.js",
		inlineDynamicImports: true,
  },
  plugins: [
    svelte({
      compilerOptions: {
        // enable run-time checks when not in production
        dev: !production,
      },
    }),
    // we'll extract any component CSS out into
    // a separate file - better for performance
    css({ output: "bundle.css" }),

    // If you have external dependencies installed from
    // npm, you'll most likely need these plugins. In
    // some cases you'll need additional configuration -
    // consult the documentation for details:
    // https://github.com/rollup/plugins/tree/master/packages/commonjs
    resolve({
      preferBuiltins: false,
      browser: true,
      dedupe: ["svelte"],
    }),
    // Add canister ID's & network to the environment
    replace(
      Object.assign(
        {
          "process.env.DFX_NETWORK": JSON.stringify(network),
          "process.env.NODE_ENV": JSON.stringify(
            production ? "production" : "development"
          ),
        },
        ...Object.keys(canisterIds)
          .filter((canisterName) => canisterName !== "__Candid_UI")
          .map((canisterName) => ({
            ["process.env." + canisterName.toUpperCase() + "_CANISTER_ID"]:
              JSON.stringify(canisterIds[canisterName][network]),
          }))
      )
    ),
    commonjs(),
    inject({
      Buffer: ["buffer", "Buffer"],
    }),
    json(),

    // In dev mode, call `npm run start` once
    // the bundle has been generated
    !production && serve(),

    // Watch the `../frontend_assets` directory and refresh the
    // browser on changes when not in production
    !production && livereload("../frontend_assets"),

    // If we're building for production (npm run build
    // instead of npm run dev), minify
    production && terser(),
  ],
  watch: {
    clearScreen: false,
  },
};
