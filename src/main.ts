import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import "./styles.css";
import App from "./App.vue";
import HomeVue from "./components/Home.vue";
import HelpVue from "./components/Help.vue";
import PatternsVue from "./components/Patterns.vue"
import { createPinia } from 'pinia';
import { useGridStore } from './store';

const routes = [
    { path: '/', component: HomeVue },
    { path: '/help', component: HelpVue },
    { path: '/patterns', component: PatternsVue },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});
const pinia = createPinia();

const app = createApp(App);

app.use(router);
app.use(pinia); 
app.mount("#app");

app.config.globalProperties.$gridStore = useGridStore();

