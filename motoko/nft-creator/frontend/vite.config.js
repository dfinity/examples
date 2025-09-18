import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import { fileURLToPath, URL } from "url";
import environment from "vite-plugin-environment";
import tailwindcss from "@tailwindcss/vite";
import dotenv from "dotenv";
import path from "path";

// Load from project root
dotenv.config({ path: path.resolve(__dirname, "../.env") });

process.env.II_URL =
    process.env.DFX_NETWORK === "local"
        ? `http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:4943/`
        : `https://id.ai/`;

export default defineConfig({
    base: "./",
    plugins: [
        react(),
        environment("all", { prefix: "CANISTER_" }),
        environment("all", { prefix: "DFX_" }),
        environment(["II_URL"]),
        tailwindcss(),
    ],
    envDir: "../",
    optimizeDeps: {
        esbuildOptions: {
            define: {
                global: "globalThis",
            },
        },
    },
    resolve: {
        alias: [
            {
                find: "declarations",
                replacement: fileURLToPath(
                    new URL("../src/declarations", import.meta.url)
                ),
            },
        ],
    },
    server: {
        proxy: {
            "/api": {
                target: "http://127.0.0.1:4943",
                changeOrigin: true,
            },
        },
        host: "127.0.0.1",
    },
});
