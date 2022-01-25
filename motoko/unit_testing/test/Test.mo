import U "../src/Utils";

import ActorSpec "./utils/ActorSpec";
type Group = ActorSpec.Group;

let assertTrue = ActorSpec.assertTrue;
let describe = ActorSpec.describe;
let it = ActorSpec.it;
let skip = ActorSpec.skip;
let pending = ActorSpec.pending;
let run = ActorSpec.run;

run([
  describe("Example Test Suite", [
    describe("Subgroup", [
      it("should assess a boolean value", do {
        assertTrue(true);
      }),
      it("should sum two positive Nats", do {
        assertTrue(U.sum(1, 2) == 3);
      }),
      it("should fail a check that doesn't match", do {
        assertTrue(U.sum(1, 2) == 4);
      }),
      skip("should skip a test", do {
        // Useful for defining a test that is not yet implemented
        true
      }),
    ]),
  ]),
]);
