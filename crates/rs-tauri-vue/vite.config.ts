/// <reference types="vitest" />
import vue from '@vitejs/plugin-vue'
import {resolve} from 'path'
import AutoImport from 'unplugin-auto-import/vite'
import {defineConfig} from 'vite'
import Components from 'unplugin-vue-components/vite'
import {NaiveUiResolver} from 'unplugin-vue-components/resolvers'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        vue(),
        AutoImport({
            imports: [
                'vue',

                /*
                * TODO X: 关键补充, 一些组件加载不正常, 用此解决
                *  - https://www.naiveui.com/zh-CN/light/docs/import-on-demand
                *
                * */
                {
                    'naive-ui': [
                        'useDialog',
                        'useMessage',
                        'useNotification',
                        'useLoadingBar'
                    ]
                }
            ],
            dts: './src/auto-imports.d.ts',
            eslintrc: {
                enabled: true,
                filepath: resolve(__dirname, '.eslintrc-auto-import.json'),
            },
        }),


        /*
        * TODO X: 关键补充, 一些组件加载不正常, 用此解决
        *  - https://www.naiveui.com/zh-CN/light/docs/import-on-demand
        *
        * */
        Components({
            resolvers: [NaiveUiResolver()]
        }),
    ],
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
        port: 5173,
        strictPort: true,
    },
    build: {
        outDir: './dist',
        // See https://tauri.app/v1/references/webview-versions for details
        target: ['es2021', 'chrome100', 'safari13'],
        minify: !!!process.env.TAURI_DEBUG,
        sourcemap: !!process.env.TAURI_DEBUG,
        emptyOutDir: true,
    },
    test: {
        include: ['tests/unit/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
    },
})
