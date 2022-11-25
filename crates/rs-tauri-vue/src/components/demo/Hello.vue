<script lang="ts" setup>
import {invoke} from '@tauri-apps/api/tauri'

defineProps<{ msg: string }>()

const count = ref(0)
const addRet = ref(0)


async function backendAdd() {
    count.value = await invoke('backend_add', {number: count.value})
}

async function addTodo() {
    // todo x: sqlite crud
    addRet.value = await invoke('add_todo', {title: "test from fe", description: "test from fe"})
    console.log("addTodo", addRet)
}


</script>

<template>

    <n-card style="background:gainsboro; margin-bottom:5px" title="rust command 测试:">
        <n-space vertical>
            <n-list align="center">
                <n-list-item>
                    <n-tag>Count is: {{ count }}</n-tag>
                </n-list-item>
            </n-list>


            <n-grid align="center" cols="3">

                <n-grid-item>
                    <n-button style="background-color: cadetblue" type="success" @click="count++">Add 1</n-button>
                </n-grid-item>

                <n-grid-item>
                    <n-button style="background-color: orange" type="error" @click="backendAdd">
                        Add 2 (call rust in backend)
                    </n-button>
                </n-grid-item>


                <n-grid-item>
                    <n-button style="background-color: orange" type="error" @click="addTodo">
                        Add Todo (call rust in backend)
                    </n-button>
                </n-grid-item>


            </n-grid>


            <n-text>
                Edit
                <code>src/components/HelloWorld.vue</code> to test hot module replacement. Find the backend
                function from <code>src-tauri/src/main.rs</code>.
            </n-text>
        </n-space>

    </n-card>


</template>
