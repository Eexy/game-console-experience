import { createRouter, createWebHistory } from "vue-router";
import GameView from "./views/GameView.vue";

const routes = [
    {
        path: "/",
        component: GameView,
    },
];

export const router = createRouter({
    history: createWebHistory(),
    routes,
});
