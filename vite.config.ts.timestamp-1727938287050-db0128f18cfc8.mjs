// vite.config.ts
import AutoImport from "file:///G:/Code/desu.life/meowpad_config/node_modules/.pnpm/unplugin-auto-import@0.18.3_rollup@4.12.1_webpack-sources@3.2.3/node_modules/unplugin-auto-import/dist/vite.js";
import Components from "file:///G:/Code/desu.life/meowpad_config/node_modules/.pnpm/unplugin-vue-components@0.27.4_@babel+parser@7.25.6_rollup@4.12.1_vue@3.4.21_typescript@5.4.2__webpack-sources@3.2.3/node_modules/unplugin-vue-components/dist/vite.js";
import { NaiveUiResolver } from "file:///G:/Code/desu.life/meowpad_config/node_modules/.pnpm/unplugin-vue-components@0.27.4_@babel+parser@7.25.6_rollup@4.12.1_vue@3.4.21_typescript@5.4.2__webpack-sources@3.2.3/node_modules/unplugin-vue-components/dist/resolvers.js";
import { loadEnv } from "file:///G:/Code/desu.life/meowpad_config/node_modules/.pnpm/vite@5.1.7_@types+node@20.11.25_sass@1.71.1/node_modules/vite/dist/node/index.js";
import { resolve } from "path";
import vue from "file:///G:/Code/desu.life/meowpad_config/node_modules/.pnpm/@vitejs+plugin-vue@5.1.4_vite@5.1.7_@types+node@20.11.25_sass@1.71.1__vue@3.4.21_typescript@5.4.2_/node_modules/@vitejs/plugin-vue/dist/index.mjs";
import vueJsx from "file:///G:/Code/desu.life/meowpad_config/node_modules/.pnpm/@vitejs+plugin-vue-jsx@4.0.1_vite@5.1.7_@types+node@20.11.25_sass@1.71.1__vue@3.4.21_typescript@5.4.2_/node_modules/@vitejs/plugin-vue-jsx/dist/index.mjs";
import { format } from "file:///G:/Code/desu.life/meowpad_config/node_modules/.pnpm/date-fns@4.1.0/node_modules/date-fns/index.js";

// package.json
var package_default = {
  name: "meowpad_configurator_v2",
  private: true,
  version: "1.0.2",
  type: "module",
  scripts: {
    dev: "vite",
    build: "vue-tsc --noEmit && vite build",
    preview: "vite preview",
    tauri: "tauri"
  },
  dependencies: {
    "@tauri-apps/api": "1.6.0",
    "@vitejs/plugin-vue-jsx": "4.0.1",
    "animate.css": "^4.1.1",
    autoprefixer: "^10.4.18",
    "date-fns": "^4.1.0",
    mitt: "^3.0.1",
    "naive-ui": "^2.38.1",
    pinia: "^2.1.7",
    postcss: "^8.4.35",
    sass: "^1.71.1",
    "tauri-plugin-store-api": "github:tauri-apps/tauri-plugin-store#v1",
    "unplugin-auto-import": "0.18.3",
    "unplugin-vue-components": "0.27.4",
    vfonts: "0.1.0",
    vue: "^3.4.21",
    "vue-i18n": "10.0.3"
  },
  devDependencies: {
    "@tauri-apps/cli": "1.6.2",
    "@types/node": "^20.11.25",
    "@vicons/fluent": "^0.12.0",
    "@vicons/ionicons5": "^0.12.0",
    "@vitejs/plugin-vue": "5.1.4",
    typescript: "^5.4.2",
    vite: "^5.1.7",
    "vue-tsc": "2.1.6"
  }
};

