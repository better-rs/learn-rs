<template>
    <n-card item-responsive="true" style="background:lightcoral; margin-bottom:5px" title="网址导航">
        <n-grid :x-gap="12" :y-gap="8" cols="4">
            <!-- 分组 -->
            <n-grid-item v-for="item in webJp">
                <n-button size="small" style="background:lightblue;"
                          @click="open_webview(item.url)">
                    {{ item.title }}
                </n-button>
            </n-grid-item>
        </n-grid>
        <br>
        <n-grid :x-gap="12" :y-gap="8" cols="4">
            <!-- 分组 -->
            <n-grid-item v-for="item in webDocs">
                <n-button size="small" style="background:greenyellow;"
                          @click="open_webview(item.url)">
                    {{ item.title }}
                </n-button>
            </n-grid-item>
        </n-grid>
        <br>
        <n-grid :x-gap="12" :y-gap="8" cols="4">
            <!-- 分组 -->
            <n-grid-item v-for="item in webSearch">
                <n-button size="small" style="background:yellow;"
                          @click="open_webview(item.url)">
                    {{ item.title }}
                </n-button>
            </n-grid-item>
        </n-grid>


    </n-card>
</template>


<script lang="ts">
import {defineComponent, getCurrentInstance, ref} from 'vue'

import {WebviewWindow} from '@tauri-apps/api/window';

// 数据: 需要注册到下面
const webDocs = ref([
    {title: 'Tauri 官网', url: "https://tauri.app/zh-cn/"},
    {title: 'Tauri Docs', url: "https://tauri.app/v1/api/js/"},
    {title: 'NaiveUI Docs', url: "https://www.naiveui.com/zh-CN/light/components/button"},
    {title: 'Vue Docs', url: "https://cn.vuejs.org/guide/essentials/list.html#v-for"},
]);
const webSearch = ref([
    {title: 'Google', url: "https://google.com"},
    {title: 'Bing', url: "https://bing.com"},
    {title: 'Github', url: "https://github.com"},

]);
const webJp = ref([
    {title: 'NHK 日语教程', url: "https://www.nhk.or.jp/lesson/zh/letters/hiragana.html"},
    {title: '日语五十音', url: "https://inihongo.github.io/Japanese_syllabary_learning/index.html"},
    {title: '新华字典', url: "https://zd.hwxnet.com/"},

])


export default defineComponent({

    components: {},
    props: ['waited'],

    data: () => ({
        webDocs,
        webJp,
        webSearch,
        toggle: false,
        counter: 0,
    }),


    setup() {
        const me = getCurrentInstance()

        function log(value: any) {
            console.log(value)
            return value
        }

        // todo x: 打开 webview 窗口
        async function open_webview(url: string) {

            let label = "label" + Math.random().toString(36).slice(2, 7);

            console.log(label);

            const webview = new WebviewWindow(label, {
                url: url
            });
            await webview.once('tauri://created', function () {
                // webview window successfully created
                console.log(url)
            });
            await webview.once('tauri://error', function (e) {
                // an error happened creating the webview window
                console.log(url, e)
            });

        }

        return {
            log,
            open_webview,
            someData: ref(0),
        }
    },

    _beforeRouteEnter() {
        this.toggle = true
    },
})
</script>
