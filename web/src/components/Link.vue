<template>
    <router-link :to="{ name }" class="text-blue-600 decoration-2 hover:underline font-medium">
        <slot />
    </router-link>
</template>

<script setup lang="ts">
defineProps({
    name: {
        type: String,
        required: true
    },
});
</script>

<style scoped>

</style>