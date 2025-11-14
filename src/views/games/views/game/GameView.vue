<template>
    <div class="flex flex-col gap-2">
        <template v-if="gameInfo">
            <GameHero :url="gameInfo.header_image"></GameHero>
            <GameTitle :title="gameInfo.name"></GameTitle>
            <span
                v-if="!isGameInstalled"
                class="border bg-red-700/10 p-2 border-red-700 text-red-700"
            >
                you need to install this game throught steam to play it</span
            >
            <Button
                v-if="isGameInstalled && gameInfo"
                @click="onPlay(gameInfo.steam_appid)"
                >play</Button
            >
            <GameDescription
                :description="gameInfo.about_the_game"
            ></GameDescription>
        </template>
    </div>
</template>

<script setup lang="ts">
import Button from "@/components/ui/button/Button.vue";
import GameHero from "@/views/games/views/game/components/GameHero.vue";
import GameTitle from "@/views/games/views/game/components/GameTitle.vue";
import GameDescription from "@/views/games/views/game/components/GameDescription.vue";
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
const isGameInstalled = ref(false);

watchEffect(async () => {
    if (game.value) {
        gameInfo.value = await getGameInfo(game.value.appid);
        if (gameInfo.value) {
            isGameInstalled.value = await getIsGameInstalled(
                gameInfo.value.steam_appid,
            );
        }
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

async function getIsGameInstalled(gameId: number) {
    try {
        const res: boolean = await invoke("is_game_installed", {
            gameId,
        });

        return res;
    } catch (e) {
        console.error(e);
        return false;
    }
}

async function onPlay(gameId: number) {
    try {
        const res: boolean = await invoke("launch_game", {
            gameId,
        });

        if (!res) throw new Error("unable to launch game");
    } catch (e) {
        console.error(e);
    }
}
</script>

<style scoped></style>
