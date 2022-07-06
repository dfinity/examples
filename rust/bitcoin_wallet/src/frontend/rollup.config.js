import svelte from 'rollup-plugin-svelte';
import commonjs from '@rollup/plugin-commonjs';
import resolve from '@rollup/plugin-node-resolve';
import livereload from 'rollup-plugin-livereload';
import { terser } from 'rollup-plugin-terser';
import sveltePreprocess from 'svelte-preprocess';
import typescript from '@rollup/plugin-typescript';
import css from 'rollup-plugin-css-only';
import inject from 'rollup-plugin-inject';
import json from '@rollup/plugin-json';
import path from 'path';
import injectProcessEnv from 'rollup-plugin-inject-process-env';

const production = !process.env.ROLLUP_WATCH;

const { DFX_NETWORK, canisterIds, useIcInternetIdentity, useMockApi } =
  (function () {
    const DFX_NETWORK = process.env.DFX_NETWORK || 'local';
    const useIcInternetIdentity = process.env.USE_PROD_II === 'true';
    const useMockApi = process.env.USE_MOCK_API === 'true';

    if (DFX_NETWORK === 'ic') {
      return {
        DFX_NETWORK,
        canisterIds: require(path.resolve('../../canister_ids.json')),
        useIcInternetIdentity,
        useMockApi,
      };
    } else {
      return {
        DFX_NETWORK,
        canisterIds: require(path.resolve(
          '../..',
          '.dfx',
          'local',
          'canister_ids.json'
        )),
        useIcInternetIdentity,
        useMockApi,
      };
    }
  })();

// console.warn(
//   JSON.stringify(
//     {
//       DFX_NETWORK,
//       canisterIds,
//       useIcInternetIdentity,
//       useMockApi,
//     },
//     null,
//     2
//   )
// );

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

console.log(process.cwd);

export default {
  input: 'src/main.ts',
  output: {
    sourcemap: true,
    format: 'iife',
    name: 'app',
    file: 'public/build/bundle.js',
  },
  plugins: [
    svelte({
      preprocess: sveltePreprocess({
        sourceMap: !production,
        postcss: {
          plugins: [require('tailwindcss')(), require('autoprefixer')()],
        },
      }),
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
    typescript({
      sourceMap: !production,
      inlineSources: !production,
    }),
    json(),
    inject({
      Buffer: ['buffer', 'Buffer'],
    }),
    injectProcessEnv({
      DFX_NETWORK,
      NODE_ENV: production ? 'production' : 'development',
      INTERNET_IDENTITY_ADDRESS: useIcInternetIdentity
        ? 'https://identity.ic0.app/#authorize'
        : `http://${canisterIds.internet_identity['local']}.localhost:8000/#authorize`,
      BACKEND_CANISTER_ID: canisterIds.bitcoin_wallet[DFX_NETWORK],
      USE_MOCK_API: useMockApi,
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
};
