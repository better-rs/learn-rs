<template>
    <n-space vertical>
        <!--        todo x: 折叠菜单-->
        <!--        <n-switch v-model:value="collapsed"/>-->

        <n-layout has-sider>
            <n-layout-sider
                    bordered
                    collapse-mode="width"
                    :collapsed-width="64"
                    :width="240"
                    :collapsed="collapsed"
                    show-trigger
                    @collapse="collapsed = true"
                    @expand="collapsed = false"
            >
                <n-menu
                        :collapsed="collapsed"
                        :collapsed-width="64"
                        :collapsed-icon-size="22"
                        :options="menuOptions"
                        :render-label="renderMenuLabel"
                        :render-icon="renderMenuIcon"
                        :expand-icon="expandIcon"
                />
            </n-layout-sider>


            <n-layout>
                <span>内容</span>
            </n-layout>


        </n-layout>
    </n-space>
</template>

<script lang="ts">
import {h, ref, defineComponent} from 'vue'
import {NIcon} from 'naive-ui'
import type {MenuOption} from 'naive-ui'
import {BookmarkOutline, CaretDownOutline} from '@vicons/ionicons5'

const menuOptions: MenuOption[] = [
    {
        label: '基础设置',
        key: 'basic',
        // href: ''
    },
    {
        label: 'API-Key 管理',
        key: 'api-key',
        // href: '',
        // disabled: true,
        children: [
            {
                label: '币安',
                key: 'binance-api-key',
            },
            {
                label: 'FTX',
                key: 'ftx-api-key',
            },
        ]
    },
    {
        label: '行情监控',
        key: 'market-watch',
        // href: '',
        // disabled: true,
        children: [
            {
                label: '币价监控',
                key: 'price-watch',
            },
            {
                label: '链上监控',
                key: 'chain-watch',
            },
        ]
    },
]

export default defineComponent({
    setup() {
        return {
            collapsed: ref(false),
            menuOptions,
            renderMenuLabel(option: MenuOption) {
                if ('href' in option) {
                    return h(
                            'a',
                            {href: option.href, target: '_blank'},
                            option.label as string
                    )
                }
                return option.label as string
            },
            renderMenuIcon(option: MenuOption) {
                // 渲染图标占位符以保持缩进
                if (option.key === '') return true
                // 返回 falsy 值，不再渲染图标及占位符
                if (option.key === 'food') return null
                return h(NIcon, null, {default: () => h(BookmarkOutline)})
            },
            expandIcon() {
                return h(NIcon, null, {default: () => h(CaretDownOutline)})
            }
        }
    }
})
</script>
