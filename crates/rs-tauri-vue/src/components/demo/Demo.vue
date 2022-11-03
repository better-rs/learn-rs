<template>
    <div class="flex flex-col items-center justify-center">

        <n-button class="btn" type="success" @click="open_webview('https://bing.com')">
            webview 打开 bing
        </n-button>

        <n-button class="btn" type="success" @click="open_webview('https://google.com')">
            webview 打开 google
        </n-button>

        <n-button class="btn" type="success" @click="open_webview('https://github.com')">
            webview 打开 github
        </n-button>


        <h1> 点击测试: </h1>
        <h1>My Data is: {{ someData }}</h1>

        <p v-if="waited != null">I waited for {{ waited }}</p>
        <h1> toggle: {{ log(toggle) }}</h1>
        <h1> toggle: {{ log(counter) }}</h1>
        <br>
        <h1> Count is : {{ counter }}</h1>

        <n-button class="btn" type="success" @click="counter++">
            Click: {{ counter }}
        </n-button>

    </div>

    <!--todo x: rust cmd 测试-->
    <Hello msg=""></Hello>

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
