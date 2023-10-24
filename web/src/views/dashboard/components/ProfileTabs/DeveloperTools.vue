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
      <div class="flex justify-between items-center mb-2">
        <h4 class="text-xs font-semibold uppercase text-gray-800 dark:text-gray-200 mb-2">API KEY</h4>
        <Button :tooltip="tooltip" size="sm" is="white" @click="copyKey">
          <font-awesome-icon icon="copy" />
        </Button>
      </div>

      <div class="w-full p-3 border border-gray-200 rounded-md dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 break-words">
        {{ apiKey }}
      </div>
    </div>

    <div class="mt-5 flex justify-end gap-x-2">
      <Button :loading="isBusy" size="md" @click="generateNewKey">Generate new API key</Button>
    </div>
  </div>
</template>
<script setup lang="ts">
import Button from "@/components/form/Button.vue";
import {ref, Ref} from "vue";
import axios from "axios";
import dialog from "@/utilities/dialog";

defineProps<{
  title: string,
  description: string
}>();

const tooltip: Ref<string> = ref('copy api key');
const apiKey: Ref<string> = ref('')
const isBusy: Ref<boolean> = ref(false)

axios.get('dev-tools').then(({data}: {data: {data: { api_key: string }}}) => {
  apiKey.value = data.data.api_key
});

function generateNewKey() {
  // isBusy.value = true;
  dialog({
    title: 'Regenerate API Key',
    type: 'danger',
    body: `You are about to regenerate the API key. This process will immediately invalidate your current key,
          and any applications, services, or systems using the old key will no longer be able to access this service.
          This action is irreversible.`,
    onOk: () => {
      axios.put('user/generate-api-key')
          .finally(() => isBusy.value = false)
          .then(({data}: {data: {data: { api_key: string }}}) => {
            apiKey.value = data.data.api_key;
          });
    }
  });
}

function copyKey() {
  navigator.clipboard.writeText(apiKey.value).then(() => {
    tooltip.value = 'copied api key';
    setTimeout(() => tooltip.value = 'copy api key', 2000);
  });
}
</script>
<style scoped>

</style>