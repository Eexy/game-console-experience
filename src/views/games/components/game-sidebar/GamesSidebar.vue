<template>
    <Sidebar>
        <SidebarHeader>
            <SteamIdForm></SteamIdForm>
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
</script>

<style scoped></style>
