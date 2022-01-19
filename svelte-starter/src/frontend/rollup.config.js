import svelte from 'rollup-plugin-svelte';
import commonjs from '@rollup/plugin-commonjs';
import resolve from '@rollup/plugin-node-resolve';
import livereload from 'rollup-plugin-livereload';
import { terser } from 'rollup-plugin-terser';
import css from 'rollup-plugin-css-only';
import inject from 'rollup-plugin-inject';
import json from '@rollup/plugin-json';
import injectProcessEnv from 'rollup-plugin-inject-process-env';

const production = !process.env.ROLLUP_WATCH;

const path = require('path');

function initCanisterIds() {
  let localCanisters, localIiCanister, prodCanisters;
  try {
    localCanisters = require(path.resolve(
      '..',
      '..',
      '.dfx',
      'local',
      'canister_ids.json'
    ));
  } catch (error) {
    console.log('No local canister_ids.json found. Continuing production');
  }
  try {
    localIiCanister = require(path.resolve(
      '..',
      '..',
      'internet-identity',
      '.dfx',
      'local',
      'canister_ids.json'
    ));
  } catch (error) {
    console.log(
      'No local internet-identity canister_ids.json found. Continuing production'
    );
  }
  try {
    prodCanisters = require(path.resolve('..', '..', 'canister_ids.json'));
  } catch (error) {
    console.log('No production canister_ids.json found. Continuing with local');
  }

  const network =
    process.env.DFX_NETWORK ||
    (process.env.NODE_ENV === 'production' ? 'ic' : 'local');

  const canisterIds =
    network === 'local'
      ? { ...(localCanisters || {}), ...(localIiCanister || {}) }
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
      server = require('child_process').spawn(
        'npm',
        ['run', 'start', '--', '--dev'],
        {
          stdio: ['ignore', 'inherit', 'inherit'],
          shell: true,
        }
      );

      process.on('SIGTERM', toExit);
      process.on('exit', toExit);
    },
  };
}

export default {
  input: 'src/main.js',
  output: {
    sourcemap: true,
    format: 'iife',
    name: 'app',
    file: 'public/build/bundle.js',
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
    css({ output: 'bundle.css' }),

    // If you have external dependencies installed from
    // npm, you'll most likely need these plugins. In
    // some cases you'll need additional configuration -
    // consult the documentation for details:
    // https://github.com/rollup/plugins/tree/master/packages/commonjs
    resolve({
      preferBuiltins: false,
      browser: true,
      dedupe: ['svelte'],
    }),

    commonjs(),
    json(),
    inject({
      Buffer: ['buffer', 'Buffer'],
    }),
    injectProcessEnv({
      DFX_NETWORK: network,
      NODE_ENV: production ? 'production' : 'development',
      ...Object.assign(
        {},
        ...Object.keys(canisterIds)
          .filter((canisterName) => canisterName !== '__Candid_UI')
          .map((canisterName) => ({
            [canisterName.toUpperCase() + '_CANISTER_ID']:
              canisterIds[canisterName][network],
          }))
      ),
    }),

    // In dev mode, call `npm run start` once
    // the bundle has been generated
    !production && serve(),

    // Watch the `public` directory and refresh the
    // browser on changes when not in production
    !production && livereload('public'),

    // If we're building for production (npm run build
    // instead of npm run dev), minify
    production && terser(),
  ],
  watch: {
    clearScreen: false,
  },
  onwarn: function (warning) {
    if (
      [
        'CIRCULAR_DEPENDENCY',
        'THIS_IS_UNDEFINED',
        'EVAL',
        'NAMESPACE_CONFLIC',
      ].includes(warning.code)
    ) {
      return;
    }
    console.warn(warning.message);
  },
};
