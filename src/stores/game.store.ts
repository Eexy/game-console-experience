import { GameItem } from "@/types/games";
import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import { ref, watchEffect } from "vue";

export const useGameStore = defineStore("game", () => {
    const games = ref<GameItem[]>([]);

    const gameSearch = ref("");

    const filteredGames = ref(games.value);

    watchEffect(async () => {
        if (!gameSearch.value) {
            filteredGames.value = games.value;
        }

        try {
            const res: GameItem[] = await invoke("filter_games_by_title", {
                title: gameSearch.value,
            });

            filteredGames.value = res;
        } catch (error) {
            console.error(error);
        }
    });

    return { games, gameSearch, filteredGames };
});
