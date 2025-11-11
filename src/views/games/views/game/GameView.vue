<template>
    <div>
        {{ game ? game.name : "hello" }}
    </div>
</template>

<script setup lang="ts">
import { useGameStore } from "@/stores/game.store";
import { computed } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();

const { getGameById } = useGameStore();

const game = computed(() => {
    const id = route.params.id;
    return getGameById(Number(id));
});
</script>

<style scoped></style>
