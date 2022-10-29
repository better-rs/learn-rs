<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
defineProps<{ msg: string }>()

const count = ref(0)
async function backendAdd() {
  count.value = await invoke('backend_add', { number: count.value })
}
</script>

<template>
  <div class="w-9/12 mx-auto">
    <div class="justify-center my-6 text-center border border-white/10">

        <h1>rust command 测试</h1>
        <div class="p-8 text-lg font-bold"> Count is: {{ count }} </div>

      <div class=" space-x-2">
        <button type="button" class="btn" @click="count++">Add 1</button>
        <button type="button" class="btn" @click="backendAdd">Add 2 in backend</button>
      </div>
    </div>


    <p>
      Edit
      <code>src/components/HelloWorld.vue</code> to test hot module replacement. Find the backend
      function from <code>src-tauri/src/main.rs</code>.
    </p>


  </div>
</template>
