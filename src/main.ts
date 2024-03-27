import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import mitt from "mitt";
import "./styles.css";
import App from "./App.vue";
import HomeVue from "./components/Home.vue";
import HelpVue from "./components/Help.vue";
import PatternsVue from "./components/Patterns.vue";

/**
 * represent pattern at timestamp t (all 81 fan values)
 */
export interface PatternFrame {
    frame: number[][];
}

/**
 * class to represent either static pattern with its value
 * or dynamic pattern from CSV
 */
// export class Pattern {
//     staticPattern?: number[][];
//     dynamicPattern?:
// }

const routes = [
    { path: '/', component: HomeVue },
    { path: '/help', component: HelpVue },
    { path: '/patterns', component: PatternsVue },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

const emitter = mitt();
const app = createApp(App);

app.use(router);
app.config.globalProperties.emitter = emitter;

app.mount("#app");
