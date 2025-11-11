<template>
    <ScrollArea>
        <ul>
            <li v-for="game in props.games" :key="game.appid">
                {{ game.name }}
            </li>
        </ul>
    </ScrollArea>
</template>

<script setup lang="ts">
import { SteamOwnedGame } from "@/types/steam/games";

type Props = {
    games: SteamOwnedGame[];
    gameCount: number;
};

const props = defineProps<Props>();
</script>

<style scoped></style>
