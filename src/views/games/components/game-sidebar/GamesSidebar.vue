<template>
    <Sidebar>
        <SidebarHeader>
            <SteamIdForm v-if="!profileId"></SteamIdForm>
            <GameSearch></GameSearch>
        </SidebarHeader>
        <SidebarContent class="overflow-hidden">
            <ScrollArea class="min-h-0">
                <GamesList></GamesList>
                <ScrollBar />
            </ScrollArea>
        </SidebarContent>
    </Sidebar>
</template>

<script setup lang="ts">
import SteamIdForm from "@/components/ui/steam-id-form/SteamIdForm.vue";
import {
    SidebarHeader,
    Sidebar,
    SidebarContent,
} from "@/components/ui/sidebar";
import { ScrollArea, ScrollBar } from "@/components/ui/scroll-area";
import GamesList from "@/views/games/components/game-list/GamesList.vue";
import GameSearch from "@/views/games/components/game-sidebar/GameSearch.vue";
import { computed } from "vue";

const profileId = computed(() => import.meta.env.VITE_STEAM_PROFILE_ID);
</script>

<style scoped></style>
