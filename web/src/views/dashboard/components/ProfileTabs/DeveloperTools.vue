<template>
  <div class="bg-white rounded-xl shadow p-4 sm:p-7 dark:bg-slate-900">
    <div class="mb-8 text-center">
      <h2 class="text-xl font-bold text-gray-800 dark:text-gray-200">{{title}}</h2>
      <p class="text-sm text-gray-600 dark:text-gray-400">{{ description }}</p>
    </div>

    <!-- Grid -->
    <div class="mt-5 sm:mt-10 grid grid-cols-2 gap-5 hidden">
      <div>
        <span class="block text-xs uppercase text-gray-500">Generated:</span>
        <span class="block text-sm font-medium text-gray-800 dark:text-gray-200">$316.8</span>
      </div>
      <!-- End Col -->

      <div>
        <span class="block text-xs uppercase text-gray-500">Date paid:</span>
        <span class="block text-sm font-medium text-gray-800 dark:text-gray-200">April 22, 2020</span>
      </div>
      <!-- End Col -->
    </div>
    <!-- End Grid -->

    <div class="mt-5 sm:mt-10">
      <div class="flex justify-between items-center">
        <h4 class="text-xs font-semibold uppercase text-gray-800 dark:text-gray-200 mb-2">API KEY</h4>
        <Button :tooltip="tooltip" size="sm" is="transparent" @click="copyKey">
          <font-awesome-icon icon="copy" />
        </Button>
      </div>

      <div class="w-full p-3 border border-gray-200 rounded-md dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 break-words">
        {{ apiKey }}
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import Button from "@/components/form/Button.vue";
import {ref, Ref} from "vue";
import axios from "axios";

defineProps<{
  title: string,
  description: string
}>();

const tooltip: Ref<string> = ref('copy api key');
const apiKey: Ref<string> = ref('')

axios.get('dev-tools').then(({data}: {data: {data: { api_key: string }}}) => {
  apiKey.value = data.data.api_key
});

function copyKey() {
  navigator.clipboard.writeText(apiKey.value).then(() => {
    tooltip.value = 'copied api key';
    setTimeout(() => tooltip.value = 'copy api key', 2000);
  });
}
</script>
<style scoped>

</style>