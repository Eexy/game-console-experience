<template>
    <SidebarProvider>
        <AppSidebar></AppSidebar>
        <main>
            <SidebarTrigger></SidebarTrigger>
        </main>
    </SidebarProvider>
</template>

<script setup lang="ts">
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import AppSidebar from "@/components/ui/AppSidebar.vue";
</script>

<style scoped></style>
