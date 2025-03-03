<template>

    <n-card style="margin-bottom: 16px" title="">

        <!--         trigger="hover" -->
        <n-tabs default-value="demo" justify-content="space-evenly" type="line">
            <n-tab-pane v-for="t in tabs" :name="t.name" :tab="t.tab">
                <!--  TODO X: 动态组件-->
                <!--     https://cn.vuejs.org/api/sfc-script-setup.html#using-components -->
                <component :is="t.widget"/>
            </n-tab-pane>
        </n-tabs>

    </n-card>
</template>

<script lang="ts" setup>
import {NCard, NTabPane, NTabs} from 'naive-ui'
import Home from './home/Home.vue'
import Demo from "./demo/Demo.vue"
import Debug from "./debug/Debug.vue"
import Settings from "./settings/Settings.vue"
import Drawer from "./drawer/Drawer.vue"


const tabs = ref([
    {"tab": "Demo", "name": "demo", "widget": Demo,},
    {"tab": "Home", "name": "home", "widget": Home},
    {"tab": "Debug", "name": "debug", "widget": Debug},
    // {"tab": "行情", "name": "trending", "widget": Settings},
    {"tab": "资讯", "name": "news", "widget": Settings},
    {"tab": "设置", "name": "settings", "widget": Settings},
    {"tab": "我的", "name": "drawer", "widget": Drawer},
]);

</script>
