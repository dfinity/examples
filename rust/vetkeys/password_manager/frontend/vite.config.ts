import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "tailwindcss";
import autoprefixer from "autoprefixer";
import css from "rollup-plugin-css-only";
import { execSync } from "child_process";

const environment = process.env.ICP_ENVIRONMENT || "local";
const CANISTER_NAMES = ["ic_vetkeys_encrypted_maps_canister"];

function getDevServerConfig() {
    const backend = process.env.BACKEND;
    if (!backend) {
        throw new Error(
            "BACKEND env var is required. Use `npm run dev:motoko` or `npm run dev:rust`.",
        );
    }

    const networkStatus = JSON.parse(
        execSync(`icp network status -e ${environment} --json --project-root-override ../${backend}`, {
            encoding: "utf-8",
        }),
    );
    const canisterParams = CANISTER_NAMES.map((name) => {
        const id = execSync(
            `icp canister status ${name} -e ${environment} --id-only --project-root-override ../${backend}`,
            { encoding: "utf-8", stdio: "pipe" },
        ).trim();
        return `PUBLIC_CANISTER_ID:${name}=${id}`;
    }).join("&");
    return {
        headers: {
            "Set-Cookie": `ic_env=${encodeURIComponent(
                `${canisterParams}&ic_root_key=${networkStatus.root_key}`,
            )}; SameSite=Lax;`,
        },
        proxy: {
            "/api": { target: networkStatus.api_url, changeOrigin: true },
        },
        hmr: false,
    };
}

export default defineConfig(({ command }) => ({
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
    root: "./",
    ...(command === "serve" ? { server: getDevServerConfig() } : {}),
}));
