import cubeHash from "./cubehash";

test('cubehash("hello")', () => {
  expect(
    Buffer.from(cubeHash(Buffer.from("hello", "utf8"))).toString("hex")
  ).toBe("fb638723f74a25864c5ffb1c3480a1e72178bd55337a4248340776aa46f46f10");
});

test('cubehash("Hello")', () => {
  expect(
    Buffer.from(cubeHash(Buffer.from("Hello", "utf8"))).toString("hex")
  ).toBe("e712139e3b892f2f5fe52d0f30d78a0cb16b51b217da0e4acb103dd0856f2db0");
});

test('cubehash("")', () => {
  expect(Buffer.from(cubeHash(Buffer.from("", "utf8"))).toString("hex")).toBe(
    "44c6de3ac6c73c391bf0906cb7482600ec06b216c7c54a2a8688a6a42676577d"
  );
});

test('cubehash("The quick brown fox jumps over the lazy dog")', () => {
  expect(
    Buffer.from(
      cubeHash(
        Buffer.from("The quick brown fox jumps over the lazy dog", "utf8")
      )
    ).toString("hex")
  ).toBe("5151e251e348cbbfee46538651c06b138b10eeb71cf6ea6054d7ca5fec82eb79");
});
