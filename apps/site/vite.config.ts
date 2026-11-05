import { defineConfig, loadEnv } from "vite";

import { robbyRunsApiPlugin } from "./vite-runs-api";

/**
 * Dev server for the Prezi-style map. Scene JSON lives under `public/`.
 *
 * loadEnv with "" prefix pulls all keys from `.env*` into a map; we copy
 * missing ones onto `process.env` so Allow → `robby metric-test` inherits
 * OPENROUTER_API_KEY (Vite only exposes VITE_* to the browser by default).
 */
export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");
  for (const [key, value] of Object.entries(env)) {
    if (process.env[key] === undefined) {
      process.env[key] = value;
    }
  }

  return {
    server: {
      port: 5173,
      host: "127.0.0.1",
    },
    publicDir: "public",
    plugins: [robbyRunsApiPlugin()],
  };
});
