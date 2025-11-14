<template>
    <div>
        <Input placeholder="Search for game" v-model="gameSearch"></Input>
    </div>
</template>

<script setup lang="ts">
import { Input } from "@/components/ui/input";
import { useGameStore } from "@/stores/game.store";
import { storeToRefs } from "pinia";

const { gameSearch } = storeToRefs(useGameStore());
</script>

<style scoped></style>
