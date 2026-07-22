import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "tailwindcss";
import autoprefixer from "autoprefixer";
import css from "rollup-plugin-css-only";
import { execSync } from "child_process";

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

export default defineConfig(({ command }) => ({
    base: "./",
    plugins: [svelte(), css({ output: "bundle.css" })],
    css: {
        postcss: {
            plugins: [autoprefixer(), tailwindcss()],
        },
    },
    build: {
        sourcemap: true,
        rollupOptions: {
            output: {
                inlineDynamicImports: true,
            },
        },
    },
    server: command === "serve" ? getDevServerConfig() : undefined,
}));
