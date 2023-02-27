import { beforeAll } from 'vitest';
// Other hooks can also be imported, such as `afterAll`.

// Runs once before each and every test suite.
beforeAll(() => {
  // Simplest way to do this afaik.
  // Note this is actually commented out in the config,
  // was used for easily console.loggin' out JSON.stringifying. 
  global.BigInt.prototype.toJSON = function () {
    return this.toString();
  };
});
