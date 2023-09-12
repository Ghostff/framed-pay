<template>
  <!-- ========== HEADER ========== -->
  <header
    class="sticky top-0 inset-x-0 flex flex-wrap sm:justify-start sm:flex-nowrap z-[48] w-full bg-white border-b text-sm py-1 lg:pl-64 dark:bg-gray-800 dark:border-gray-700"
  >
    <nav class="flex basis-full items-center w-full mx-auto px-4 sm:px-6 md:px-8" aria-label="Global">
      <div class="mr-5 lg:mr-0 lg:hidden">
        <Logo />
      </div>

      <div class="w-full flex items-center justify-end">
        <div class="flex flex-row items-center justify-end gap-2">
          <ProfileMenu :menu="[MenuItem.route('Profile', 'profile', 'user')]" />
        </div>
      </div>
    </nav>
  </header>
  <!-- ========== END HEADER ========== -->

  <!-- ========== SIDEBAR ========== -->
  <!-- Sidebar Toggle -->
  <div
    class="sticky top-0 inset-x-0 z-20 bg-white border-y px-4 sm:px-6 md:px-8 lg:hidden dark:bg-gray-800 dark:border-gray-700"
  >
    <div class="flex items-center py-2">
      <!-- Navigation Toggle -->
      <button
        type="button"
        class="text-gray-500 hover:text-gray-600"
        data-hs-overlay="#application-sidebar"
        aria-controls="application-sidebar"
        aria-label="Toggle navigation"
      >
        <span class="sr-only">Toggle Navigation</span>
        <IconList class="w-5 h-5" />
      </button>
      <!-- End Navigation Toggle -->

      <!-- Breadcrumb -->
      <ol class="ml-3 flex items-center whitespace-nowrap min-w-0" aria-label="Breadcrumb">
        <li class="flex items-center text-sm text-gray-800 dark:text-gray-400">
          Menu
          <IconChevronRight class="flex-shrink-0 mx-3 overflow-visible h-2.5 w-2.5 text-gray-400 dark:text-gray-600" />
        </li>
        <li class="text-sm font-semibold text-gray-800 truncate dark:text-gray-400 capitalize" aria-current="page">
          {{ routeName }}
        </li>
      </ol>
      <!-- End Breadcrumb -->
    </div>
  </div>
  <!-- End Sidebar Toggle -->
  <!-- Sidebar -->
  <div
    id="application-sidebar"
    class="hs-overlay hs-overlay-open:translate-x-0 -translate-x-full transition-all duration-300 transform hidden fixed top-0 left-0 bottom-0 z-[60] w-64 bg-white border-r border-gray-200 pt-7 pb-10 overflow-y-auto scrollbar-y lg:block lg:translate-x-0 lg:right-auto lg:bottom-0 dark:scrollbar-y dark:bg-gray-800 dark:border-gray-700"
  >
    <div class="px-6">
      <Logo />
    </div>
    <nav class="hs-accordion-group p-6 w-full flex flex-col flex-wrap" data-hs-accordion-always-open>
      <ul class="space-y-0.5">
        <NavLink title="Dashboard" :is-active="routeName == 'dashboard'" color="pink" :icon="IconHouse" />
        <NavLink
          title="Integrations"
          :is-active="routeName == 'integrations'"
          color="purple"
          :icon="IconCircleNodes"
          :sub-menu="[
            new GatewayItem('1', 'Paypal', 'https://icons8.com/icon/24459/webhook'),
            new GatewayItem('2', 'Abc', 'https://icons8.com/icon/24459/webhook'),
            new GatewayItem('3', 'NMI', 'https://icons8.com/icon/24459/webhook')
          ]"
        />
        <NavLink title="Plans" :is-active="routeName == 'plan'" color="green" :icon="IconStack" />
        <NavLink title="Documentation" :is-active="routeName == 'documentation'" color="orange" :icon="IconBookHalf" />
      </ul>
    </nav>
  </div>
  <!-- ========== END SIDEBAR ========== -->

  <!-- ========== MAIN CONTENT ========== -->
  <!-- Content -->
  <div class="w-full pt-5 px-4 sm:px-6 md:px-8 lg:pl-72">
    <router-view></router-view>
  </div>
  <!-- End Content -->
  <!-- ========== END MAIN CONTENT ========== -->
</template>

<script setup lang="ts">
import Logo from '@/components/Logo.vue';
import IconHouse from '@/components/icons/IconHouse.vue';
import IconCircleNodes from '@/components/icons/IconCircleNodes.vue';
import IconStack from '@/components/icons/IconStack.vue';
import IconBookHalf from '@/components/icons/IconBookHalf.vue';
import NavLink from '@/components/NavLink.vue';
import { GatewayItem } from '@/models/gatewayItem';
import IconList from '@/components/icons/IconList.vue';
import IconChevronRight from '@/components/icons/IconChevronRight.vue';
import { useRoute } from 'vue-router';
import ProfileMenu from '@/views/dashboard/components/ProfileMenu.vue';
import { MenuItem } from '@/models/menuItem';

const route = useRoute();
const routeName: string = <string>route.name;
</script>