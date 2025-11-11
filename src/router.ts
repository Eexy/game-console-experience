import { createRouter, createWebHistory } from "vue-router";
import GamesView from "@/views/games/GamesView.vue";
import Game from "@/views/games/views/game/GameView.vue";

const routes = [
    {
        path: "/",
        redirect: "/games",
    },
    {
        path: "/games",
        component: GamesView,
        children: [
            {
                path: ":id",
                component: Game,
            },
        ],
    },
];

export const router = createRouter({
    history: createWebHistory(),
    routes,
});
