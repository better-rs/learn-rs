<template>
    <div class="flex flex-col items-center justify-center">

        <h1> 点击测试: </h1>
        <h1>My Data is: {{ someData }}</h1>

        <p v-if="waited != null">I waited for {{ waited }}</p>
        <h1> toggle: {{ log(toggle) }}</h1>
        <h1> toggle: {{ log(counter) }}</h1>
        <br>
        <h1> Count is : {{ counter }}</h1>

        <n-button class="btn" type="success" @click="counter++">
            Click: {{ counter }}
        </n-button>

    </div>

    <!--todo x: rust cmd 测试-->
    <Hello msg=""></Hello>

</template>


<script lang="ts">
import {defineComponent, getCurrentInstance, ref} from 'vue'

export default defineComponent({
    name: 'Home',
    props: ['waited'],

    data: () => ({
        toggle: false,
        counter: 0,
    }),


    setup() {
        const me = getCurrentInstance()

        function log(value: any) {
            console.log(value)
            return value
        }

        return {
            log,
            someData: ref(0),
        }
    },

    _beforeRouteEnter() {
        this.toggle = true
    },
})
</script>
