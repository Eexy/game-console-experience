<template>
    <ul>
        <li v-for="game in games" :key="game.appid">
            <RouterLink :to="`/games/${game.appid}`">
                {{ game.name }}
            </RouterLink>
        </li>
    </ul>
</template>

<script setup lang="ts">
import { useGameStore } from "@/stores/game.store";
import { RouterLink } from "vue-router";
import { storeToRefs } from "pinia";

const { games } = storeToRefs(useGameStore());
</script>

<style scoped></style>
