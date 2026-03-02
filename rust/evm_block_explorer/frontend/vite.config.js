import { defineConfig, loadEnv } from "vite";
import react from "@vitejs/plugin-react";
import { execSync } from "child_process";
import { icpBindgen } from "@icp-sdk/bindgen/plugins/vite";

function getDevServerConfig() {
  // Try icp-cli first
  try {
    const canisterId = execSync("icp canister status backend -e local -i", {
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
      headers: {
        "Set-Cookie": `ic_env=${encodeURIComponent(
          `ic_root_key=${networkStatus.root_key}&PUBLIC_CANISTER_ID:backend=${canisterId}`
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
    const canisterId = execSync("dfx canister id backend", {
      encoding: "utf-8",
      stdio: "pipe",
    }).trim();
    return {
      headers: {
        "Set-Cookie": `ic_env=${encodeURIComponent(
          `ic_root_key=${rootKeyHex}&PUBLIC_CANISTER_ID:backend=${canisterId}`
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
  // dfx generates ../.env with CANISTER_ID_* vars on deploy. Bake them into the
  // bundle so actor.js can fall back to them when the ic_env cookie does not
  // contain canister IDs (dfx does not inject PUBLIC_CANISTER_ID:* env vars
  // into the asset canister, unlike icp-cli).
  const env = loadEnv(mode, "..", ["CANISTER_"]);

  return {
    base: "./",
    plugins: [
      react(),
      icpBindgen({
        didFile: "../backend/backend.did",
        outDir: "./src/bindings",
      }),
    ],
    define: {
      "process.env.CANISTER_ID_BACKEND": JSON.stringify(
        env.CANISTER_ID_BACKEND
      ),
    },
    optimizeDeps: {
      esbuildOptions: { define: { global: "globalThis" } },
    },
    server: command === "serve" ? getDevServerConfig() : undefined,
  };
});
