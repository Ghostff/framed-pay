<template>
    <li :class="{'hs-accordion': hasSubMenu }">
        <a
            class="group flex items-center gap-x-3.5 py-2 px-2.5 text-sm rounded-md dark:text-white hover:bg-gray-100 dark:hover:bg-gray-900 dark:text-slate-400 dark:hover:text-slate-300"
            :class="{
                'bg-gray-100 dark:bg-gray-900': isActive,
                'hs-accordion-toggle dark:hs-accordion-active:text-white hs-accordion-active:text-blue-600 hs-accordion-active:hover:bg-transparent': hasSubMenu
            }"
            href="javascript:;"
        >
            <div
                v-if="icon"
                class="p-1.5 border rounded-md shadow-sm dark:border-gray-700"
                :class="[`group-hover:shadow-${color}-200`, `dark:group-hover:shadow-${color}-900`]"
            >
                <component :is="icon" :class="`text-${color}-600`" />
            </div>
            {{ title }}

            <template v-if="hasSubMenu">
                <IconChevronUp class="hs-accordion-active:block ml-auto hidden w-4 h-4 text-gray-600 group-hover:text-gray-500 dark:text-gray-400" />
                <IconChevronDown class="hs-accordion-active:hidden ml-auto block bold w-4 h-4 text-gray-600 group-hover:text-gray-500 dark:text-gray-400" />
            </template>
        </a>
        <div v-if="subMenu" :id="`accordion-sub-${id}`" class="hs-accordion-content w-full overflow-hidden transition-[height] duration-300 hidden">
            <ul class="pl-4">
                <li v-for="menu in subMenu" :key="menu.id">
                    <a
                        class="flex items-center gap-x-3.5 py-2 px-2.5 text-sm text-slate-700 rounded-md hover:bg-gray-100 dark:hover:bg-gray-700 dark:text-slate-400 dark:hover:text-slate-300"
                        href="javascript:;"
                    >
                        <IconBoxSeam class="text-yellow-600" /> {{ menu.name }}
                    </a>
                </li>
            </ul>
        </div>
        <template v-if="false">
            <!--            Register dynamic classes-->
            <div :class="[
                'group-hover:shadow-pink-200',
                'dark:group-hover:shadow-pink-900',
                'text-pink-600',

                'group-hover:shadow-blue-200',
                'dark:group-hover:shadow-blue-900',
                'text-blue-600',

                'group-hover:shadow-green-200',
                'dark:group-hover:shadow-green-900',
                'text-green-600',

                'group-hover:shadow-purple-200',
                'dark:group-hover:shadow-purple-900',
                'text-purple-600',

                'group-hover:shadow-orange-200',
                'dark:group-hover:shadow-orange-900',
                'text-orange-600',
            ]" />
        </template>
    </li>
</template>

<script setup lang="ts">
import { v4 as uuid } from 'uuid';
import type {GatewayItem} from "@/models/gatewayItem";
import IconChevronUp from "@/components/icons/IconChevronUp.vue";
import IconChevronDown from "@/components/icons/IconChevronDown.vue";
import IconBoxSeam from "@/components/icons/IconBoxSeam.vue";

const props = defineProps({
    title: {
        type: String,
        required: true,
    },
    color: {
        type: String,
        required: true,
    },
    icon: {
        type: Object,
        required: false
    },
    subMenu: {
        type: Array<GatewayItem>,
        required: false
    },
    isActive: {
        type: Boolean,
        required: false
    }
});

const id: string = uuid();
const hasSubMenu: boolean = Array.isArray(props.subMenu) && props.subMenu.length > 0;
</script>

<style>
a.group {
    font-weight: 600;
}
</style>