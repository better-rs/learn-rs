<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import HelloWorld from './components/HelloWorld.vue'
import Layout from './components/Layout.vue'
import Tab from './components/Tab.vue'
</script>

<template>
    <div class="flex flex-col p-2">

        <div class="flex flex-row items-center justify-center">
            <img src="/tauri.svg" class="w-10 h-10 logo tauri" alt="Tauri logo"/>
            <img src="/vue.svg" class="w-10 h-10 logo vue" alt="Vue logo"/>
        </div>

        <Tab></Tab>
        <!--        <Button/>-->

        <!--        <HelloWorld msg="Hello Tauri + Vue!"/>-->


        <n-card title="Route View">
            <n-tabs type="line" animated>

                <n-tab-pane name="debug" tab="Debug">
                    <router-link to="/debug">Debug 页面</router-link>
                </n-tab-pane>

                <n-tab-pane name="home" tab="Home">
                    <router-link to="/">Go to Home</router-link>
                </n-tab-pane>

                <n-tab-pane name="about" tab="About">
                    <router-link to="/about">Go to About</router-link>
                </n-tab-pane>

            </n-tabs>
        </n-card>

    </div>


    <!-- 路由出口 -->
    <!-- 路由匹配到的组件将渲染在这里 -->
    <!--    <router-view></router-view>-->

</template>
