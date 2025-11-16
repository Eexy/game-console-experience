<template>
    <RouterView></RouterView>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted } from "vue";
import { RouterView } from "vue-router";
import { storeToRefs } from "pinia";
import { useGameStore } from "./stores/game.store";
import { GameItem } from "./types/games";

const { games } = storeToRefs(useGameStore());

onMounted(async () => {
    const key = import.meta.env.VITE_STEAM_KEY;
    const profileId = import.meta.env.VITE_STEAM_PROFILE_ID;

    if (!key) throw new Error("missing steam api key");
    if (!profileId) throw new Error("missing steam profileId");

    try {
        const res: GameItem[] = await invoke("get_games", {
            profileId: profileId,
            steamKey: key,
        });
        console.log(res);
        games.value = res;
    } catch (e) {
        console.log(e);
    }
});
</script>

<style scoped></style>
