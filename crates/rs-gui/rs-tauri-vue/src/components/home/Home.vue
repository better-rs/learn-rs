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
        label: 'Dashboard',
        key: 'dashboard',
        // href: ''
    },
    {
        label: '持仓分析',
        key: 'analyze',
        // href: '',
        // disabled: true,
        children: [
            {
                label: '持仓成本',
                key: 'avg-cost'
            },
            {
                label: '持仓收益',
                key: 'avg-profit'
            },
        ]
    }, {
        label: '充值/提现记录',
        key: 'tx',
        // href: '',
        // disabled: true,
        children: [
            {
                label: '充值记录',
                key: 'deposit'
            },
            {
                label: '提现记录',
                key: 'withdraw'
            },
        ]
    },


    {
        label: '现货交易',
        key: 'spot',
        // href: '',
        // disabled: true,
        children: [
            {
                label: '买入记录',
                key: 'spot-buy'
            },
            {
                label: '卖出记录',
                key: 'spot-sell'
            },
        ]
    },
    {
        label: '合约交易',
        key: 'futures',
        // href: '',
        // disabled: true,
        children: [
            {
                label: '买入记录',
                key: 'futures-buy'
            },
            {
                label: '卖出记录',
                key: 'futures-sell'
            },
        ]
    },


    {
        label: '数据导入/导出',
        key: 'data-io',
        children: [
            {
                type: 'group',
                label: '导入',
                key: 'import',
                children: [
                    {
                        label: '币安',
                        key: 'import-binance'
                    },
                    {
                        label: 'FTX',
                        key: 'import-ftx'
                    }
                ]
            },
            {
                type: 'group',
                label: '导出',
                key: 'export',
                children: [
                    {
                        label: '币安',
                        key: 'export-binance'
                    },
                    {
                        label: 'FTX',
                        key: 'export-ftx'
                    }
                ]
            },
        ]
    }
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
