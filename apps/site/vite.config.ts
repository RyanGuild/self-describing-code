import { defineConfig } from "vite";

/** Dev server for the Prezi-style map. Scene JSON lives under `public/`. */
export default defineConfig({
  server: {
    port: 5173,
  },
  publicDir: "public",
});
