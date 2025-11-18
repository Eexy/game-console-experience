<template>
    <div class="flex flex-col gap-2 p-2">
        <template v-if="game">
            <h1 class="font-bold text-xl">{{ game.title }}</h1>
        </template>
    </div>
</template>

<script setup lang="ts">
import { GameItem } from "@/types/games";
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();

const game = ref<GameItem | null>(null);

onMounted(async () => {
    try {
        const res: GameItem = await invoke("get_game_by_id", {
            id: Number(route.params.id),
        });

        game.value = res;
    } catch (error) {
        console.error(error);
    }
});
</script>

<style scoped></style>
