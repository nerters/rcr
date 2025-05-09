import { createApp } from 'vue'

import router from './router'
import App from './App.vue'



import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import { VTreeview } from 'vuetify/labs/VTreeview'

import '@mdi/font/css/materialdesignicons.css'
import { aliases, mdi } from 'vuetify/iconsets/mdi'


const vuetify = createVuetify({
    components: {
        ...components,
        VTreeview // Add VTreeview to your components
    },
    directives,
    icons: {
        defaultSet: 'mdi',
        aliases,
        sets: {
        mdi,
        },
    },
})

const app = createApp(App)
app.use(router)
app.use(vuetify)
app.mount('#app')