import { defineConfig } from "vite";

import { robbyRunsApiPlugin } from "./vite-runs-api";

/** Dev server for the Prezi-style map. Scene JSON lives under `public/`. */
export default defineConfig({
  server: {
    port: 5173,
    host: "127.0.0.1",
  },
  publicDir: "public",
  plugins: [robbyRunsApiPlugin()],
});
