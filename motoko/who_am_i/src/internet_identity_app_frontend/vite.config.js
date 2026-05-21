import { defineConfig, loadEnv } from "vite";
import { execSync } from "child_process";
import react from "@vitejs/plugin-react";
import { icpBindgen } from "@icp-sdk/bindgen/plugins/vite";

function getDevServerConfig() {
  // Try icp-cli first
  try {
    const canisterId = execSync("icp canister status internet_identity_app_backend -e local -i", {
      encoding: "utf-8",
      stdio: "pipe",
    }).trim();
    const networkStatus = JSON.parse(
      execSync("icp network status --json", {
        encoding: "utf-8",
        stdio: "pipe",
      })
    );
    return {
      replicaPort: "8000",
      headers: {
        "Set-Cookie": `ic_env=${encodeURIComponent(
          `ic_root_key=${networkStatus.root_key}&PUBLIC_CANISTER_ID:internet_identity_app_backend=${canisterId}`
        )}; SameSite=Lax;`,
      },
      proxy: {
        "/api": { target: "http://127.0.0.1:8000", changeOrigin: true },
      },
    };
  } catch {}

  // Try dfx
  try {
    const pingResult = JSON.parse(
      execSync("dfx ping", { encoding: "utf-8", stdio: "pipe" })
    );
    const rootKeyHex = Buffer.from(pingResult.root_key).toString("hex");
    const canisterId = execSync(
      "dfx canister id internet_identity_app_backend",
      {
        encoding: "utf-8",
        stdio: "pipe",
      }
    ).trim();
    return {
      replicaPort: "4943",
      headers: {
        "Set-Cookie": `ic_env=${encodeURIComponent(
          `ic_root_key=${rootKeyHex}&PUBLIC_CANISTER_ID:internet_identity_app_backend=${canisterId}`
        )}; SameSite=Lax;`,
      },
      proxy: {
        "/api": {
          target: "http://127.0.0.1:4943",
          changeOrigin: true,
        },
      },
      host: "127.0.0.1",
    };
  } catch {}

  throw new Error(
    "No local network running. Start with:\n  icp network start -d && icp deploy\nor:\n  dfx start --background && dfx deploy"
  );
}

export default defineConfig(({ command, mode }) => {
  // dfx generates ../../.env with CANISTER_ID_* vars on deploy. Bake them into
  // the bundle so actor.js can fall back to them when the ic_env cookie does not
  // contain canister IDs (dfx does not inject PUBLIC_CANISTER_ID:* env vars
  // into the asset canister, unlike icp-cli).
  const env = loadEnv(mode, "../..", ["CANISTER_"]);
  const devConfig = command === "serve" ? getDevServerConfig() : undefined;

  return {
    base: "./",
    plugins: [
      react(),
      icpBindgen({
        didFile:
          "../internet_identity_app_backend/internet_identity_app_backend.did",
        outDir: "./src/bindings",
      }),
    ],
    define: {
      "process.env.CANISTER_ID_INTERNET_IDENTITY_APP_BACKEND": JSON.stringify(
        env.CANISTER_ID_INTERNET_IDENTITY_APP_BACKEND
      ),
      "process.env.REPLICA_PORT": JSON.stringify(devConfig?.replicaPort ?? ""),
    },
    optimizeDeps: {
      esbuildOptions: { define: { global: "globalThis" } },
    },
    server: devConfig
      ? {
          headers: devConfig.headers,
          proxy: devConfig.proxy,
          host: devConfig.host,
        }
      : undefined,
  };
});
