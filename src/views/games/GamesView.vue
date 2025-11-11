<template>
    <SidebarProvider>
        <GamesSidebar />
        <main>
            <SidebarTrigger></SidebarTrigger>
            <RouterView></RouterView>
        </main>
    </SidebarProvider>
</template>

<script setup lang="ts">
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import { RouterView } from "vue-router";
import GamesSidebar from "@/views/games/components/game-sidebar/GamesSidebar.vue";
</script>

<style scoped></style>
