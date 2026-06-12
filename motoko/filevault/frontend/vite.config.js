import { defineConfig, loadEnv } from "vite";
import { execSync } from "child_process";
import react from "@vitejs/plugin-react";
import { icpBindgen } from "@icp-sdk/bindgen/plugins/vite";

function getDevServerConfig() {
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
      replicaPort: "8000",
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

  throw new Error(
    "No local network running. Start with:\n  icp network start -d && icp deploy"
  );
}

export default defineConfig(({ command, mode }) => {
  const env = loadEnv(mode, "..", ["CANISTER_"]);
  const devConfig = command === "serve" ? getDevServerConfig() : undefined;

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
