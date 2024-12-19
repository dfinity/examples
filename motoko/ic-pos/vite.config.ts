import { TanStackRouterVite } from "@tanstack/router-plugin/vite";
import { defineConfig } from "vite";
import dotenv from "dotenv";
import environment from "vite-plugin-environment";
import viteReact from "@vitejs/plugin-react";
import path from "path"

dotenv.config({ path: ".env" });

process.env.II_URL =
  process.env.DFX_NETWORK === "local"
    ? `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943`
    : `https://identity.ic0.app`;


export default defineConfig({
  build: {
    emptyOutDir: true,
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src/icpos_frontend"),
    },
  },
  optimizeDeps: {
    esbuildOptions: {
      define: {
        global: "globalThis",
      },
    },
  },
  server: {
    proxy: {
      "/api": {
        target: "http://127.0.0.1:4943",
        changeOrigin: true,
      },
    },
  },
  plugins: [
    TanStackRouterVite({
      routesDirectory: "src/icpos_frontend/routes",
      generatedRouteTree: "src/icpos_frontend/routeTree.gen.ts",
    }),
    viteReact(),
    environment("all", { prefix: "CANISTER_" }),
    environment("all", { prefix: "DFX_" }),
    environment(['II_URL']),
  ],
});
