<template>
    <ul class="px-2">
        <li v-for="game in filteredGames" :key="game.id" class="">
            <GameItem :game="game"></GameItem>
        </li>
    </ul>
</template>

<script setup lang="ts">
import { useGameStore } from "@/stores/game.store";
import GameItem from "@/views/games/components/game-list/components/GameItem.vue";
import { storeToRefs } from "pinia";

const { filteredGames } = storeToRefs(useGameStore());
</script>

<style scoped></style>
