<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { ref } from "vue";
import { fetch } from "@tauri-apps/plugin-http";

const profileId = ref("");

async function onSubmit() {
    const key = import.meta.env.VITE_STEAM_KEY;
    if (!key) throw new Error("missing steam api key");

    const res = await fetch(
        `http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key=${key}&steamid=${profileId.value}`,
        { method: "GET" },
    );
    const data = await res.json();
    console.log(data.response);
}
</script>

<template>
    <div>
        <div class="flex gap-2 p-2">
            <Input v-model="profileId"></Input>
            <Button @click="onSubmit">submit</Button>
        </div>
        <div></div>
    </div>
</template>

<style scoped></style>
