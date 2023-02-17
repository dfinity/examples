import { beforeAll } from 'vitest';
// Other hooks can also be imported, such as `afterAll`.

// Runs once before each and every test suite.
beforeAll(() => {
  // Simplest way to do this afaik.
  global.BigInt.prototype.toJSON = function () {
    return this.toString();
  };
});
