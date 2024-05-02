# How to Add a New Example

Ideally, each example should come with both Rust and Motoko implementations,
implementing the same Candid interface (and, ideally, semantics).

To illustrate the pattern, this repo now contains one such example, project `hello`.

The (dual) sources for the projects live in:

motoko/hello
rust/hello

When adding a new dfx generated project, make sure to delete its GH metadata files (.gitignore, .git etc).

Each project should have a similar, but language specific `README.md`. E.g.

motoko/hello/README.md
rust/hello/README.md

Each `README.md` should link to the other project's README.md for
language-curious readers.

# CI

Apart from the standard dfx material, each project should provide a
`Makefile` used by GitHub Actions CI to run (very) basic tests. E.g.

motoko/hello/Makefile
rust/hello/Makefile

For each example, there is a single CI file with four build actions to
produce darwin and linux builds and tests of the motoko/rust
projects. E.g.

.github/workflows/hello.yml

Implementing the GH action will ensure it runs in CI and gives us some
hope of keeping examples in sync with releases of dfx.

# Documentation

Make sure to add documentation for the example in the [dfinity/portal](https://github.com/dfinity/portal) repository once your example has been merged.

Add a quick section to [`docs/samples/overview.mdx`](https://github.com/dfinity/portal/blob/master/docs/samples/overview.mdx) and make sure the example is listed in the "Sample apps" category in [`sidebars.js`](https://github.com/dfinity/portal/blob/master/sidebars.js).

Also make sure to add an entry in [`src/components/Common/sampleItems.ts`](https://github.com/dfinity/portal/blob/master/src/components/Common/sampleItems.ts) for your example.

The content of the documentation is generated automatically from the `README.md` files in the examples repository, make sure your `README.md` contains a link to the actual example so people can easily find it. The following could be the first sentence of your `README.md`:

> [View this samples code on GitHub](https://github.com/dfinity/examples/tree/master/rust/token_transfer_from).

For your new example to be included in the auto generated documentation, make sure you update the submodule in the portal repository to point to the latest commit in the examples repository with the following command:

```bash
git submodule update --remote submodules/samples
```

# Issues

This structuring of examples isn't ideal since it requires duplication
of similar files (typically frontend code) but has the advantage that
Motoko users only need to see Motoko specific content and dually for
Rust.

It also makes it possible to have language restricted examples, for
example, when the other language does not support a particular example
well.
