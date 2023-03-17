<script lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {defineComponent} from 'vue'
import {dateZhCN, NConfigProvider, zhCN} from 'naive-ui';

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
    <n-config-provider :date-locale="dateZhCN" :locale="zhCN">

        <div class="flex-col p-5">
            <!--            todo x: tab-->
            <Tab></Tab>
        </div>

    </n-config-provider>

</template>
