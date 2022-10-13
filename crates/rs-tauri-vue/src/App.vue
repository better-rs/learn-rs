<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import HelloWorld from './components/HelloWorld.vue'
import Button from './components/Button.vue'
import Layout from './components/Layout.vue'
import Menu from './components/Menu.vue'
import Menu2 from './components/Menu2.vue'
import Tab from './components/Tab.vue'
</script>

<template>
    <div class="flex flex-col p-2">

        <Tab></Tab>

        <Menu2></Menu2>

        <div class="flex flex-row items-center justify-center">
            <img src="/tauri.svg" class="w-20 h-20 logo tauri" alt="Tauri logo"/>
            <img src="/vue.svg" class="w-20 h-20 logo vue" alt="Vue logo"/>
        </div>

        <Button/>
        <HelloWorld msg="Hello Tauri + Vue!"/>


        <Layout></Layout>

    </div>

    <Menu></Menu>
    <Menu2></Menu2>


</template>
