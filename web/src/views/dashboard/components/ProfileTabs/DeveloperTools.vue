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

    <div v-for="apiKey in apiKeys" :key="apiKey.id" class="mt-5 sm:mt-10 ">
      <div class="flex justify-between items-center mb-2">
        <h4 class="text-xs font-semibold uppercase text-gray-800 dark:text-gray-200 mb-2">{{ apiKey.name }}</h4>
        <div class="actions">
          <Button
              tooltip="Delete Key"
              v-if="apiKeys.length > 1"
              size="sm"
              is="white"
              @click="remove(apiKey.id)"
              class="mr-2 text-red-500"
          >
            <font-awesome-icon icon="trash-can" />
          </Button>

          <Button :tooltip="tooltip" size="sm" is="white" @click="copyKey(apiKey.key)">
            <font-awesome-icon icon="copy" />
          </Button>
        </div>
      </div>

      <div class="w-full p-3 border border-gray-200 rounded-md dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 break-words">
        {{ apiKey.key }}
      </div>
    </div>

    <BasicModal ref="apiKeyModal" size="sm">
      <div class="py-10 px-4">
        <Input v-model="name" required label="Name" type="text" minlength="2" maxlength="200" pattern="^[\w\- ]+$" />
        <div class="text-right pt-3">
          <Button :disabled="name.length < 2" :loading="isBusy" size="md" @click.stop="generateNewKey">
            Create
          </Button>
        </div>
      </div>
    </BasicModal>

    <div class="mt-5 flex justify-end gap-x-2">
      <Button :disabled="isBusy" size="md" @click="() => apiKeyModal?.open()">Generate new API key</Button>
    </div>
  </div>
</template>
<script setup lang="ts">
import Button from "@/components/form/Button.vue";
import {ref, Ref} from "vue";
import axios from "axios";
import dialog from "@/utilities/dialog";
import BasicModal from "@/components/modals/BasicModal.vue";
import {BasicModalInterface} from "@/models/generic";
import Input from "@/components/form/Input.vue";

defineProps<{
  title: string,
  description: string
}>();

interface ApiKey {
  id: string,
  name: string,
  key: string,
}

const tooltip: Ref<string> = ref('copy api key');
const apiKeys: Ref<ApiKey[]> = ref([]);
const isBusy: Ref<boolean> = ref(false);
const apiKeyModal: Ref<BasicModalInterface|null> = ref(null);
const name: Ref<string> = ref('');

axios.get('dev-tools').then(({data}: {data: {data: { api_keys: ApiKey[] }}}) => {
  apiKeys.value = data.data.api_keys
});

function generateNewKey() {
  isBusy.value = true;
  axios.post('user/api-key', {name: name.value})
      .finally(() => isBusy.value = false)
      .then(({data}: {data: {data: ApiKey}}) => {
        apiKeyModal.value?.close();
        apiKeys.value.push(data.data);
        name.value = '';
      });
}

function remove(id: string) {
  dialog({
    title: 'Delete API Key',
    type: 'danger',
    body: `You are about to delete this API key. This process will immediately invalidate your current key,
          and any applications, services, or systems using the key will no longer be able to access this service.
          This action is irreversible.`,
    onOk: () => {
      axios.delete('user/api-key/' + id).then(() => {
        apiKeys.value.splice(apiKeys.value.findIndex(k => k.id === id), 1);
      });
    }
  });
}

function copyKey(value: string) {
  navigator.clipboard.writeText(value).then(() => {
    tooltip.value = 'copied api key';
    setTimeout(() => tooltip.value = 'copy api key', 2000);
  });
}
</script>
<style scoped>

</style>