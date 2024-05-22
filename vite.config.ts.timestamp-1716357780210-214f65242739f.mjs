// vite.config.ts
import AutoImport from "file:///G:/Code/desu.life/meowpad_config/node_modules/.pnpm/unplugin-auto-import@0.17.5/node_modules/unplugin-auto-import/dist/vite.js";
import Components from "file:///G:/Code/desu.life/meowpad_config/node_modules/.pnpm/unplugin-vue-components@0.26.0_vue@3.4.21/node_modules/unplugin-vue-components/dist/vite.js";
import { NaiveUiResolver } from "file:///G:/Code/desu.life/meowpad_config/node_modules/.pnpm/unplugin-vue-components@0.26.0_vue@3.4.21/node_modules/unplugin-vue-components/dist/resolvers.js";
import { loadEnv } from "file:///G:/Code/desu.life/meowpad_config/node_modules/.pnpm/vite@5.1.5_@types+node@20.11.25_sass@1.71.1/node_modules/vite/dist/node/index.js";
import { resolve } from "path";
import vue from "file:///G:/Code/desu.life/meowpad_config/node_modules/.pnpm/@vitejs+plugin-vue@4.6.2_vite@5.1.5_vue@3.4.21/node_modules/@vitejs/plugin-vue/dist/index.mjs";
import vueJsx from "file:///G:/Code/desu.life/meowpad_config/node_modules/.pnpm/@vitejs+plugin-vue-jsx@3.1.0_vite@5.1.5_vue@3.4.21/node_modules/@vitejs/plugin-vue-jsx/dist/index.mjs";
import { format } from "file:///G:/Code/desu.life/meowpad_config/node_modules/.pnpm/date-fns@3.3.1/node_modules/date-fns/index.mjs";

