<script lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {defineComponent} from 'vue'
import {dateZhCN, zhCN} from 'naive-ui';
import {NConfigProvider} from 'naive-ui'

export default defineComponent({
    components: {
        NConfigProvider
    },
    setup() {
        return {
            zhCN,
            dateZhCN
        }
    }
})
</script>

<script lang="ts" setup>
import Tab from './components/Tab.vue'
</script>


<template>
    <n-config-provider :locale="zhCN" :date-locale="dateZhCN">

        <div class="flex-col p-5">

            <div class="flex flex-row items-center justify-center">
                <img src="/tauri.svg" class="w-5 h-5 logo tauri" alt="Tauri logo"/>
                <img src="/vue.svg" class="w-5 h-5 logo vue" alt="Vue logo"/>
            </div>


            <!--            todo x: tab-->
            <Tab></Tab>

        </div>

    </n-config-provider>

</template>
