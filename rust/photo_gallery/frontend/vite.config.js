import { fileURLToPath, URL } from 'url';
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';
// This is needed to translate `process.env.` variables
import environment from 'vite-plugin-environment';
import dotenv from 'dotenv';

dotenv.config({ path: '../.env' });
console.log("Loaded env:", process.env.DFX_NETWORK);

export default defineConfig({
  build: {
    emptyOutDir: true,
    minify: false,
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
    react(),
    environment("all", { prefix: "CANISTER_" }),
    environment("all", { prefix: "DFX_" }),
  ],
  resolve: {
    dedupe: ['@dfinity/agent'],
  },
  envPrefix: ['VITE_', 'DFX_', 'CANISTER_'], // use to interpolate `import.meta.env` variables
});
