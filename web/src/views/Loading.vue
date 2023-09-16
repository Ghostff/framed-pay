<template>
  <div
    v-show="!hideSpinner"
    :class="{
      'w-4 h-4': size === undefined || size === 'xs',
      'w-8 h-8': size === 'md',
      'w-12 h-12': size === 'lg',
      'w-20 h-20': size === 'xl'
    }"
    class="animate-spin inline-block border-[3px] border-current border-t-transparent text-indigo-600 rounded-full"
    role="status"
    aria-label="loading"
  >
    <span class="sr-only">Loading...</span>
  </div>
  <span class="block text-xs">{{ text || 'Loading...' }}</span>
</template>

<script setup lang="ts">
defineProps<{
  text?: string
  size?: string
  hideSpinner?: boolean
}>()
</script>
