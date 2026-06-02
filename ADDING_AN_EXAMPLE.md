# How to add a new example

Each example should be available in both Rust and Motoko variations, implementing the same Candid interface (and, ideally, semantics).

To illustrate the pattern, this repo contains one such example, project `hello_world`:

```
motoko/hello_world
rust/hello_world
```

When adding a new project, make sure to delete any generated GitHub metadata files (`.gitignore`, `.git` etc).

Each project should include a language-specific README.md that also links to the corresponding README.md of its counterpart in another language, making it easy for language-curious readers to explore both implementations.

## CI

Each project should provide a `Makefile` with a `test` target that runs basic canister tests using `icp canister call`. Each example also needs a GitHub Actions workflow file at `.github/workflows/<example_name>.yml`.

Use the workflow template as a starting point:

```
.github/workflow-template.yml
```

Copy it, replace the placeholders, and add the appropriate container image:

- Motoko: `ghcr.io/dfinity/icp-dev-env-motoko`
- Rust: `ghcr.io/dfinity/icp-dev-env-rust`

See `hello_world` and `who_am_i` for reference implementations. Workflows run on Linux only using container images — no provision scripts needed.

## Notes

While this structure leads to some duplication (especially shared frontend code) it ensures that Motoko users can focus solely on Motoko-specific content, and likewise for Rust users. It also enables easily finding language-specific examples when a given use case is not easily supported in the other language.
