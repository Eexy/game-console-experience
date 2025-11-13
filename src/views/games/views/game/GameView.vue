<template>
    <div class="flex flex-col gap-2">
        <template v-if="gameInfo">
            <GameHero :url="gameInfo.header_image"></GameHero>
            <GameTitle :title="gameInfo.name"></GameTitle>
            <GameDescription
                :description="gameInfo.about_the_game"
            ></GameDescription>
        </template>
    </div>
</template>

<script setup lang="ts">
import GameHero from "./components/GameHero.vue";
import GameTitle from "./components/GameTitle.vue";
import GameDescription from "./components/GameDescription.vue";
import { useGameStore } from "@/stores/game.store";
import { computed, ref, watchEffect } from "vue";
import { useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { SteamGameInfoResponse } from "@/types/steam/api";
import { SteamGameInfo } from "@/types/steam/games";

const route = useRoute();

const { getGameById } = useGameStore();

const game = computed(() => {
    const id = route.params.id;
    return getGameById(Number(id));
});

const gameInfo = ref<SteamGameInfo | null>(null);

watchEffect(async () => {
    if (game.value) {
        gameInfo.value = await getGameInfo(game.value.appid);
    }
});

async function getGameInfo(id: number) {
    try {
        const res: SteamGameInfoResponse = await invoke("get_game_info", {
            gameId: id,
        });

        if (!res[id].success) throw new Error("unable to get game info");

        return res[id].data;
    } catch (e) {
        console.error(e);
        return null;
    }
}
</script>

<style scoped></style>
