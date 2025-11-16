import { GameItem } from "@/types/games";
import { defineStore } from "pinia";
import { ref } from "vue";

export const useGameStore = defineStore("game", () => {
    const games = ref<GameItem[]>([]);

    const gameSearch = ref("");

    return { games, gameSearch };
});
