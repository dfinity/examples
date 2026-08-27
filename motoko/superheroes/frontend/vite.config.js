import { defineConfig } from "vite";
import { execSync } from "child_process";
import { icpBindgen } from "@icp-sdk/bindgen/plugins/vite";
import react from "@vitejs/plugin-react";

const CANISTER_NAME = "backend";
const ENVIRONMENT = process.env.ICP_ENVIRONMENT || "local";

function getDevServerConfig() {
  let networkStatus;
  try {
    networkStatus = JSON.parse(
      execSync(`icp network status -e ${ENVIRONMENT} --json`, {
        encoding: "utf-8",
        stdio: "pipe",
      })
    );
  } catch {
    console.error(
      `No network running for environment "${ENVIRONMENT}". Start it and deploy first:\n` +
        "  icp network start -d && icp deploy"
    );
    process.exit(1);
  }

  let canisterId;
  try {
    canisterId = execSync(
      `icp canister status ${CANISTER_NAME} -e ${ENVIRONMENT} -i`,
      { encoding: "utf-8", stdio: "pipe" }
    ).trim();
  } catch {
    console.error(
      `Canister "${CANISTER_NAME}" is not deployed in environment "${ENVIRONMENT}". Deploy it first:\n` +
        `  icp deploy ${CANISTER_NAME} -e ${ENVIRONMENT}`
    );
    process.exit(1);
  }

  return {
    headers: {
      "Set-Cookie": `ic_env=${encodeURIComponent(
        `ic_root_key=${networkStatus.root_key}&PUBLIC_CANISTER_ID:${CANISTER_NAME}=${canisterId}`
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
      react(),
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
