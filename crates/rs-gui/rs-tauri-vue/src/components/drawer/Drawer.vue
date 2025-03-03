<template>

    <n-drawer
            v-model:show="active"
            :default-width="300"
            :placement="placement"
            resizable
    >
        <n-drawer-content title="我的">

            显示个人信息

        </n-drawer-content>
    </n-drawer>
</template>

<script lang="ts">
import {defineComponent, ref} from 'vue'
import type {DrawerPlacement} from 'naive-ui'

export default defineComponent({
    setup() {
        const activeRef = ref(true)
        const placementRef = ref<DrawerPlacement>('right')
        const activate = (place: DrawerPlacement) => {
            activeRef.value = true
            placementRef.value = place
        }
        return {
            active: activeRef,
            placement: placementRef,
            activate
        }
    }
})
</script>
