<template>
    <div class="flex flex-col h-full text-center">
        <div class="bg-white pt-8 pb-5 px-8 dark:bg-slate-900">
            <h4 class="font-medium text-lg text-gray-800 dark:text-gray-200">{{ name }}</h4>
        </div>

        <div class="h-full bg-white lg:mt-px lg:py-5 px-8 dark:bg-slate-900">
            <span class="mt-7 font-bold text-5xl text-gray-800 dark:text-gray-200">
                <span v-show="Number.isInteger(price)" class="font-bold text-2xl -mr-2">$</span> {{ price }}
            </span>
        </div>

        <div class="bg-white flex justify-center lg:mt-px pt-7 px-8 dark:bg-slate-900">
            <ul class="space-y-2.5 text-center text-sm">
                <li
                    v-for="(option, i) in planOptions"
                    :key="option"
                    class="text-gray-800 dark:text-gray-400"
                    v-show="options.includes(i.toString())"
                >
                    {{ option }}
                </li>
            </ul>
        </div>

        <div class="bg-white py-8 px-8 dark:bg-slate-900">
            <a class="inline-flex justify-center items-center gap-2 rounded-md border-2 border-blue-600 font-semibold text-blue-600 hover:shadow-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm py-3 px-6 dark:text-blue-500 dark:border-blue-500 dark:hover:border-blue-700" href="#">
                Sign up
            </a>
        </div>
    </div>
</template>

<script setup lang="ts">
const props = defineProps({
    name: {
        type: String,
        required: true,
    },
    price: {
        type: [String, Number],
        required: true,
    },
    options: {
        type: Array<String>,
        required: true,
    },
});

const planOptions: Array<string> = [

]
</script>

<style scoped>

</style>