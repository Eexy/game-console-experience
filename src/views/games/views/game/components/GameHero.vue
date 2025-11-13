<template>
    <div>
        <img :src="props.url" />
    </div>
</template>

<script setup lang="ts">
import { defineProps } from "vue";

type Props = {
    url: string;
};

const props = defineProps<Props>();
</script>

<style scoped></style>
