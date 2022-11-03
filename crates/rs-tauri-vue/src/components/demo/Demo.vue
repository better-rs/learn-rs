<template>

    <n-card title="webview 测试" style="background:lightpink; margin-bottom:5px">
        <n-grid cols="3" align="center">
            <n-grid-item>
                <n-button class="btn" type="success" @click="open_webview('https://bing.com')">
                    webview 打开 bing
                </n-button>
            </n-grid-item>

            <n-grid-item>
                <n-button class="btn" type="success" @click="open_webview('https://google.com')">
                    webview 打开 google
                </n-button>
            </n-grid-item>

            <n-grid-item>
                <n-button class="btn" type="success" @click="open_webview('https://github.com')">
                    webview 打开 github
                </n-button>
            </n-grid-item>
        </n-grid>
    </n-card>


    <!--todo x: rust cmd 测试-->
    <Hello msg=""></Hello>

    <n-card title="点击测试:" style="background:darkorange; margin-bottom:5px">
        <n-list align="center" style="padding-bottom: 10px; padding-top: 5px">
            <n-list-item style="padding-top: 10px">
                <h1>My Data is: {{ someData }}</h1>
                <p v-if="waited != null">I waited for {{ waited }}</p>
            </n-list-item>
            <br>
            <n-grid cols="3">
                <n-grid-item>
                    <n-tag>
                        toggle: {{ log(toggle) }}
                    </n-tag>
                </n-grid-item>

                <n-grid-item>
                    <n-tag>
                        <h1> toggle: {{ log(counter) }}</h1>
                    </n-tag>
                </n-grid-item>

                <n-grid-item>
                    <n-tag>
                        Count is : {{ counter }}
                    </n-tag>
                </n-grid-item>
            </n-grid>


            <n-list-item>
                <n-button class="btn" type="success" @click="counter++">
                    Click: {{ counter }}
                </n-button>
            </n-list-item>

        </n-list>
    </n-card>




</template>


<script lang="ts">
import {defineComponent, getCurrentInstance, ref} from 'vue'

import {WebviewWindow} from '@tauri-apps/api/window';


export default defineComponent({
    name: 'Home',
    props: ['waited'],

    data: () => ({
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
