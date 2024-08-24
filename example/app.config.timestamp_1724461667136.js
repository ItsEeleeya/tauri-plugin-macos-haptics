// app.config.ts
import { defineConfig } from "@solidjs/start/config";
import { internalIpV4 } from "internal-ip";
import path from "path";
var mobile = !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM);
var host = await internalIpV4();
var hmrPort = 5183;
var app_config_default = defineConfig({
  ssr: false,
  server: { preset: "static" },
  vite: () => ({
    resolve: {
      alias: {
        "@": path.resolve("./src")
      }
    },
    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    // 1. tauri expects a fixed port, fail if that port is not available
    server: {
      host: mobile ? "0.0.0.0" : false,
      // listen on all addresses
      port: 1420,
      strictPort: true,
      hmr: mobile ? {
        protocol: "ws",
        host,
        port: hmrPort++
      } : void 0,
      watch: {
        // 2. tell vite to ignore watching `src-tauri`
        ignored: ["**/src-tauri/**"]
      }
    },
    // 3. to make use of `TAURI_DEBUG` and other env variables
    // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
    envPrefix: ["VITE_", "TAURI_"]
  })
});
export {
  app_config_default as default
};
