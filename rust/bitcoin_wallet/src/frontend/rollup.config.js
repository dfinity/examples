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
import fs from 'fs';
import injectProcessEnv from 'rollup-plugin-inject-process-env';

const production = !process.env.ROLLUP_WATCH;

const { DFX_NETWORK, canisterIds } = (function () {
  if (production && fs.existsSync('../../canister_ids.json')) {
    return {
      DFX_NETWORK: 'ic',
      canisterIds: require(path.resolve('../../canister_ids.json')),
    };
  } else {
    return {
      DFX_NETWORK: 'local',
      canisterIds: require(path.resolve(
        '../..',
        '.dfx',
        'local',
        'canister_ids.json'
      )),
    };
  }
})();

// const DFX_NETWORK = production ? 'ic' : 'local';
// const canisterIds = production
//   ? require(path.resolve('../../canister_ids.json'))
//   : require(path.resolve('../..', '.dfx', 'local', 'canister_ids.json'));

console.log({ canisterIds });

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
      INTERNET_IDENTITY_CANISTER_ID: canisterIds.internet_identity[DFX_NETWORK],
      BACKEND_CANISTER_ID: canisterIds.bitcoin_wallet[DFX_NETWORK],
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
