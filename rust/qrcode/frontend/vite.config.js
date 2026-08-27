import { defineConfig } from "vite";
import { execSync } from "child_process";
import { icpBindgen } from "@icp-sdk/bindgen/plugins/vite";

function getDevServerConfig() {
  let networkStatus;
  try {
    networkStatus = JSON.parse(
      execSync("icp network status -e local --json", {
        encoding: "utf-8",
        stdio: "pipe",
      })
    );
  } catch {
    console.error(
      "No local network running. Start it and deploy first:\n" +
        "  icp network start -d && icp deploy"
    );
    process.exit(1);
  }

  let canisterId;
  try {
    canisterId = execSync("icp canister status backend -e local -i", {
      encoding: "utf-8",
      stdio: "pipe",
    }).trim();
  } catch {
    console.error(
      "Canister 'backend' is not deployed on the local network. Deploy it first:\n" +
        "  icp deploy backend"
    );
    process.exit(1);
  }

  return {
    headers: {
      "Set-Cookie": `ic_env=${encodeURIComponent(
        `ic_root_key=${networkStatus.root_key}&PUBLIC_CANISTER_ID:backend=${canisterId}`
      )}; SameSite=Lax;`,
    },
    proxy: {
      "/api": { target: networkStatus.api_url, changeOrigin: true },
    },
  };
}

export default defineConfig(({ command }) => {
  return {
    base: "./",
    plugins: [
      icpBindgen({
        didFile: "../backend/backend.did",
        outDir: "./src/bindings",
      }),
    ],
    optimizeDeps: {
      esbuildOptions: { define: { global: "globalThis" } },
    },
    server: command === "serve" ? getDevServerConfig() : undefined,
  };
});
