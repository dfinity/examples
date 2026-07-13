import { defineConfig } from "vite";
import { icpBindgen } from "@icp-sdk/bindgen/plugins/vite";
import { execSync } from "child_process";

export default defineConfig(({ command }) => {
  const plugins = [
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

  const networkStatus = JSON.parse(
    execSync(`icp network status -e ${environment} --json`, {
      encoding: "utf-8",
    })
  );
  const rootKey = networkStatus.root_key;
  const proxyTarget = networkStatus.api_url;

  let canisterId;
  try {
    canisterId = execSync(
      `icp canister status ${CANISTER_NAME} -e ${environment} -i`,
      { encoding: "utf-8" }
    ).trim();
  } catch {
    console.error(`
     Backend canister "${CANISTER_NAME}" not found in environment "${environment}"

     Before running the dev server, deploy the backend canister:

       icp deploy ${CANISTER_NAME} -e ${environment}
    `);
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
        "/api": {
          target: proxyTarget,
          changeOrigin: true,
        },
      },
    },
  };
});