// package.json
var package_default = {
  name: "meowpad_configurator_v2",
  private: true,
  version: "1.0.1",
  type: "module",
  scripts: {
    dev: "vite",
    build: "vue-tsc --noEmit && vite build",
    preview: "vite preview",
    tauri: "tauri"
  },
  dependencies: {
    "@tauri-apps/api": "^1.5.3",
    "@vitejs/plugin-vue-jsx": "^3.1.0",
    "animate.css": "^4.1.1",
    autoprefixer: "^10.4.18",
    "date-fns": "^3.3.1",
    "naive-ui": "^2.38.1",
    pinia: "^2.1.7",
    postcss: "^8.4.32",
    sass: "^1.71.1",
    "unplugin-auto-import": "^0.17.5",
    "unplugin-vue-components": "^0.26.0",
    vfonts: "^0.0.3",
    vue: "^3.4.21",
    "vue-i18n": "^9.10.1"
  },
  devDependencies: {
    "@tauri-apps/cli": "^1.5.10",
    "@types/node": "^20.10.3",
    "@vicons/fluent": "^0.12.0",
    "@vicons/ionicons5": "^0.12.0",
    "@vitejs/plugin-vue": "^4.6.2",
    typescript: "^5.4.2",
    vite: "^5.1.7",
    "vue-tsc": "^1.8.27"
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
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiLCAicGFja2FnZS5qc29uIl0sCiAgInNvdXJjZXNDb250ZW50IjogWyJjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZGlybmFtZSA9IFwiRzpcXFxcQ29kZVxcXFxkZXN1LmxpZmVcXFxcbWVvd3BhZF9jb25maWdcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZmlsZW5hbWUgPSBcIkc6XFxcXENvZGVcXFxcZGVzdS5saWZlXFxcXG1lb3dwYWRfY29uZmlnXFxcXHZpdGUuY29uZmlnLnRzXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ltcG9ydF9tZXRhX3VybCA9IFwiZmlsZTovLy9HOi9Db2RlL2Rlc3UubGlmZS9tZW93cGFkX2NvbmZpZy92aXRlLmNvbmZpZy50c1wiO2ltcG9ydCB0eXBlIHsgVXNlckNvbmZpZywgQ29uZmlnRW52IH0gZnJvbSBcInZpdGVcIjtcclxuaW1wb3J0IHsgZGVmaW5lQ29uZmlnIH0gZnJvbSAndml0ZSdcclxuaW1wb3J0IEF1dG9JbXBvcnQgZnJvbSAndW5wbHVnaW4tYXV0by1pbXBvcnQvdml0ZSdcclxuaW1wb3J0IENvbXBvbmVudHMgZnJvbSAndW5wbHVnaW4tdnVlLWNvbXBvbmVudHMvdml0ZSdcclxuaW1wb3J0IHsgTmFpdmVVaVJlc29sdmVyIH0gZnJvbSAndW5wbHVnaW4tdnVlLWNvbXBvbmVudHMvcmVzb2x2ZXJzJ1xyXG5pbXBvcnQgeyBsb2FkRW52IH0gZnJvbSBcInZpdGVcIjtcclxuaW1wb3J0IHsgcmVzb2x2ZSB9IGZyb20gXCJwYXRoXCI7XHJcbmltcG9ydCB2dWUgZnJvbSBcIkB2aXRlanMvcGx1Z2luLXZ1ZVwiO1xyXG5pbXBvcnQgdnVlSnN4IGZyb20gXCJAdml0ZWpzL3BsdWdpbi12dWUtanN4XCI7XHJcbmltcG9ydCB7IGZvcm1hdCB9IGZyb20gXCJkYXRlLWZuc1wiO1xyXG5pbXBvcnQgcGtnIGZyb20gXCIuL3BhY2thZ2UuanNvblwiO1xyXG5jb25zdCB7IGRlcGVuZGVuY2llcywgZGV2RGVwZW5kZW5jaWVzLCBuYW1lLCB2ZXJzaW9uIH0gPSBwa2c7XHJcblxyXG5jb25zdCBfX0FQUF9JTkZPX18gPSB7XHJcbiAgcGtnOiB7IGRlcGVuZGVuY2llcywgZGV2RGVwZW5kZW5jaWVzLCBuYW1lLCB2ZXJzaW9uIH0sXHJcbiAgbGFzdEJ1aWxkVGltZTogZm9ybWF0KG5ldyBEYXRlKCksIFwieXl5eS1NTS1kZCBISDptbTpzc1wiKSxcclxufTtcclxuXHJcbmZ1bmN0aW9uIHBhdGhSZXNvbHZlKGRpcjogc3RyaW5nKSB7XHJcbiAgcmV0dXJuIHJlc29sdmUocHJvY2Vzcy5jd2QoKSwgXCIuXCIsIGRpcik7XHJcbn1cclxuXHJcbmV4cG9ydCBkZWZhdWx0ICh7IGNvbW1hbmQsIG1vZGUgfTogQ29uZmlnRW52KTogVXNlckNvbmZpZyA9PiB7XHJcbiAgY29uc3Qgcm9vdCA9IHByb2Nlc3MuY3dkKCk7XHJcbiAgY29uc3QgZW52ID0gbG9hZEVudihtb2RlLCByb290KTtcclxuICBjb25zdCBpc0J1aWxkID0gY29tbWFuZCA9PT0gXCJidWlsZFwiO1xyXG4gIHJldHVybiB7XHJcbiAgICBiYXNlOiBcIi4vXCIsXHJcbiAgICBlc2J1aWxkOiB7fSxcclxuICAgIHBsdWdpbnM6IFtcclxuICAgICAgdnVlKCksXHJcbiAgICAgIHZ1ZUpzeCgpLFxyXG4gICAgICBBdXRvSW1wb3J0KHtcclxuICAgICAgICBpbXBvcnRzOiBbXHJcbiAgICAgICAgICAndnVlJyxcclxuICAgICAgICAgIHtcclxuICAgICAgICAgICAgJ25haXZlLXVpJzogWyd1c2VEaWFsb2cnLCAndXNlTWVzc2FnZScsICd1c2VOb3RpZmljYXRpb24nLCAndXNlTG9hZGluZ0JhciddLFxyXG4gICAgICAgICAgfSxcclxuICAgICAgICBdLFxyXG4gICAgICB9KSxcclxuICAgICAgQ29tcG9uZW50cyh7XHJcbiAgICAgICAgcmVzb2x2ZXJzOiBbTmFpdmVVaVJlc29sdmVyKCldXHJcbiAgICAgIH0pXHJcbiAgICBdLFxyXG4gICAgcmVzb2x2ZToge1xyXG4gICAgICBhbGlhczogW1xyXG4gICAgICAgIHtcclxuICAgICAgICAgIGZpbmQ6IFwiQFwiLFxyXG4gICAgICAgICAgcmVwbGFjZW1lbnQ6IHBhdGhSZXNvbHZlKFwic3JjXCIpICsgXCIvXCIsXHJcbiAgICAgICAgfSxcclxuICAgICAgXSxcclxuICAgICAgZGVkdXBlOiBbXCJ2dWVcIl0sXHJcbiAgICB9LFxyXG4gICAgZGVmaW5lOiB7XHJcbiAgICAgIF9fQVBQX0lORk9fXzogSlNPTi5zdHJpbmdpZnkoX19BUFBfSU5GT19fKSxcclxuICAgIH0sXHJcbiAgICBjc3M6IHtcclxuICAgICAgcHJlcHJvY2Vzc29yT3B0aW9uczoge30sXHJcbiAgICB9LFxyXG4gICAgc2VydmVyOiB7XHJcbiAgICAgIGhvc3Q6IHRydWUsXHJcbiAgICAgIHBvcnQ6IDU3NzcsXHJcbiAgICAgIHN0cmljdFBvcnQ6IHRydWUsXHJcbiAgICAgIC8vIHByb3h5OiB7XHJcbiAgICAgIC8vICAgICAnL2FwaSc6IHtcclxuICAgICAgLy8gICAgICAgICB0YXJnZXQ6ICcnLFxyXG4gICAgICAvLyAgICAgICAgIGNoYW5nZU9yaWdpbjogdHJ1ZSxcclxuICAgICAgLy8gICAgICAgICByZXdyaXRlOiAocGF0aCkgPT4gcGF0aC5yZXBsYWNlKC9eXFwvYXBpLywgJy9hcGkvdjEnKVxyXG4gICAgICAvLyAgICAgfVxyXG4gICAgICAvLyB9XHJcbiAgICB9LFxyXG4gICAgY2xlYXJTY3JlZW46IGZhbHNlLFxyXG4gICAgZW52UHJlZml4OiBbJ1ZJVEVfJywgJ1RBVVJJX1BMQVRGT1JNJywgJ1RBVVJJX0FSQ0gnLCAnVEFVUklfRkFNSUxZJywgJ1RBVVJJX1BMQVRGT1JNX1ZFUlNJT04nLCAnVEFVUklfUExBVEZPUk1fVFlQRScsICdUQVVSSV9ERUJVRyddLFxyXG4gICAgYnVpbGQ6IHtcclxuICAgICAgLy8gVGF1cmkgdXNlcyBDaHJvbWl1bSBvbiBXaW5kb3dzIGFuZCBXZWJLaXQgb24gbWFjT1MgYW5kIExpbnV4XHJcbiAgICAgIHRhcmdldDogcHJvY2Vzcy5lbnYuVEFVUklfUExBVEZPUk0gPT0gJ3dpbmRvd3MnID8gJ2Nocm9tZTEwNScgOiAnc2FmYXJpMTMnLFxyXG4gICAgICAvLyBkb24ndCBtaW5pZnkgZm9yIGRlYnVnIGJ1aWxkc1xyXG4gICAgICBtaW5pZnk6ICFwcm9jZXNzLmVudi5UQVVSSV9ERUJVRyA/ICdlc2J1aWxkJyA6IGZhbHNlLFxyXG4gICAgICAvLyBwcm9kdWNlIHNvdXJjZW1hcHMgZm9yIGRlYnVnIGJ1aWxkc1xyXG4gICAgICBzb3VyY2VtYXA6ICEhcHJvY2Vzcy5lbnYuVEFVUklfREVCVUcsXHJcbiAgICB9LFxyXG4gIH07XHJcbn07XHJcbiIsICJ7XG4gIFwibmFtZVwiOiBcIm1lb3dwYWRfY29uZmlndXJhdG9yX3YyXCIsXG4gIFwicHJpdmF0ZVwiOiB0cnVlLFxuICBcInZlcnNpb25cIjogXCIxLjAuMVwiLFxuICBcInR5cGVcIjogXCJtb2R1bGVcIixcbiAgXCJzY3JpcHRzXCI6IHtcbiAgICBcImRldlwiOiBcInZpdGVcIixcbiAgICBcImJ1aWxkXCI6IFwidnVlLXRzYyAtLW5vRW1pdCAmJiB2aXRlIGJ1aWxkXCIsXG4gICAgXCJwcmV2aWV3XCI6IFwidml0ZSBwcmV2aWV3XCIsXG4gICAgXCJ0YXVyaVwiOiBcInRhdXJpXCJcbiAgfSxcbiAgXCJkZXBlbmRlbmNpZXNcIjoge1xuICAgIFwiQHRhdXJpLWFwcHMvYXBpXCI6IFwiXjEuNS4zXCIsXG4gICAgXCJAdml0ZWpzL3BsdWdpbi12dWUtanN4XCI6IFwiXjMuMS4wXCIsXG4gICAgXCJhbmltYXRlLmNzc1wiOiBcIl40LjEuMVwiLFxuICAgIFwiYXV0b3ByZWZpeGVyXCI6IFwiXjEwLjQuMThcIixcbiAgICBcImRhdGUtZm5zXCI6IFwiXjMuMy4xXCIsXG4gICAgXCJuYWl2ZS11aVwiOiBcIl4yLjM4LjFcIixcbiAgICBcInBpbmlhXCI6IFwiXjIuMS43XCIsXG4gICAgXCJwb3N0Y3NzXCI6IFwiXjguNC4zMlwiLFxuICAgIFwic2Fzc1wiOiBcIl4xLjcxLjFcIixcbiAgICBcInVucGx1Z2luLWF1dG8taW1wb3J0XCI6IFwiXjAuMTcuNVwiLFxuICAgIFwidW5wbHVnaW4tdnVlLWNvbXBvbmVudHNcIjogXCJeMC4yNi4wXCIsXG4gICAgXCJ2Zm9udHNcIjogXCJeMC4wLjNcIixcbiAgICBcInZ1ZVwiOiBcIl4zLjQuMjFcIixcbiAgICBcInZ1ZS1pMThuXCI6IFwiXjkuMTAuMVwiXG4gIH0sXG4gIFwiZGV2RGVwZW5kZW5jaWVzXCI6IHtcbiAgICBcIkB0YXVyaS1hcHBzL2NsaVwiOiBcIl4xLjUuMTBcIixcbiAgICBcIkB0eXBlcy9ub2RlXCI6IFwiXjIwLjEwLjNcIixcbiAgICBcIkB2aWNvbnMvZmx1ZW50XCI6IFwiXjAuMTIuMFwiLFxuICAgIFwiQHZpY29ucy9pb25pY29uczVcIjogXCJeMC4xMi4wXCIsXG4gICAgXCJAdml0ZWpzL3BsdWdpbi12dWVcIjogXCJeNC42LjJcIixcbiAgICBcInR5cGVzY3JpcHRcIjogXCJeNS40LjJcIixcbiAgICBcInZpdGVcIjogXCJeNS4xLjdcIixcbiAgICBcInZ1ZS10c2NcIjogXCJeMS44LjI3XCJcbiAgfVxufVxuIl0sCiAgIm1hcHBpbmdzIjogIjtBQUVBLE9BQU8sZ0JBQWdCO0FBQ3ZCLE9BQU8sZ0JBQWdCO0FBQ3ZCLFNBQVMsdUJBQXVCO0FBQ2hDLFNBQVMsZUFBZTtBQUN4QixTQUFTLGVBQWU7QUFDeEIsT0FBTyxTQUFTO0FBQ2hCLE9BQU8sWUFBWTtBQUNuQixTQUFTLGNBQWM7OztBQ1R2QjtBQUFBLEVBQ0UsTUFBUTtBQUFBLEVBQ1IsU0FBVztBQUFBLEVBQ1gsU0FBVztBQUFBLEVBQ1gsTUFBUTtBQUFBLEVBQ1IsU0FBVztBQUFBLElBQ1QsS0FBTztBQUFBLElBQ1AsT0FBUztBQUFBLElBQ1QsU0FBVztBQUFBLElBQ1gsT0FBUztBQUFBLEVBQ1g7QUFBQSxFQUNBLGNBQWdCO0FBQUEsSUFDZCxtQkFB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
