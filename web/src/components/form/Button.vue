<template>
  <button
      :class="{
      'py-1 px-2': size === 'sm',
      'py-2 px-3': size === 'md',
      'py-3 px-4': size === 'lg',
      'py-4 px-8': size === 'xl',
      'rounded-full': rounded,
      'rounded-md': !rounded,
      'text-white border-transparent bg-gray-800 hover:bg-gray-900 focus:ring-gray-800 dark:focus:ring-gray-900 dark:focus:ring-offset-gray-800': is === 'dark',
      'text-white border-transparent bg-gray-500 hover:bg-gray-600 focus:ring-gray-500 dark:bg-gray-700 dark:hover:bg-gray-600 dark:focus:ring-offset-gray-800':  is === 'secondary',
      'text-white border-transparent bg-red-500 hover:bg-red-600 focus:ring-red-500 dark:focus:ring-offset-gray-800': is === 'danger',
      'text-white border-transparent bg-yellow-500 hover:bg-yellow-600 focus:ring-yellow-500 dark:focus:ring-offset-gray-800': is === 'warning',
      'text-white border-transparent bg-green-500 hover:bg-green-600 focus:ring-green-500 dark:focus:ring-offset-gray-800':is === 'success',
      'text-white border-transparent dark:bg-indigo-600 bg-indigo-500 hover:bg-indigo-600 focus:outline-none focus:ring-indigo-500 ': is === 'primary',
      'text-white border-transparent bg-blue-500 hover:bg-blue-600 focus:ring-blue-500 dark:focus:ring-offset-gray-800': is === 'blue',
      'border-transparent text-gray-700 align-middle focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-blue-600 dark:bg-gray-800 dark:text-gray-400 dark:hover:text-white': is === 'transparent',
      'text-gray-700 bg-white text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-offset-white focus:ring-blue-600 dark:bg-gray-800 dark:hover:bg-slate-800 dark:border-gray-700 dark:text-gray-400 dark:hover:text-white dark:focus:ring-offset-gray-800': is === 'white'
    }"
      :disabled="isLoading"
      :type="type"
      data-cooltipz-dir="top"
      :aria-label="tooltip"
      class="inline-flex justify-center items-center gap-2 border font-semibold focus:outline-none focus:ring-2 transition-all text-sm"
  >
    <span
        v-show="isLoadable && isLoading"
        aria-label="loading"
        class="animate-spin inline-block w-4 h-4 border-[3px] border-white border-t-transparent rounded-full"
        role="status"
    />
    <slot />
  </button>
</template>

<script lang="ts" setup>
import type { ButtonHTMLAttributes, PropType } from 'vue'

defineProps({
  size: {
    type: String,
    default: 'lg'
  },
  tooltip: {
    type: String
  },
  isLoadable: {
    type: Boolean,
    default: true
  },
  isLoading: {
    type: Boolean,
    default: false
  },
  rounded: {
    type: Boolean,
    default: false
  },
  type: {
    type: String as PropType<ButtonHTMLAttributes['type']>,
    default: 'button'
  },
  is: {
    type: String as PropType<string>,
    default: 'primary',
    validator(value: string) {
      return ['dark', 'secondary', 'danger', 'warning', 'success', 'primary', 'white', 'transparent'].includes(value)
    }
  }
})
</script>

<style lang="scss" scoped>
button:disabled {
  @apply opacity-60;
}
</style>
