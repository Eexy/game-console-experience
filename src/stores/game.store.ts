import { SteamOwnedGame } from "@/types/steam/games";
import { defineStore } from "pinia";
import { ref } from "vue";

export const useGameStore = defineStore("game", () => {
    const games = ref<SteamOwnedGame[]>([]);

    function getGameById(id: number) {
        const game = games.value.find((g) => g.appid === id);

        if (!game) return null;

        return game;
    }

    return { games, getGameById };
});
