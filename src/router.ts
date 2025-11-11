import { createRouter, createWebHistory } from "vue-router";
import GamesView from "@/views/games/GamesView.vue";

const routes = [
    {
        path: "/",
        component: GamesView,
    },
];

export const router = createRouter({
    history: createWebHistory(),
    routes,
});
