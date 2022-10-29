<script setup>
import {NCard, NTabs, NTabPane} from 'naive-ui'
import Home from './home/Home.vue'
import Settings from './settings/Settings.vue'
import Drawer from "./drawer/Drawer.vue";
import Debug from "./debug/Debug.vue";
import Demo from "./demo/Demo.vue"

</script>

<template>

    <n-card title="" style="margin-bottom: 16px">
        <n-tabs trigger="hover" default-value="demo" justify-content="space-evenly" type="line">


            <n-tab-pane name="demo" tab="Demo">
                <Demo></Demo>
            </n-tab-pane>


            <n-tab-pane name="home" tab="首页">
                <Home></Home>
            </n-tab-pane>


            <n-tab-pane name="trending" tab="行情">
                行情页面
            </n-tab-pane>

            <n-tab-pane name="news" tab="资讯">
                资讯页面
            </n-tab-pane>

            <n-tab-pane name="settings" tab="设置">
                <Settings></Settings>
            </n-tab-pane>

            <n-tab-pane name="debug" tab="Debug">
                <Debug></Debug>
            </n-tab-pane>


            <n-tab-pane name="drawer" tab="我的">
                <Drawer></Drawer>
            </n-tab-pane>


        </n-tabs>
    </n-card>
</template>
