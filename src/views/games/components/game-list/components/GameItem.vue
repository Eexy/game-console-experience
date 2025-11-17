<template>
    <RouterLink :to="`/games/${game.id}`">
        <span
            class="flex gap-2 items-center p-1 rounded"
            :class="isActive ? 'bg-black/15' : ''"
        >
            <img :src="iconUrl" class="size-8 rounded-md" />
            {{ game.title }}
        </span>
    </RouterLink>
</template>

<script setup lang="ts">
import { GameItem } from "@/types/games";
import { computed } from "vue";
import { useRoute } from "vue-router";

type Props = {
    game: GameItem;
};

const route = useRoute();

const isActive = computed(() => route.params.id === String(props.game.id));

const props = defineProps<Props>();

const iconUrl = computed(
    () =>
        `http://media.steampowered.com/steamcommunity/public/images/apps/${props.game.store_app_id}/${props.game.logo_url}.jpg`,
);
</script>

<style scoped></style>
