<template>
    <form @submit.prevent="onSubmit" class="flex gap-2 items-center">
        <Input v-model="profileId"></Input>
        <Button @click="onSubmit" type="submit">submit</Button>
    </form>
</template>

<script setup lang="ts">
import { SteamOwnedGameResponse } from "@/types/steam/api";
import { invoke } from "@tauri-apps/api/core";
import { defineModel } from "vue";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";

const profileId = defineModel<string>({ required: true });

const emit = defineEmits<{
    (e: "submit", response: SteamOwnedGameResponse): void;
}>();

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

        emit("submit", res);
    } catch (e) {
        console.error(e);
    }
}
</script>

<style scoped></style>
