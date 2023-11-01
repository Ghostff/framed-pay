<template>
  <!-- Features -->
  <div class="max-w-[85rem] px-4 py-10 sm:px-6 xl:px-8 lg:py-14 mx-auto">
    <div class="relative p-3 xl:p-16">
      <!-- Grid -->
      <div class="relative lg:grid lg:grid-cols-12 gap-4 xl:gap-16 lg:items-center">
        <div class="mb-10 lg:mb-0 lg:col-span-6 lg:order-2">
          <h2 class="text-2xl text-gray-800 font-bold sm:text-3xl dark:text-gray-200">
            Fully customizable rules to match your unique needs
          </h2>

          <!-- Tab Navs -->
          <nav class="grid gap-4 mt-5 md:mt-10" aria-label="Tabs" role="tablist">
            <template v-for="(tab, i) in tabs" :key="tab.title">
              <button
                  type="button"
                  class="hs-tab-active:bg-white hs-tab-active:shadow-md hs-tab-active:hover:border-transparent text-left hover:bg-gray-200 p-4 md:p-5 rounded-xl dark:hs-tab-active:bg-slate-900 dark:hover:bg-gray-700"
                  :class="{'active': i === 0}"
                  :id="`tabs-with-card-item-${i}`"
                  :data-hs-tab="`#${tab.id}`"
                  :aria-controls="tab.id"
                  role="tab"
              >
                <span class="flex">
                  <font-awesome-icon
                      class="flex-shrink-0 mt-2 h-6 w-6 md:w-7 md:h-7 hs-tab-active:text-blue-600 text-gray-800 dark:hs-tab-active:text-blue-500 dark:text-gray-200"
                      :icon="tab.icon"
                  />
                  <span class="grow ml-6">
                    <span class="block text-lg font-semibold hs-tab-active:text-blue-600 text-gray-800 dark:hs-tab-active:text-blue-500 dark:text-gray-200">{{ tab.title }}</span>
                    <span class="block mt-1 text-gray-800 dark:hs-tab-active:text-gray-200 dark:text-gray-200">{{ tab.desc }}</span>
                  </span>
                </span>
              </button>
            </template>
          </nav>
          <!-- End Tab Navs -->
        </div>
        <!-- End Col -->

        <div class="lg:col-span-6">
          <div class="relative">
            <!-- Tab Content -->
            <div>
              <div
                  v-for="(tab, i) in tabs"
                  :key="tab.title"
                  :id="tab.id"
                  role="tabpanel"
                  :class="{'hidden': i !== 0 }"
                  :aria-labelledby="`tabs-with-card-item-${i}`"
              >
                <component :is="tab.component" :title="tab.title" :description="tab.desc" />
              </div>
            </div>
            <!-- End Tab Content -->
          </div>
        </div>
        <!-- End Col -->
      </div>
      <!-- End Grid -->

      <!-- Background Color -->
      <div class="absolute z-[-1] inset-0 grid grid-cols-12 w-full h-full">
        <div class="col-span-full lg:col-span-7 lg:col-start-6 bg-gray-100 w-full h-5/6 rounded-xl sm:h-3/4 lg:h-full dark:bg-white/[.075]"></div>
      </div>
      <!-- End Background Color -->
    </div>
  </div>
  <!-- End Features -->
</template>
<script setup lang="ts">

import TabPayment from "@/views/dashboard/components/ProfileTabs/Payment.vue";
import TabUser from "@/views/dashboard/components/ProfileTabs/User.vue";
import TabVendor from "@/views/dashboard/components/ProfileTabs/DeveloperTools.vue";
import Button from "@/components/form/Button.vue";

interface Tab {title: string; desc: string; icon: string; id: string; component: object}

const tabs: Tab[] = [
    createTab('Account', 'address-card', TabUser, 'some random description'),
    createTab('Payment', 'credit-card', TabPayment, 'some random description'),
    createTab('Developer Tools', 'laptop-code', TabVendor, 'some random description'),
];
function createTab(title: string, icon: string, component: object, desc: string): Tab {
  return {
    title,
    icon,
    id: `profile-tab-${title.replace(' ', '-').toLowerCase()}`,
    desc,
    component,
  }
}
</script>
<style scoped>

</style>