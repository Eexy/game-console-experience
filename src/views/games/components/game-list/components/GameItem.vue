<template>
    <RouterLink :to="`/games/${game.appid}`">
        <span
            class="flex gap-2 items-center p-1 rounded"
            :class="isActive ? 'bg-black/15' : ''"
        >
            <img :src="iconUrl" class="size-8 rounded-md" />
            {{ game.name }}
        </span>
    </RouterLink>
</template>

<script setup lang="ts">
import { SteamOwnedGame } from "@/types/steam/games";
import { computed } from "vue";
import { useRoute } from "vue-router";

type Props = {
    game: SteamOwnedGame;
};

const route = useRoute();

const isActive = computed(() => route.params.id === String(props.game.appid));

const props = defineProps<Props>();

const iconUrl = computed(
    () =>
        ` http://media.steampowered.com/steamcommunity/public/images/apps/${props.game.appid}/${props.game.img_icon_url}.jpg`,
);
</script>

<style scoped></style>
