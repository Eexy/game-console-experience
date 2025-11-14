import { SteamOwnedGame } from "@/types/steam/games";
import { defineStore } from "pinia";
import { computed, ref } from "vue";

export const useGameStore = defineStore("game", () => {
    const games = ref<SteamOwnedGame[]>([]);

    function getGameById(id: number) {
        const game = games.value.find((g) => g.appid === id);

        if (!game) return null;

        return game;
    }

    const gameSearch = ref("");

    const filteredGames = computed(() =>
        gameSearch.value.length
            ? games.value.filter((game) =>
                  game.name
                      .toLowerCase()
                      .includes(gameSearch.value.toLowerCase()),
              )
            : games.value,
    );

    return { games, filteredGames, gameSearch, getGameById };
});
