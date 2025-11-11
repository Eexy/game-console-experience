<template>
    <Sidebar>
        <SidebarHeader>
            <div class="flex gap-2 items-center">
                <Input v-model="profileId"></Input>
                <Button @click="onSubmit">submit</Button>
            </div>
        </SidebarHeader>
        <SidebarContent class="overflow-hidden">
            <ScrollArea class="min-h-0">
                <GameList
                    :game-count="gameCount"
                    :games="ownedGames"
                ></GameList>
                <ScrollBar />
            </ScrollArea>
        </SidebarContent>
    </Sidebar>
</template>

<script setup lang="ts">
import { SteamOwnedGameResponse } from "@/types/steam/api";
import { SteamOwnedGame } from "@/types/steam/games";
import {
    SidebarHeader,
    Sidebar,
    SidebarContent,
} from "@/components/ui/sidebar";
import { ScrollArea, ScrollBar } from "@/components/ui/scroll-area";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api/core";
import GameList from "@/components/ui/GameList.vue";
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
