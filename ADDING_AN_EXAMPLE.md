# How to add a new example

> For ICP Ninja: check [NINJA_CONTRIBUTING.md](./NINJA_CONTRIBUTING.md) for how to contribute a project to ICP Ninja.

Each example should be available in both Rust and Motoko variations, implementing the same Candid interface (and, ideally, semantics).

To illustrate the pattern, this repo now contains one such example, project `hello_world`:

`motoko/hello_world`
`rust/hello_world`

When adding a new `dfx` generated project, make sure to delete its GitHub metadata files (`.gitignore`, `.git` etc).

Each project should include a language-specific README.md that also links to the corresponding README.md of its counterpart in another language, making it easy for language-curious readers to explore both implementations.

## CI

Apart from the standard `dfx` material, each project should provide a `Makefile` used by GitHub Actions CI to run (very) basic tests.

For each example, there is a single CI file with four build actions to produce Darwin and Linux builds and tests of the Motoko/Rust, projects, such as:

```
.github/workflows/hello_world.yml
```

Implementing the GitHub action will ensure it runs in CI and helps keep examples in sync with releases of `dfx`.

## Documentation

For your new example to be included in the ICP developer documentation, make sure you update the `samples` submodule in the portal repository to point to the latest commit in this examples repository using the following command:

```bash
git submodule update --remote submodules/samples
```

After you run this command, commit the changes to a new PR to have them merged into the portal repo. 

## Issues

While this structure leads to some duplication (especially shared components like frontend code) it ensures that Motoko users can focus solely on Motoko-specific content, and likewise for Rust users. It also enables easily finding language-specific examples when a given use case is not easily supported in the other language.