// vite.config.ts
var { dependencies, devDependencies, name, version } = package_default;
var __APP_INFO__ = {
  pkg: { dependencies, devDependencies, name, version },
  lastBuildTime: format(/* @__PURE__ */ new Date(), "yyyy-MM-dd HH:mm:ss")
};
function pathResolve(dir) {
  return resolve(process.cwd(), ".", dir);
}
var vite_config_default = ({ command, mode }) => {
  const root = process.cwd();
  const env = loadEnv(mode, root);
  const isBuild = command === "build";
  return {
    base: "./",
    esbuild: {},
    plugins: [
      vue(),
      vueJsx(),
      AutoImport({
        imports: [
          "vue",
          {
            "naive-ui": ["useDialog", "useMessage", "useNotification", "useLoadingBar"]
          }
        ]
      }),
      Components({
        resolvers: [NaiveUiResolver()]
      })
    ],
    resolve: {
      alias: [
        {
          find: "@",
          replacement: pathResolve("src") + "/"
        }
      ],
      dedupe: ["vue"]
    },
    define: {
      __APP_INFO__: JSON.stringify(__APP_INFO__)
    },
    css: {
      preprocessorOptions: {}
    },
    server: {
      host: true,
      port: 5777,
      strictPort: true
      // proxy: {
      //     '/api': {
      //         target: '',
      //         changeOrigin: true,
      //         rewrite: (path) => path.replace(/^\/api/, '/api/v1')
      //     }
      // }
    },
    clearScreen: false,
    envPrefix: ["VITE_", "TAURI_PLATFORM", "TAURI_ARCH", "TAURI_FAMILY", "TAURI_PLATFORM_VERSION", "TAURI_PLATFORM_TYPE", "TAURI_DEBUG"],
    build: {
      // Tauri uses Chromium on Windows and WebKit on macOS and Linux
      target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
      // don't minify for debug builds
      minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
      // produce sourcemaps for debug builds
      sourcemap: !!process.env.TAURI_DEBUG
    }
  };
};
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiLCAicGFja2FnZS5qc29uIl0sCiAgInNvdXJjZXNDb250ZW50IjogWyJjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZGlybmFtZSA9IFwiRzpcXFxcQ29kZVxcXFxkZXN1LmxpZmVcXFxcbWVvd3BhZF9jb25maWdcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZmlsZW5hbWUgPSBcIkc6XFxcXENvZGVcXFxcZGVzdS5saWZlXFxcXG1lb3dwYWRfY29uZmlnXFxcXHZpdGUuY29uZmlnLnRzXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ltcG9ydF9tZXRhX3VybCA9IFwiZmlsZTovLy9HOi9Db2RlL2Rlc3UubGlmZS9tZW93cGFkX2NvbmZpZy92aXRlLmNvbmZpZy50c1wiO2ltcG9ydCB0eXBlIHsgVXNlckNvbmZpZywgQ29uZmlnRW52IH0gZnJvbSBcInZpdGVcIjtcclxuaW1wb3J0IHsgZGVmaW5lQ29uZmlnIH0gZnJvbSAndml0ZSdcclxuaW1wb3J0IEF1dG9JbXBvcnQgZnJvbSAndW5wbHVnaW4tYXV0by1pbXBvcnQvdml0ZSdcclxuaW1wb3J0IENvbXBvbmVudHMgZnJvbSAndW5wbHVnaW4tdnVlLWNvbXBvbmVudHMvdml0ZSdcclxuaW1wb3J0IHsgTmFpdmVVaVJlc29sdmVyIH0gZnJvbSAndW5wbHVnaW4tdnVlLWNvbXBvbmVudHMvcmVzb2x2ZXJzJ1xyXG5pbXBvcnQgeyBsb2FkRW52IH0gZnJvbSBcInZpdGVcIjtcclxuaW1wb3J0IHsgcmVzb2x2ZSB9IGZyb20gXCJwYXRoXCI7XHJcbmltcG9ydCB2dWUgZnJvbSBcIkB2aXRlanMvcGx1Z2luLXZ1ZVwiO1xyXG5pbXBvcnQgdnVlSnN4IGZyb20gXCJAdml0ZWpzL3BsdWdpbi12dWUtanN4XCI7XHJcbmltcG9ydCB7IGZvcm1hdCB9IGZyb20gXCJkYXRlLWZuc1wiO1xyXG5pbXBvcnQgcGtnIGZyb20gXCIuL3BhY2thZ2UuanNvblwiO1xyXG5jb25zdCB7IGRlcGVuZGVuY2llcywgZGV2RGVwZW5kZW5jaWVzLCBuYW1lLCB2ZXJzaW9uIH0gPSBwa2c7XHJcblxyXG5jb25zdCBfX0FQUF9JTkZPX18gPSB7XHJcbiAgcGtnOiB7IGRlcGVuZGVuY2llcywgZGV2RGVwZW5kZW5jaWVzLCBuYW1lLCB2ZXJzaW9uIH0sXHJcbiAgbGFzdEJ1aWxkVGltZTogZm9ybWF0KG5ldyBEYXRlKCksIFwieXl5eS1NTS1kZCBISDptbTpzc1wiKSxcclxufTtcclxuXHJcbmZ1bmN0aW9uIHBhdGhSZXNvbHZlKGRpcjogc3RyaW5nKSB7XHJcbiAgcmV0dXJuIHJlc29sdmUocHJvY2Vzcy5jd2QoKSwgXCIuXCIsIGRpcik7XHJcbn1cclxuXHJcbmV4cG9ydCBkZWZhdWx0ICh7IGNvbW1hbmQsIG1vZGUgfTogQ29uZmlnRW52KTogVXNlckNvbmZpZyA9PiB7XHJcbiAgY29uc3Qgcm9vdCA9IHByb2Nlc3MuY3dkKCk7XHJcbiAgY29uc3QgZW52ID0gbG9hZEVudihtb2RlLCByb290KTtcclxuICBjb25zdCBpc0J1aWxkID0gY29tbWFuZCA9PT0gXCJidWlsZFwiO1xyXG4gIHJldHVybiB7XHJcbiAgICBiYXNlOiBcIi4vXCIsXHJcbiAgICBlc2J1aWxkOiB7fSxcclxuICAgIHBsdWdpbnM6IFtcclxuICAgICAgdnVlKCksXHJcbiAgICAgIHZ1ZUpzeCgpLFxyXG4gICAgICBBdXRvSW1wb3J0KHtcclxuICAgICAgICBpbXBvcnRzOiBbXHJcbiAgICAgICAgICAndnVlJyxcclxuICAgICAgICAgIHtcclxuICAgICAgICAgICAgJ25haXZlLXVpJzogWyd1c2VEaWFsb2cnLCAndXNlTWVzc2FnZScsICd1c2VOb3RpZmljYXRpb24nLCAndXNlTG9hZGluZ0JhciddLFxyXG4gICAgICAgICAgfSxcclxuICAgICAgICBdLFxyXG4gICAgICB9KSxcclxuICAgICAgQ29tcG9uZW50cyh7XHJcbiAgICAgICAgcmVzb2x2ZXJzOiBbTmFpdmVVaVJlc29sdmVyKCldXHJcbiAgICAgIH0pXHJcbiAgICBdLFxyXG4gICAgcmVzb2x2ZToge1xyXG4gICAgICBhbGlhczogW1xyXG4gICAgICAgIHtcclxuICAgICAgICAgIGZpbmQ6IFwiQFwiLFxyXG4gICAgICAgICAgcmVwbGFjZW1lbnQ6IHBhdGhSZXNvbHZlKFwic3JjXCIpICsgXCIvXCIsXHJcbiAgICAgICAgfSxcclxuICAgICAgXSxcclxuICAgICAgZGVkdXBlOiBbXCJ2dWVcIl0sXHJcbiAgICB9LFxyXG4gICAgZGVmaW5lOiB7XHJcbiAgICAgIF9fQVBQX0lORk9fXzogSlNPTi5zdHJpbmdpZnkoX19BUFBfSU5GT19fKSxcclxuICAgIH0sXHJcbiAgICBjc3M6IHtcclxuICAgICAgcHJlcHJvY2Vzc29yT3B0aW9uczoge30sXHJcbiAgICB9LFxyXG4gICAgc2VydmVyOiB7XHJcbiAgICAgIGhvc3Q6IHRydWUsXHJcbiAgICAgIHBvcnQ6IDU3NzcsXHJcbiAgICAgIHN0cmljdFBvcnQ6IHRydWUsXHJcbiAgICAgIC8vIHByb3h5OiB7XHJcbiAgICAgIC8vICAgICAnL2FwaSc6IHtcclxuICAgICAgLy8gICAgICAgICB0YXJnZXQ6ICcnLFxyXG4gICAgICAvLyAgICAgICAgIGNoYW5nZU9yaWdpbjogdHJ1ZSxcclxuICAgICAgLy8gICAgICAgICByZXdyaXRlOiAocGF0aCkgPT4gcGF0aC5yZXBsYWNlKC9eXFwvYXBpLywgJy9hcGkvdjEnKVxyXG4gICAgICAvLyAgICAgfVxyXG4gICAgICAvLyB9XHJcbiAgICB9LFxyXG4gICAgY2xlYXJTY3JlZW46IGZhbHNlLFxyXG4gICAgZW52UHJlZml4OiBbJ1ZJVEVfJywgJ1RBVVJJX1BMQVRGT1JNJywgJ1RBVVJJX0FSQ0gnLCAnVEFVUklfRkFNSUxZJywgJ1RBVVJJX1BMQVRGT1JNX1ZFUlNJT04nLCAnVEFVUklfUExBVEZPUk1fVFlQRScsICdUQVVSSV9ERUJVRyddLFxyXG4gICAgYnVpbGQ6IHtcclxuICAgICAgLy8gVGF1cmkgdXNlcyBDaHJvbWl1bSBvbiBXaW5kb3dzIGFuZCBXZWJLaXQgb24gbWFjT1MgYW5kIExpbnV4XHJcbiAgICAgIHRhcmdldDogcHJvY2Vzcy5lbnYuVEFVUklfUExBVEZPUk0gPT0gJ3dpbmRvd3MnID8gJ2Nocm9tZTEwNScgOiAnc2FmYXJpMTMnLFxyXG4gICAgICAvLyBkb24ndCBtaW5pZnkgZm9yIGRlYnVnIGJ1aWxkc1xyXG4gICAgICBtaW5pZnk6ICFwcm9jZXNzLmVudi5UQVVSSV9ERUJVRyA/ICdlc2J1aWxkJyA6IGZhbHNlLFxyXG4gICAgICAvLyBwcm9kdWNlIHNvdXJjZW1hcHMgZm9yIGRlYnVnIGJ1aWxkc1xyXG4gICAgICBzb3VyY2VtYXA6ICEhcHJvY2Vzcy5lbnYuVEFVUklfREVCVUcsXHJcbiAgICB9LFxyXG4gIH07XHJcbn07XHJcbiIsICJ7XG4gIFwibmFtZVwiOiBcIm1lb3dwYWRfY29uZmlndXJhdG9yX3YyXCIsXG4gIFwicHJpdmF0ZVwiOiB0cnVlLFxuICBcInZlcnNpb25cIjogXCIxLjAuMlwiLFxuICBcInR5cGVcIjogXCJtb2R1bGVcIixcbiAgXCJzY3JpcHRzXCI6IHtcbiAgICBcImRldlwiOiBcInZpdGVcIixcbiAgICBcImJ1aWxkXCI6IFwidnVlLXRzYyAtLW5vRW1pdCAmJiB2aXRlIGJ1aWxkXCIsXG4gICAgXCJwcmV2aWV3XCI6IFwidml0ZSBwcmV2aWV3XCIsXG4gICAgXCJ0YXVyaVwiOiBcInRhdXJpXCJcbiAgfSxcbiAgXCJkZXBlbmRlbmNpZXNcIjoge1xuICAgIFwiQHRhdXJpLWFwcHMvYXBpXCI6IFwiMS42LjBcIixcbiAgICBcIkB2aXRlanMvcGx1Z2luLXZ1ZS1qc3hcIjogXCI0LjAuMVwiLFxuICAgIFwiYW5pbWF0ZS5jc3NcIjogXCJeNC4xLjFcIixcbiAgICBcImF1dG9wcmVmaXhlclwiOiBcIl4xMC40LjE4XCIsXG4gICAgXCJkYXRlLWZuc1wiOiBcIl40LjEuMFwiLFxuICAgIFwibWl0dFwiOiBcIl4zLjAuMVwiLFxuICAgIFwibmFpdmUtdWlcIjogXCJeMi4zOC4xXCIsXG4gICAgXCJwaW5pYVwiOiBcIl4yLjEuN1wiLFxuICAgIFwicG9zdGNzc1wiOiBcIl44LjQuMzVcIixcbiAgICBcInNhc3NcIjogXCJeMS43MS4xXCIsXG4gICAgXCJ0YXVyaS1wbHVnaW4tc3RvcmUtYXBpXCI6IFwiZ2l0aHViOnRhdXJpLWFwcHMvdGF1cmktcGx1Z2luLXN0b3JlI3YxXCIsXG4gICAgXCJ1bnBsdWdpbi1hdXRvLWltcG9ydFwiOiBcIjAuMTguM1wiLFxuICAgIFwidW5wbHVnaW4tdnVlLWNvbXBvbmVudHNcIjogXCIwLjI3LjRcIixcbiAgICBcInZmb250c1wiOiBcIjAuMS4wXCIsXG4gICAgXCJ2dWVcIjogXCJeMy40LjIxXCIsXG4gICAgXCJ2dWUtaTE4blwiOiBcIjEwLjAuM1wiXG4gIH0sXG4gIFwiZGV2RGVwZW5kZW5jaWVzXCI6IHtcbiAgICBcIkB0YXVyaS1hcHBzL2NsaVwiOiBcIjEuNi4yXCIsXG4gICAgXCJAdHlwZXMvbm9kZVwiOiBcIl4yMC4xMS4yNVwiLFxuICAgIFwiQHZpY29ucy9mbHVlbnRcIjogXCJeMC4xMi4wXCIsXG4gICAgXCJAdmljb25zL2lvbmljb25zNVwiOiBcIl4wLjEyLjBcIixcbiAgICBcIkB2aXRlanMvcGx1Z2luLXZ1ZVwiOiBcIjUuMS40XCIsXG4gICAgXCJ0eXBlc2NyaXB0XCI6IFwiXjUuNC4yXCIsXG4gICAgXCJ2aXRlXCI6IFwiXjUuMS43XCIsXG4gICAgXCJ2dWUtdHNjXCI6IFwiMi4xLjZcIlxuICB9XG59XG4iXSwKICAibWFwcGluZ3MiOiAiO0FBRUEsT0FBTyxnQkFBZ0I7QUFDdkIsT0FBTyxnQkFBZ0I7QUFDdkIsU0FBUyx1QkFBdUI7QUFDaEMsU0FBUyxlQUFlO0FBQ3hCLFNBQVMsZUFBZTtBQUN4QixPQUFPLFNBQVM7QUFDaEIsT0FBTyxZQUFZO0FBQ25CLFNBQVMsY0FBYzs7O0FDVHZCO0FBQUEsRUFDRSxNQUFRO0FBQUEsRUFDUixTQUFXO0FBQUEsRUFDWCxTQUFXO0FBQUEsRUFDWCxNQUFRO0FBQUEsRUFDUixTQUFXO0FBQUEs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
