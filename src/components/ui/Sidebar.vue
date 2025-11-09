<template>
    <div class="w-sm border-r flex flex-col gap-2 p-2">
        <div class="flex gap-2">
            <Input v-model="profileId"></Input>
            <Button @click="onSubmit">submit</Button>
        </div>
        <GameList :games="ownedGames" :game-count="gameCount"></GameList>
    </div>
</template>

<script setup lang="ts">
import { SteamOwnedGameResponse } from "@/types/steam/api";
import { SteamOwnedGame } from "@/types/steam/games";
import GameList from "./GameList.vue";
import { invoke } from "@tauri-apps/api/core";
import Button from "./button/Button.vue";
import Input from "./input/Input.vue";
import { ref } from "vue";

const profileId = ref("");

async function onSubmit() {
    const key = import.meta.env.VITE_STEAM_KEY;
    if (!key) throw new Error("missing steam api key");

    const res: SteamOwnedGameResponse = await invoke("get_steam_owned_games", {
        profileId: profileId.value,
        steamKey: key,
    });

    ownedGames.value = res.games;
    gameCount.value = res.game_count;
}

const ownedGames = ref<SteamOwnedGame[]>([]);
const gameCount = ref(0);
</script>

<style scoped></style>
