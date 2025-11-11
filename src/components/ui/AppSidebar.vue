<template>
    <Sidebar>
        <SidebarHeader>
            <SteamIdForm
                v-model="profileId"
                @submit="
                    (data) => (
                        (ownedGames = data.games),
                        (gameCount = data.game_count)
                    )
                "
            ></SteamIdForm>
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
import SteamIdForm from "./steam-id-form/SteamIdForm.vue";
import { SteamOwnedGame } from "@/types/steam/games";
import {
    SidebarHeader,
    Sidebar,
    SidebarContent,
} from "@/components/ui/sidebar";
import { ScrollArea, ScrollBar } from "@/components/ui/scroll-area";
import GameList from "@/components/ui/GameList.vue";
import { ref } from "vue";

const profileId = ref("");

const ownedGames = ref<SteamOwnedGame[]>([]);
const gameCount = ref(0);
</script>

<style scoped></style>
