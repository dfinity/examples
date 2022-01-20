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

In repo dfinity/docs, add some general, language agnostic documentation for the
example. E.g. for `hello`:

* modules/examples/pages/index.adoc: add new bullet in early subsection pointing pointing to hello.adoc.
* modules/examples/pages/hello.adoc: add one page description of hello example.
* modules/examples/pages/assets/hello.png: screenshot of UI linked by hello.adoc (optional).
* modules/ROOT/nav.adoc: add direct site navigation to hello.adoc.

# Issues

This structuring of examples isn't ideal since it requires duplication
of similar files (typically frontend code) but has the advantage that
Motoko users only need to see Motoko specific content and dually for
Rust.

It also makes it possible to have language restricted examples, for
example, when the other language does not support a particular example
well.