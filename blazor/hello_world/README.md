# ICP Blazor Hello World

A hello world template combining **Blazor WebAssembly (.NET 10)** on the frontend and **Motoko** on the backend, deployed fully on-chain on the **Internet Computer (ICP)**.

> The first working template for Blazor WASM + icp-cli + Motoko.

## Stack

| Layer    | Technology                          |
|----------|-------------------------------------|
| Frontend | Blazor WebAssembly (.NET 10)        |
| Backend  | Motoko canister                     |
| Bridge   | @dfinity/agent (webpack bundle)     |
| Platform | Internet Computer (ICP)             |
| CLI      | icp-cli                             |

## Project Structure

```
icp-blazor-hello/
├── icp.yaml                          # icp-cli project config
├── .gitignore
├── backend/
│   ├── canister.yaml                 # backend canister config
│   ├── backend.did                   # Candid interface (semicolons required!)
│   └── src/
│       └── main.mo                   # Motoko canister
└── frontend/
    ├── canister.yaml                 # frontend canister config
    └── BlazorFrontend/
        ├── BlazorFrontend.csproj     # .NET 10 Blazor WASM
        ├── Program.cs
        ├── App.razor
        ├── _Imports.razor            # required — Blazor namespace imports
        ├── package.json              # webpack + @dfinity/agent
        ├── webpack.config.js         # bundles icpAgent.ts → wwwroot/icpAgent.js
        ├── tsconfig.json
        ├── src/
        │   └── icpAgent.ts           # TypeScript ICP agent bridge
        ├── Layout/
        │   └── MainLayout.razor
        ├── Pages/
        │   └── Home.razor            # main UI with canister calls
        ├── Services/
        │   └── IcpAgentService.cs    # C# → JS interop service
        └── wwwroot/
            ├── index.html
            └── app.css
```

## How it works

```
Home.razor (C#)
  → IcpAgentService.cs (IJSRuntime)
    → window.IcpAgent.* (webpack bundle, defer loaded)
      → @dfinity/agent
        → Motoko backend canister on ICP
```

The key insight: use **webpack** to bundle `@dfinity/agent` into a plain JS file
loaded with `defer`, not as an ES module. This avoids the race condition between
the module loader and Blazor's JS interop system.

## Prerequisites

```bash
# icp-cli
npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm

# Motoko toolchain
npm install -g ic-mops
```

## Prerequisites

### icp-cli
```bash
npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm
npm install -g ic-mops
```

### .NET 10 SDK

**Method 1: Ubuntu 24.04 LTS or newer (APT)**
```bash
sudo apt update
sudo apt install -y dotnet-sdk-10.0
```

> **Ubuntu 22.04 LTS?** Register the backports PPA first:
> ```bash
> sudo add-apt-repository ppa:dotnet/backports
> sudo apt update
> sudo apt install -y dotnet-sdk-10.0
> ```

**Method 2: Official Microsoft script (recommended for non-Ubuntu or if APT fails)**
```bash
wget https://dot.net/v1/dotnet-install.sh -O dotnet-install.sh
chmod +x dotnet-install.sh
./dotnet-install.sh --channel 10.0
```

After installing, add .NET to your PATH:
```bash
echo 'export PATH="$HOME/.dotnet:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Blazor WASM workload (required)
```bash
dotnet workload install wasm-tools
```

### Remove conflicting `icp` binary (Ubuntu/Debian)
Ubuntu ships a package called `renameutils` that installs its own unrelated `icp` binary.
Remove it before installing icp-cli:
```bash
sudo apt remove renameutils -y
```

Verify the correct `icp` is active:
```bash
icp --version  # should show icp-cli, not renameutils
```

## Run locally

```bash
# 1. Start the local ICP network (make sure port 8000 is free)
icp network start -d

# 2. Deploy both canisters
icp deploy

# 3. Open the URL printed by icp deploy
#    Format: http://<frontend-canister-id>.localhost:8000
```

## Deploy to mainnet

```bash
icp deploy --network ic
```

## Known gotchas

### `_Imports.razor` is required
Without `_Imports.razor`, Blazor component events silently do nothing.
Always include `@using Microsoft.JSInterop` in it.

### Port 8000 conflict
If `icp network start` exits with status 101, check the log:
```bash
cat .icp/cache/networks/local/network-launcher/stderr.log
```
Common cause: Docker container mapped to port 8000.
```bash
docker ps | grep 8000
docker stop <container>
```
## License

MIT
