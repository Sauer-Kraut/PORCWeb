import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
  ],
  resolve: {
    alias: {
      '@/models/pub_account_info': fileURLToPath(new URL('./src/logic/models/account', import.meta.url)),
      '@/models/availability': fileURLToPath(new URL('./src/logic/models/account/schedule/availability', import.meta.url)),
      '@/models/match_event': fileURLToPath(new URL('./src/logic/models/account/schedule/match_event', import.meta.url)),
      '@/models/matchplan': fileURLToPath(new URL('./src/logic/models/matchplan', import.meta.url)),
      '@/models': fileURLToPath(new URL('./src/logic/models', import.meta.url)),
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
})
