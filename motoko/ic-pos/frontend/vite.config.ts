import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import { TanStackRouterVite } from "@tanstack/router-plugin/vite";
import { icpBindgen } from "@icp-sdk/bindgen/plugins/vite";
import { execSync } from "child_process";
import path from "path";

// Canisters whose IDs the frontend needs at runtime. The asset canister injects
// these as `PUBLIC_CANISTER_ID:<name>` env vars in production (see icp.yaml). In
// dev the Vite server sets the same values via the `ic_env` cookie below.
const FRONTEND_CANISTERS = [
  "backend",
  "icrc1_ledger",
  "icrc1_index",
];

export default defineConfig(({ command }) => {
  const plugins = [
    react(),
    tailwindcss(),
    TanStackRouterVite({
      routesDirectory: "src/routes",
      generatedRouteTree: "src/routeTree.gen.ts",
    }),
    icpBindgen({
      didFile: "../backend/backend.did",
      outDir: "src/bindings",
    }),
  ];

  const config = {
    build: {
      emptyOutDir: true,
    },
    resolve: {
      alias: {
        "@": path.resolve(__dirname, "./src"),
      },
    },
    optimizeDeps: {
      esbuildOptions: {
        define: {
          global: "globalThis",
        },
      },
    },
    plugins,
  };

  // Build only — no dev-server setup needed.
  if (command !== "serve") {
    return config;
  }

  // Dev server: look up the local network root key and canister IDs and expose
  // them to the app via the same `ic_env` cookie the asset canister sets in
  // production.
  const environment = process.env.ICP_ENVIRONMENT || "local";

  const networkStatus = JSON.parse(
    execSync(`icp network status -e ${environment} --json`, {
      encoding: "utf-8",
    })
  );
  const rootKey = networkStatus.root_key;
  const proxyTarget = networkStatus.api_url;

  const envParts: string[] = [];
  for (const name of FRONTEND_CANISTERS) {
    try {
      const canisterId = execSync(
        `icp canister status ${name} -e ${environment} -i`,
        { encoding: "utf-8" }
      ).trim();
      envParts.push(`PUBLIC_CANISTER_ID:${name}=${canisterId}`);
    } catch {
      if (name === "backend") {
        console.error(`
     Canister "${name}" not found in environment "${environment}".

     Before running the dev server, deploy the canisters:

       ./deploy.sh
    `);
        process.exit(1);
      }
      console.warn(
        `[vite] Canister "${name}" not found in environment "${environment}"; skipping.`
      );
    }
  }
  envParts.push(`ic_root_key=${rootKey}`);

  return {
    ...config,
    server: {
      headers: {
        "Set-Cookie": `ic_env=${encodeURIComponent(envParts.join("&"))}; SameSite=Lax;`,
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
