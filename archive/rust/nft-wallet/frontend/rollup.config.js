import svelte from "rollup-plugin-svelte";
import commonjs from "@rollup/plugin-commonjs";
import resolve from "@rollup/plugin-node-resolve";
import livereload from "rollup-plugin-livereload";
import { terser } from "rollup-plugin-terser";
import css from "rollup-plugin-css-only";
import replace from "@rollup/plugin-replace";
import inject from "rollup-plugin-inject";
import json from "@rollup/plugin-json";
import sveltePreprocess from "svelte-preprocess";
import typescript from "@rollup/plugin-typescript";
import { initCanisterIds, serve } from "./util";

const { canisterIds, network } = initCanisterIds();

const production = !process.env.ROLLUP_WATCH;
const isNetworkLocal = network === "local";

export default {
  input: "src/main.js",
  output: {
    sourcemap: true,
    name: "app",
    format: "iife",
    file: "public/build/main.js",
		inlineDynamicImports: true,
  },
  plugins: [
    svelte({
      preprocess: sveltePreprocess({ sourceMap: !production }),
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
          preventAssignment: false,
          "process.env.DFX_NETWORK": JSON.stringify(network),
          "process.env.NODE_ENV": JSON.stringify(process.env.NODE_ENV),
          "process.env.INTERNET_IDENTITY_CANISTER_ID": JSON.stringify(
            process.env.INTERNET_IDENTITY_CANISTER_ID
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
    typescript({
      sourceMap: !production,
      inlineSources: !production,
    }),
    inject({
      Buffer: ["buffer", "Buffer"],
    }),
    json(),

    // In dev mode, call `npm run start` once
    // the bundle has been generated
    !production && serve(),

    // Watch the `public` directory and refresh the
    // browser on changes when not in production
    !production && livereload("public"),

    // If we're building for production (npm run build
    // instead of npm run dev), minify
    production && terser(),
  ],
  watch: {
    include: "src/**",
    clearScreen: false,
  },
  onwarn: (warning, warn) => {
    if (!/@dfinity|sha256/.test(warning)) {
      warn(warning);
    }
  },
};
