<script setup>
import {NButton, NCard, NTabs, NTabPane, NForm, NFormItemRow,} from 'naive-ui'

import TabMenu from './TabMenu.vue'

</script>


<template>
    <n-card title="" style="margin-bottom: 16px">
        <n-tabs default-value="home" justify-content="space-evenly" type="line">
            <n-tab-pane name="home" tab="首页">
                首页内容
                <TabMenu></TabMenu>
            </n-tab-pane>


            <n-tab-pane name="trending" tab="行情">

                <TabMenu></TabMenu>

            </n-tab-pane>

            <n-tab-pane name="news" tab="资讯">
                资讯页面
            </n-tab-pane>


            <n-tab-pane name="settings" tab="设置">
                用户设置页面
            </n-tab-pane>

        </n-tabs>
    </n-card>


</template>
