<script setup lang="ts">
import {invoke} from '@tauri-apps/api/tauri'

defineProps<{ msg: string }>()

const count = ref(0)

async function backendAdd() {
    count.value = await invoke('backend_add', {number: count.value})
}
</script>

<template>

    <n-card title="rust command 测试:" style="background:mediumpurple; margin-bottom:5px">
        <n-list align="center">
            <n-list-item >
                <n-tag>Count is: {{ count }}</n-tag>
            </n-list-item>
        </n-list>

        <n-grid cols="2" align="center">
            <n-grid-item>
                <n-button type="success" class="btn" @click="count++">Add 1</n-button>
            </n-grid-item>
            <n-grid-item>
                <n-button type="success" class="btn" @click="backendAdd">Add 2 (call rust in backend)</n-button>
            </n-grid-item>
        </n-grid>


        <n-text>
            Edit
            <code>src/components/HelloWorld.vue</code> to test hot module replacement. Find the backend
            function from <code>src-tauri/src/main.rs</code>.
        </n-text>


    </n-card>



</template>
