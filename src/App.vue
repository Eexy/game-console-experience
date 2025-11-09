<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

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

type OwnedGames = { name: string; appid: number };

type SteamOwnedGameResponse = {
    game_count: number;
    games: OwnedGames[];
};

const ownedGames = ref<OwnedGames[]>([]);
const gameCount = ref(0);
</script>

<template>
    <div>
        <div class="flex gap-2 p-2">
            <Input v-model="profileId"></Input>
            <Button @click="onSubmit">submit</Button>
        </div>
        <div>
            <p>count : {{ gameCount }}</p>
            <ul>
                <li v-for="game in ownedGames" :key="game.appid">
                    {{ game.name }}
                </li>
            </ul>
        </div>
    </div>
</template>

<style scoped></style>
