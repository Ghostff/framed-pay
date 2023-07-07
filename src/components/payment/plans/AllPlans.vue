<template>
  <!-- Pricing -->
    <div class="max-w-[85rem] px-4 py-10 sm:px-6 lg:px-8 lg:py-14 mx-auto">
        <!-- Grid -->
        <div class="mt-12 relative before:absolute before:inset-0 before:-z-[1] before:bg-[radial-gradient(closest-side,#cbd5e1,transparent)] dark:before:bg-[radial-gradient(closest-side,#334155,transparent)]">
            <div class="grid gap-px sm:grid-cols-2 lg:grid-cols-4 lg:items-center">
                <!-- Card -->
                <EachPlan :options="[]" price="Free" name="Sandbox" />
                <EachPlan :options="[]" :price="39" name="Startup" />
                <EachPlan :options="[]" :price="89" name="Team" />
                <EachPlan :options="[]" :price="149" name="Enterprise" />
            </div>
        </div>
        <!-- End Grid -->
    </div>
  <!-- End Pricing -->
</template>

<script setup lang="ts">
import EachPlan from "@/components/payment/plans/EachPlan.vue";
</script>

<style scoped>

</style>