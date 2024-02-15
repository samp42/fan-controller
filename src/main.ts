import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import "./styles.css";
import App from "./App.vue";
import HomeVue from "./components/Home.vue";
import HelpVue from "./components/Help.vue";

const routes = [
    { path: '/', component: HomeVue },
    { path: '/help', component: HelpVue },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

const app = createApp(App);

app.use(router);

app.mount("#app");
