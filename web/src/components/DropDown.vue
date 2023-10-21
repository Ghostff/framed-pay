<template>
  <div class="hs-dropdown relative inline-flex">
    <slot
      :id="id"
      name="toggle"
    >
      <Button
        is="transparent"
        v-if="!toggleLabel"
        :id="id"
        :size="size || 'md'"
      >
        <font-awesome-icon icon="ellipsis" />
      </Button>
      <Button
        is="transparent"
        v-else
        :id="id"
        :size="size || 'md'"
      >
        {{ toggleLabel }}
        <font-awesome-icon icon="chevron-down" />
      </Button>
    </slot>

    <div
      class="hs-dropdown-menu transition-[opacity,margin] duration hs-dropdown-open:opacity-100 opacity-0 hidden min-w-[15rem] bg-white shadow-md rounded-lg p-2 mt-2 divide-y divide-gray-200 dark:bg-gray-800 dark:border dark:border-gray-700 dark:divide-gray-700"
      :aria-labelledby="id"
    >
      <slot name="header" />
      <div
        v-for="({ label, action: menus }, i) in namedMenus"
        :key="i"
        class="py-2 first:pt-0 last:pb-0"
        :class="{ 'border-t-0 py-4': slots.header !== undefined && i == 0 }"
      >
        <span
          v-if="label"
          class="block py-2 px-3 text-xs font-medium uppercase text-gray-400 dark:text-gray-500"
        >
          {{ label }}
        </span>

        <template
          v-for="(menu, j) in menus as MenuItem[]"
          :key="`${i}-${j}`"
        >
          <router-link
            v-if="menu.isRoute"
            :to="menu.action"
            class="flex items-center gap-x-3.5 py-2 px-3 rounded-md text-sm text-gray-800 hover:bg-gray-100 focus:ring-2 focus:ring-blue-500 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
          >
            <font-awesome-icon v-if="menu.iconName" :icon="menu.iconName"/>
            {{ menu.label }}
          </router-link>
          <a
            v-else
            class="flex items-center gap-x-3.5 py-2 px-3 rounded-md text-sm text-gray-800 hover:bg-gray-100 focus:ring-2 focus:ring-blue-500 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
            href="#"
            @click.prevent="menu.action"
          >
            <font-awesome-icon v-if="menu.iconName" :icon="menu.iconName"/>
            {{ menu.label }}
          </a>
        </template>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref, type Ref, useSlots } from 'vue'
import { MenuItem } from '@/models/menuItem'
import Button from '@/components/form/Button.vue'
import { v4 as uuid } from 'uuid'

const props = defineProps<{
  options: Array<MenuItem>
  toggleLabel?: string
  size?: string
}>()

const id: string = 'dropdown-' + uuid()
const slots = useSlots()
const namedMenus: Ref = ref<Array<MenuItem>>([])
const unnamedMenus: Ref = ref<Array<MenuItem>>([])
props.options.forEach((option) => {
  if (option.isParent) {
    namedMenus.value.push(option)
  } else {
    unnamedMenus.value.push(option)
  }
})
namedMenus.value.push(MenuItem.group('', unnamedMenus.value))
</script>
<style scoped>
.border-t-0 {
  border: none !important;
}
</style>
