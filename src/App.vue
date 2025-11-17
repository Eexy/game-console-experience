<template>
    <RouterView></RouterView>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { RouterView } from "vue-router";
import { storeToRefs } from "pinia";
import { useGameStore } from "./stores/game.store";
import { GameItem } from "./types/games";

const { games } = storeToRefs(useGameStore());

window.addEventListener("load", async () => {
    const isRefresh = sessionStorage.getItem("appLoaded");

    if (!isRefresh || !Boolean(isRefresh)) {
        try {
            const res: GameItem[] = await invoke("get_games");
            games.value = res;
        } catch (e) {
            console.log(e);
        }
    } else {
        try {
            await invoke("refresh_games");
            const res: GameItem[] = await invoke("get_games");
            games.value = res;
        } catch (e) {
            console.error(e);
        }
    }

    sessionStorage.setItem("appLoaded", "true");
});
</script>

<style scoped></style>
