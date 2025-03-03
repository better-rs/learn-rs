import devtools from '@vue/devtools'
import {createApp} from 'vue'
import type {ComponentPublicInstance} from 'vue'
import {router, routerHistory} from './router'
import {globalState} from './store'
import App from './App.vue'

import './assets/main.postcss'

if (process.env.NODE_ENV === 'development') {

  // todo x: 暂时关闭 devtool
  // devtools.connect('http://localhost', 5001) // TODO X: 注意和vue devtool 调试工具的端口保持一致

}


declare global {
  interface Window {
    // h: HTML5History
    h: typeof routerHistory
    r: typeof router
    // @ts-ignore
    vm: ComponentPublicInstance
  }
}

// for testing purposes
window.h = routerHistory
window.r = router

const app = createApp(App)
app.mixin({
  beforeRouteEnter() {
    console.log('mixin enter')
  },
})

app.provide('state', globalState)

// todo x: 路由表
app.use(router)

window.vm = app.mount('#app')
