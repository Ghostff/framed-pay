<template>
  <div class="bg-white rounded-xl shadow p-4 sm:p-7 dark:bg-slate-900">
    <div class="mb-8 text-center">
      <h2 class="text-xl font-bold text-gray-800 dark:text-gray-200">{{title}}</h2>
      <p class="text-sm text-gray-600 dark:text-gray-400">{{ description }}</p>
    </div>

    <form>
      <!-- Grid -->
      <div class="grid sm:grid-cols-12 gap-2 sm:gap-6">

        <div class="sm:col-span-3">
          <label for="af-account-full-name" class="inline-block text-sm text-gray-800 mt-2.5 dark:text-gray-200">
            Full name
          </label>
        </div>
        <!-- End Col -->

        <div class="sm:col-span-9">
          <div class="sm:flex">
            <input id="af-account-full-name" type="text" class="py-2 px-3 pr-11 block w-full border-gray-200 shadow-sm -mt-px -ml-px first:rounded-t-lg last:rounded-b-lg sm:first:rounded-l-lg sm:mt-0 sm:first:ml-0 sm:first:rounded-tr-none sm:last:rounded-bl-none sm:last:rounded-r-lg text-sm relative focus:z-10 focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400" placeholder="Maria">
            <input type="text" class="py-2 px-3 pr-11 block w-full border-gray-200 shadow-sm -mt-px -ml-px first:rounded-t-lg last:rounded-b-lg sm:first:rounded-l-lg sm:mt-0 sm:first:ml-0 sm:first:rounded-tr-none sm:last:rounded-bl-none sm:last:rounded-r-lg text-sm relative focus:z-10 focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400" placeholder="Boone">
          </div>
        </div>
        <!-- End Col -->

        <div class="sm:col-span-3">
          <label for="af-account-email" class="inline-block text-sm text-gray-800 mt-2.5 dark:text-gray-200">
            Email
          </label>
        </div>
        <!-- End Col -->

        <div class="sm:col-span-9">
          <input id="af-account-email" type="email" class="py-2 px-3 pr-11 block w-full border-gray-200 shadow-sm text-sm rounded-lg focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400" placeholder="maria@site.com">
        </div>
        <!-- End Col -->

        <div class="sm:col-span-3">
          <label for="af-account-password" class="inline-block text-sm text-gray-800 mt-2.5 dark:text-gray-200">
            Password
          </label>
        </div>
        <!-- End Col -->

        <div class="sm:col-span-9">
          <div class="space-y-2">
            <input id="af-account-password" type="text" class="py-2 px-3 pr-11 block w-full border-gray-200 shadow-sm rounded-lg text-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400" placeholder="Enter current password">
            <input type="text" class="py-2 px-3 pr-11 block w-full border-gray-200 shadow-sm rounded-lg text-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400" placeholder="Enter new password">
          </div>
        </div>
        <!-- End Col -->

        <div class="sm:col-span-3">
          <div class="inline-block">
            <label for="af-account-phone" class="inline-block text-sm text-gray-800 mt-2.5 dark:text-gray-200">
              Phone
            </label>
            <span class="text-sm text-gray-400 dark:text-gray-600">
              (Optional)
            </span>
          </div>
        </div>
        <!-- End Col -->

        <div class="sm:col-span-9">
          <div class="sm:flex">
            <input id="af-account-phone" type="text" class="py-2 px-3 pr-11 block w-full border-gray-200 shadow-sm -mt-px -ml-px first:rounded-t-lg last:rounded-b-lg sm:first:rounded-l-lg sm:mt-0 sm:first:ml-0 sm:first:rounded-tr-none sm:last:rounded-bl-none sm:last:rounded-r-lg text-sm relative focus:z-10 focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400" placeholder="+x(xxx)xxx-xx-xx">
          </div>
        </div>
      </div>
      <!-- End Grid -->

      <div class="mt-5 flex justify-end gap-x-2">
        <button type="button" class="py-2 px-3 inline-flex justify-center items-center gap-2 rounded-md border font-medium bg-white text-gray-700 shadow-sm align-middle hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-blue-600 transition-all text-sm dark:bg-slate-900 dark:hover:bg-slate-800 dark:border-gray-700 dark:text-gray-400 dark:hover:text-white dark:focus:ring-offset-gray-800">
          Cancel
        </button>
        <button type="button" class="py-2 px-3 inline-flex justify-center items-center gap-2 rounded-md border border-transparent font-semibold bg-blue-500 text-white hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm dark:focus:ring-offset-gray-800">
          Save changes
        </button>
      </div>
    </form>
  </div>
</template>
<script setup lang="ts">
defineProps<{
  title: string,
  description: string
}>()
</script>
<style scoped>

</style>