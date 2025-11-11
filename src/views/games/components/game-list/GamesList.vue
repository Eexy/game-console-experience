<template>
    <ul>
        <li @click="currentGame = game" v-for="game in games" :key="game.appid">
            {{ game.name }}
        </li>
    </ul>
</template>

<script setup lang="ts">
import { useGameStore } from "@/stores/game.store";
import { storeToRefs } from "pinia";

const { games, currentGame } = storeToRefs(useGameStore());
</script>

<style scoped></style>
