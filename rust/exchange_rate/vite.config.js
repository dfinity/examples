const isProduction = process.env.DFX_NETWORK === "ic";

import { svelte } from "@sveltejs/vite-plugin-svelte";
import path from "path";
import { defineConfig } from "vite";
import EnvironmentPlugin from "vite-plugin-environment";

export default defineConfig({
  mode: isProduction ? "production" : "development",
  root: path.resolve(__dirname, "src", "frontend"),
  build: {
    outDir: path.resolve(__dirname, "src", "frontend", "public"),
    emptyOutDir: true,
  },
  define: {
    global: "window",
  },
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:8080",
        changeOrigin: true,
      },
    },
  },
  plugins: [
    svelte({}),
    EnvironmentPlugin("all", { prefix: "CANISTER_" }),
    EnvironmentPlugin("all", { prefix: "DFX_" }),
  ],
});
