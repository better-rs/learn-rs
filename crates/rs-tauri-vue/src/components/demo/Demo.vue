<template>
    <div>
        <div>Home</div>
        <p>My Data is: {{ someData }}</p>
        <p v-if="waited != null">I waited for {{ waited }}</p>
        toggle: {{ log(toggle) }}
        <button @click="counter++"> click{{ counter }}</button>

        <n-button bordered color="#ff69b4" type="success"> click</n-button>

    </div>

    <div class="flex flex-row items-center justify-center">
        <n-space>

            <n-button class="btn" type="success" @click="counter++">
                Click: {{ counter }}
            </n-button>


        </n-space>
    </div>


    <!--    <HelloWorld></HelloWorld>-->

</template>


<!--<script lang="ts" setup>-->
<!--import Hello from './Hello.vue'-->
<!--</script>-->

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

        function log(value) {
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
