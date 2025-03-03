<template>
    <n-tabs type="line" trigger="hover" justify-content="space-evenly">

        <n-tab-pane name="tab" tab="tab test">
            <DebugTab/>
        </n-tab-pane>

        <n-tab-pane name="btn" tab="button">
           <DebugButton/>
        </n-tab-pane>

        <n-tab-pane name="jay chou" tab="test3">
            test3
        </n-tab-pane>

    </n-tabs>
</template>
<script>
import DebugTab from "./DebugTab.vue";
import DebugButton from "./DebugButton.vue";
export default {
    components: {DebugButton, DebugTab}
}
</script>
