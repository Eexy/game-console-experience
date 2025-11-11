<template>
    <Sidebar>
        <SidebarHeader>
            <SteamIdForm></SteamIdForm>
        </SidebarHeader>
        <SidebarContent class="overflow-hidden">
            <ScrollArea class="min-h-0">
                <GameList></GameList>
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
import GameList from "@/components/ui/GameList.vue";
</script>

<style scoped></style>
