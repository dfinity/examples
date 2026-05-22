# How to add a new example

Each example should be available in both Rust and Motoko variations, implementing the same Candid interface (and, ideally, semantics).

To illustrate the pattern, this repo contains one such example, project `hello_world`:

```
motoko/hello_world
rust/hello_world
```

When adding a new project, make sure to delete any generated GitHub metadata files (`.gitignore`, `.git` etc).

Each project should include a language-specific README.md that also links to the corresponding README.md of its counterpart in another language, making it easy for language-curious readers to explore both implementations.

## Codespaces

To make the example available in GitHub Codespaces, add a devcontainer config under `.devcontainer/<language>-<example>/devcontainer.json` pointing to the appropriate image:

- Motoko: `ghcr.io/dfinity/icp-dev-env-motoko`
- Rust: `ghcr.io/dfinity/icp-dev-env-rust`

Add a Codespaces badge to the example's README pointing to the new devcontainer config. See the existing `who_am_i` examples for reference.

## CI

Each project should provide a `Makefile` used by GitHub Actions CI to run basic tests. For each example, there is a single CI file:

```
.github/workflows/hello_world.yml
```

Implementing the GitHub action ensures it runs in CI and helps keep examples in sync with [icp-cli](https://cli.internetcomputer.org) releases.

## Notes

While this structure leads to some duplication (especially shared frontend code) it ensures that Motoko users can focus solely on Motoko-specific content, and likewise for Rust users. It also enables easily finding language-specific examples when a given use case is not easily supported in the other language.
