<template>
    <form @submit.prevent="onSubmit" class="flex gap-2 items-center">
        <Input v-model="profileId"></Input>
        <Button @click="onSubmit" type="submit">submit</Button>
    </form>
</template>

<script setup lang="ts">
import { SteamOwnedGameResponse } from "@/types/steam/api";
import { invoke } from "@tauri-apps/api/core";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { storeToRefs } from "pinia";
import { useGameStore } from "@/stores/game.store";
import { ref } from "vue";

const profileId = ref("");
const { games } = storeToRefs(useGameStore());

async function onSubmit() {
    const key = import.meta.env.VITE_STEAM_KEY;
    if (!key) throw new Error("missing steam api key");

    try {
        const res: SteamOwnedGameResponse = await invoke(
            "get_steam_owned_games",
            {
                profileId: profileId.value,
                steamKey: key,
            },
        );

        games.value = res.games;
    } catch (e) {
        console.error(e);
    }
}
</script>

<style scoped></style>
