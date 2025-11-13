<template>
    <h1>{{ props.title }}</h1>
</template>

<script setup lang="ts">
import { defineProps } from "vue";

type Props = {
    title: string;
};

const props = defineProps<Props>();
</script>

<style scoped></style>
