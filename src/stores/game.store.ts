import { SteamOwnedGame } from "@/types/steam/games";
import { defineStore } from "pinia";
import { ref } from "vue";

export const useGameStore = defineStore("game", () => {
    const games = ref<SteamOwnedGame[]>([]);
    const currentGame = ref<SteamOwnedGame | null>(null);

    return { games, currentGame };
});
