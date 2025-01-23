import { createApp } from 'vue';
import { createPinia } from 'pinia';
import App from './App.vue';
import PrimeVue from 'primevue/config';
import Aura from '@primevue/themes/aura';
// 配置PrimeVue
const app = createApp(App);
app.use(PrimeVue, {
    theme: {
        preset: Aura,
    },
});
// 配置Pinia
const pinia = createPinia();
app.use(pinia);
// 挂载到DOM
app.mount('#app');
