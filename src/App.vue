<template>
    <RouterView></RouterView>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted } from "vue";
import { RouterView } from "vue-router";
import { SteamOwnedGameResponse } from "./types/steam/api";
import { storeToRefs } from "pinia";
import { useGameStore } from "./stores/game.store";

const { games } = storeToRefs(useGameStore());

onMounted(async () => {
    const key = import.meta.env.VITE_STEAM_KEY;
    const profileId = import.meta.env.VITE_STEAM_PROFILE_ID;

    if (!key) throw new Error("missing steam api key");
    if (!profileId) throw new Error("missing steam profileId");

    try {
        const res: SteamOwnedGameResponse = await invoke(
            "get_steam_owned_games",
            {
                profileId: profileId,
                steamKey: key,
            },
        );

        games.value = res.games;
    } catch (e) {
        console.error(e);
    }
});
</script>

<style scoped></style>
