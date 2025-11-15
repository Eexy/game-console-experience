import { SteamOwnedGame } from "@/types/steam/games";
import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import { ref, watch, watchEffect } from "vue";

export const useGameStore = defineStore("game", () => {
    const games = ref<SteamOwnedGame[]>([]);

    function getGameById(id: number) {
        const game = games.value.find((g) => g.appid === id);

        if (!game) return null;

        return game;
    }

    const gameSearch = ref("");

    const filteredGames = ref(games.value);

    watchEffect(async () => {
        if (!gameSearch.value.length) {
            filteredGames.value = games.value;
            return;
        }

        try {
            const res: SteamOwnedGame[] = await invoke("filter_games", {
                games: games.value,
                search: gameSearch.value,
            });
            filteredGames.value = res;
        } catch (e) {
            console.error(e);
        }
    });

    return { games, filteredGames, gameSearch, getGameById };
});
