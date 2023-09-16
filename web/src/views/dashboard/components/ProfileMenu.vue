<template>
  <div class="w-full flex items-center justify-end">
    <div class="flex flex-row items-center justify-end gap-2">
      <Theme />

      <button
        type="button"
        class="inline-flex flex-shrink-0 justify-center items-center gap-2 h-[2.375rem] w-[2.375rem] rounded-full font-medium bg-white text-gray-700 align-middle hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-white transition-all text-xs dark:bg-gray-800 dark:hover:bg-slate-800 dark:text-gray-400 dark:hover:text-white dark:focus:ring-gray-700 dark:focus:ring-offset-gray-800"
      >
        <svg
          class="w-3.5 h-3.5"
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          fill="currentColor"
          viewBox="0 0 16 16"
        >
          <path
            d="M8 16a2 2 0 0 0 2-2H6a2 2 0 0 0 2 2zM8 1.918l-.797.161A4.002 4.002 0 0 0 4 6c0 .628-.134 2.197-.459 3.742-.16.767-.376 1.566-.663 2.258h10.244c-.287-.692-.502-1.49-.663-2.258C12.134 8.197 12 6.628 12 6a4.002 4.002 0 0 0-3.203-3.92L8 1.917zM14.22 12c.223.447.481.801.78 1H1c.299-.199.557-.553.78-1C2.68 10.2 3 6.88 3 6c0-2.42 1.72-4.44 4.005-4.901a1 1 0 1 1 1.99 0A5.002 5.002 0 0 1 13 6c0 .88.32 4.2 1.22 6z"
          />
        </svg>
      </button>

      <DropDown :options="[...menu, new MenuItem('Logout', store.logout, 'arrow-right-from-bracket')]">
        <template #toggle="{ id }">
          <Button is="transparent" :id="id" size="sm">
            <IconPersonCircle class="w-4 h-4" />
          </Button>
        </template>
        <template #header>
          <div class="py-3 px-5 -m-2 bg-gray-100 rounded-t-lg dark:bg-gray-700">
            <p class="text-sm text-gray-500 dark:text-gray-400">Signed in as</p>
            <p class="text-sm font-medium text-gray-800 dark:text-gray-300">
              {{ (store.user as UserInterface).email }}
            </p>
          </div>
        </template>
      </DropDown>
    </div>
  </div>
</template>
<script setup lang="ts">
import IconPersonCircle from '@/components/icons/IconPersonCircle.vue';
import { MenuItem } from '@/models/menuItem';
import DropDown from '@/components/DropDown.vue';
import Button from '@/components/form/Button.vue';
import { useAuthStore } from '@/stores/auth';
import Theme from '@/components/Theme.vue';
import {UserInterface} from "@/models/user";

const store = useAuthStore();
defineProps<{
  menu: Array<MenuItem>;
}>();
</script>

<style scoped></style>