import { defineConfig } from "vite";
import { icpBindgen } from "@icp-sdk/bindgen/plugins/vite";
import react from "@vitejs/plugin-react";
import { execSync } from "child_process";

export default defineConfig(({ command }) => {
  const plugins = [
    react(),
    icpBindgen({
      didFile: "../backend/backend.did",
      outDir: "./src/bindings",
    }),
  ];

  if (command !== "serve") {
    return { plugins };
  }

  const environment = process.env.ICP_ENVIRONMENT || "local";
  const CANISTER_NAME = "backend";

  let networkStatus;
  try {
    networkStatus = JSON.parse(
      execSync(`icp network status -e ${environment} --json`, {
        encoding: "utf-8",
        stdio: "pipe",
      })
    );
  } catch {
    console.error(
      `No local network running for environment "${environment}". Start it and deploy first:\n` +
        "  icp network start -d && icp deploy"
    );
    process.exit(1);
  }
  const rootKey = networkStatus.root_key;
  const proxyTarget = networkStatus.api_url;

  let canisterId;
  try {
    canisterId = execSync(
      `icp canister status ${CANISTER_NAME} -e ${environment} -i`,
      { encoding: "utf-8", stdio: "pipe" }
    ).trim();
  } catch {
    console.error(
      `Canister "${CANISTER_NAME}" is not deployed in environment "${environment}". Deploy it first:\n` +
        `  icp deploy ${CANISTER_NAME} -e ${environment}`
    );
    process.exit(1);
  }

  return {
    plugins,
    server: {
      headers: {
        "Set-Cookie": `ic_env=${encodeURIComponent(
          `PUBLIC_CANISTER_ID:${CANISTER_NAME}=${canisterId}&ic_root_key=${rootKey}`
        )}; SameSite=Lax;`,
      },
      proxy: {
        "/api": { target: proxyTarget, changeOrigin: true },
      },
    },
  };
});
