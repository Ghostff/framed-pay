<template>
  <label v-if="label" :for="id" class="block text-sm mb-2 dark:text-white">
    {{ label }}
  </label>
</template>
<script setup lang="ts">
defineProps<{
  label?: string
  required?: boolean
  id?: string
}>()
</script>
<style scoped></style>
