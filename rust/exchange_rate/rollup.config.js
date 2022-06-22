import svelte from "rollup-plugin-svelte";
import commonjs from "@rollup/plugin-commonjs";
import resolve from "@rollup/plugin-node-resolve";
import { terser } from "rollup-plugin-terser";
import css from "rollup-plugin-css-only";
import replace from "@rollup/plugin-replace";

const production = !process.env.ROLLUP_WATCH;

const path = require('path');

function initCanisterIds() {
	let localCanisters, prodCanisters;
	try {
		localCanisters = require(path.resolve(
			'.dfx',
			'local',
			'canister_ids.json'
		));
	} catch (error) {
		console.log('No local canister_ids.json found. Continuing production');
	}

	try {
		prodCanisters = require('./canister_ids.json');
	} catch (error) {
		console.log('No production canister_ids.json found. Continuing with local');
	}

	const network =
		process.env.DFX_NETWORK ||
		(process.env.NODE_ENV === 'production' ? 'ic' : 'local');

	const canisterIds =
		network === "local"
		? localCanisters
		: prodCanisters;

	return { canisterIds, network };
}

const { canisterIds, network } = initCanisterIds();

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
	input: 'src/frontend/src/main.js',
	output: {
		sourcemap: true,
		format: 'iife',
		name: 'app',
		file: 'src/frontend/public/build/bundle.js'
	},
	plugins: [
		svelte({
			compilerOptions: {
				// enable run-time checks when not in production
				dev: !production
			}
		}),
		// we'll extract any component CSS out into
		// a separate file - better for performance
		css({ output: 'bundle.css' }),

		// If you have external dependencies installed from
		// npm, you'll most likely need these plugins. In
		// some cases you'll need additional configuration -
		// consult the documentation for details:
		// https://github.com/rollup/plugins/tree/master/packages/commonjs
		resolve({
			preferBuiltins: false,
			browser: true,
			dedupe: ['svelte']
		}),

		replace(
			Object.assign(
			  {
				preventAssignment: false,
				"process.env.DFX_NETWORK": JSON.stringify(network),
				"process.env.NODE_ENV": JSON.stringify(
					network === "ic" ? "production" : "development"
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

		terser()
	],
    onwarn: function (warning) {
      if (
        [
          'CIRCULAR_DEPENDENCY',
          'THIS_IS_UNDEFINED',
          'EVAL',
        ].includes(warning.code)
      ) {
        return;
      }
      console.warn(warning.message);
    },
};
