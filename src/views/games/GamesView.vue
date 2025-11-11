<template>
    <SidebarProvider>
        <GamesSidebar />
        <main>
            <SidebarTrigger></SidebarTrigger>
            {{ currentGame }}
        </main>
    </SidebarProvider>
</template>

<script setup lang="ts">
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import GamesSidebar from "@/views/games/components/game-sidebar/GamesSidebar.vue";
import { useGameStore } from "@/stores/game.store";
import { storeToRefs } from "pinia";

const { currentGame } = storeToRefs(useGameStore());
</script>

<style scoped></style>
