<script setup>
import {NSpace, NLayout, NLayoutSider, NLayoutContent} from 'naive-ui'
</script>

<template>
    <n-space vertical size="large">
        <n-layout has-sider>
            <n-layout-sider
                    collapse-mode="transform"
                    :collapsed-width="120"
                    :width="240"
                    show-trigger="arrow-circle"
                    content-style="padding: 24px;"
                    bordered
            >
                <n-h2>海淀桥</n-h2>
            </n-layout-sider>


            <n-layout-content content-style="padding: 24px;">
                平山道
            </n-layout-content>


        </n-layout>
    </n-space>
</template>
