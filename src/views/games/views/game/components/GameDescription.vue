<template>
    <p>{{ props.description }}</p>
</template>

<script setup lang="ts">
import { defineProps } from "vue";

type Props = {
    description: string;
};

const props = defineProps<Props>();
</script>

<style scoped></style>
