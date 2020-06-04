
To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with dfx_react_typescript, see the following documentation available online:

- [Quick Start](https://sdk.dfinity.org/developers-guide/quickstart.html)
- [Developer's Guide](https://sdk.dfinity.org/developers-guide)
- [Language Reference](https://sdk.dfinity.org/language-guide)

```bash
cd dfx_react_typescript/

dfx start

# open a new terminal

dfx build
dfx install canister dfx_react_typescript

# use above output ID here:
# open http://localhost:8000/?canisterId=<your-canister-id>
```
