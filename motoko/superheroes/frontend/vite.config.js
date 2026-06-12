import { defineConfig, loadEnv } from "vite";
import { execSync } from "child_process";
import { icpBindgen } from "@icp-sdk/bindgen/plugins/vite";
import react from "@vitejs/plugin-react";

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

  throw new Error(
    "No local network running. Start with:\n  icp network start -d && icp deploy"
  );
}

export default defineConfig(({ command }) => {
  const env = loadEnv("production", "..", ["CANISTER_"]);

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
