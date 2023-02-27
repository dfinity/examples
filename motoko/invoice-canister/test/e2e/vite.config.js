import { defineConfig } from 'vite';

/* 
  Since all addressing computations rely on the known canister id the
  invoice canister uses, can skip the normal parsing canister_ids.json 
  and linking with process.env. For an example on how to do such, 
  check out https://github.com/dscvr-one/ic-drip/blob/main/vite.config.js
*/

export default defineConfig({
  test: {
    /*
    //  { describe, it, expect, etc } automatically available globally
    //  however autocomplete in vscode doesn't happen when that's true.
    globals: true,
    //  Runs once before all test suites (but different suites don't 
    //  share the same context).
    globalSetup: [ ],
    */
    //  beforeAll, afterAll, etc hooks: (was only necessary to easily json stringify nat)
    //setupFiles: ['./src/utils/setup-teardown-hooks.js'],

    // Since each test suite does not deploy its own invoice
    // canister to use, this is to run suites sequentially:
    threads: false,
    // Maximum allowed timeout per test (ms):
    testTimeout: 15000,
    include: ['**/tests/*.test.js'],
    reporters: 'verbose',
    // Could use a custom coverage reporter to index and map values of from the actor's IDL.
  },
});
