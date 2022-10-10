import devtools from '@vue/devtools'
import { createApp } from 'vue'
import App from './App.vue'

import './assets/main.postcss'

if (process.env.NODE_ENV === 'development') {
  devtools.connect('http://localhost', 5001) // TODO X: 注意和vue devtool 调试工具的端口保持一致
}

createApp(App).mount('#app')
