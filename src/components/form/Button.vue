<template>
    <button
        :type="type"
        :class="{
          'py-1 px-1': size === 'sm',
          'py-2 px-2': size === 'md',
          'py-3 px-4': size === 'lg',
          'py-4 px-8': size === 'xl',
          'rounded-full': rounded,
          'text-white border-transparent bg-gray-800 hover:bg-gray-900 focus:ring-gray-800 dark:focus:ring-gray-900 dark:focus:ring-offset-gray-800': is === 'dark',
          'text-white border-transparent bg-gray-500 hover:bg-gray-600 focus:ring-gray-500 dark:bg-gray-700 dark:hover:bg-gray-600 dark:focus:ring-offset-gray-800': is === 'secondary',
          'text-white border-transparent bg-red-500 hover:bg-red-600 focus:ring-red-500 dark:focus:ring-offset-gray-800': is === 'danger',
          'text-white border-transparent bg-yellow-500 hover:bg-yellow-600 focus:ring-yellow-500 dark:focus:ring-offset-gray-800': is === 'warning',
          'text-white border-transparent bg-green-500 hover:bg-green-600 focus:ring-green-500 dark:focus:ring-offset-gray-800': is === 'success',
          'text-white border-transparent dark:bg-indigo-600 bg-indigo-500 hover:bg-indigo-600 focus:outline-none focus:ring-indigo-500 ': is === 'primary',
          'text-white border-transparent bg-blue-500 hover:bg-blue-600 focus:ring-blue-500 dark:focus:ring-offset-gray-800': is === 'blue',
          'text-gray-700 bg-white text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-offset-white focus:ring-blue-600 dark:bg-gray-800 dark:hover:bg-slate-800 dark:border-gray-700 dark:text-gray-400 dark:hover:text-white dark:focus:ring-offset-gray-800': is === 'white',
        }"
        :disabled="loading"
        class="inline-flex justify-center items-center gap-2 rounded-md border font-semibold focus:outline-none focus:ring-2 transition-all text-sm"
    >
        <span
            v-show="loadable && loading"
            class="animate-spin inline-block w-4 h-4 border-[3px] border-current border-t-transparent rounded-full"
            role="status"
            aria-label="loading"
        />
        <slot />
    </button>
</template>

<script setup lang="ts">
import type {ButtonHTMLAttributes, PropType} from "vue";

const props = defineProps({
    size: {
        type: String,
        default: 'lg',
    },
    loadable: {
        type: Boolean,
        default: true,
    },
    loading: {
        type: Boolean,
        default: false,
    },
    rounded: {
        type: Boolean,
        default: false,
    },
    type: {
        type: String as PropType<ButtonHTMLAttributes['type']>,
        default: 'button',
    },
    is: {
        type: String as PropType<string>,
        default: 'primary',
        validator(value: string) {
            return ['dark', 'secondary', 'danger', 'warning', 'success', 'primary', 'white'].includes(value)
        }
    }
});
</script>

<style scoped>

</style>