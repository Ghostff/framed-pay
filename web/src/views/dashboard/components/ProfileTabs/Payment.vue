<template>
  <div class="bg-white rounded-xl shadow p-4 sm:p-7 dark:bg-slate-900">
    <div class="mb-8 text-center">
      <h2 class="text-xl font-bold text-gray-800 dark:text-gray-200">{{title}}</h2>
      <p class="text-sm text-gray-600 dark:text-gray-400">{{ description }}</p>
    </div>

    <form>
      <!-- Section -->
      <div class="py-6 first:pt-0 last:pb-0 border-t first:border-transparent border-gray-200 dark:border-gray-700">
        <label for="af-payment-billing-address" class="inline-block text-sm font-medium dark:text-white">
          Billing address
        </label>

        <div class="mt-2 space-y-3">
          <Input v-model="name" placeholder="Street Address" size="md" />
          <Input v-model="name" placeholder="Apt, Floor, Suite, Building" size="md" />

          <div class="sm:flex gap-3">
            <Input v-model="name" placeholder="Postal Code" size="md" />
            <Input v-model="name" placeholder="City" size="md" />
            <Input v-model="name" placeholder="State" size="md" maxlength="2" minlength="2" />
          </div>
        </div>
      </div>
      <!-- End Section -->

      <!-- Section -->
      <div class="py-6 first:pt-0 last:pb-0 border-t first:border-transparent border-gray-200 dark:border-gray-700">
        <label for="af-payment-payment-method" class="inline-block text-sm font-medium dark:text-white">
          Payment method
        </label>

        <div class="mt-2 space-y-3">
          <input id="af-payment-payment-method" type="text" class="py-2 px-3 pr-11 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400" placeholder="Name on Card">
          <input type="text" class="py-2 px-3 pr-11 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400" placeholder="Card Number">
          <div class="sm:flex gap-3">
            <input type="text" class="py-2 px-3 pr-11 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400" placeholder="Expiration Date">
            <input type="text" class="py-2 px-3 pr-11 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400" placeholder="CVV Code">
          </div>
        </div>
      </div>
      <!-- End Section -->
    </form>

    <div class="mt-5 flex justify-end gap-x-2">
      <button type="button" class="py-2 px-3 inline-flex justify-center items-center gap-2 rounded-md border font-medium bg-white text-gray-700 shadow-sm align-middle hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-blue-600 transition-all text-sm dark:bg-slate-900 dark:hover:bg-slate-800 dark:border-gray-700 dark:text-gray-400 dark:hover:text-white dark:focus:ring-offset-gray-800">
        Cancel
      </button>
      <button type="button" class="py-2 px-3 inline-flex justify-center items-center gap-2 rounded-md border border-transparent font-semibold bg-blue-500 text-white hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm dark:focus:ring-offset-gray-800">
        Save changes
      </button>
    </div>
  </div>
</template>
<script setup lang="ts">
import Input from "@/components/form/Input.vue";
import {ref} from "vue";

defineProps<{
  title: string,
  description: string
}>();

const name = ref();

</script>
<style scoped>

</style>