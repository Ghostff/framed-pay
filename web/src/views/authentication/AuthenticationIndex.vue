<template>
    <div class="max-w-[85rem] px-4 py-0 mx-auto">
        <!-- Grid -->
        <div class="lg:grid lg:grid-cols-12 lg:gap-8 lg:items-center">
            <div class="lg:col-span-7 py-20">
                <!-- Grid -->
                <div class="grid grid-cols-12 gap-2 sm:gap-6 items-center lg:-translate-x-10">
                    <div class="col-span-4">
                        <img class="rounded-xl" src="https://images.unsplash.com/photo-1606868306217-dbf5046868d2?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1981&q=80" alt="Image Description">
                    </div>
                    <!-- End Col -->

                    <div class="col-span-3">
                        <img class="rounded-xl" src="https://images.unsplash.com/photo-1605629921711-2f6b00c6bbf4?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=987&q=80" alt="Image Description">
                    </div>
                    <!-- End Col -->

                    <div class="col-span-5">
                        <img class="rounded-xl" src="https://images.unsplash.com/photo-1600194992440-50b26e0a0309?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=987&q=80" alt="Image Description">
                    </div>
                    <!-- End Col -->
                </div>
                <!-- End Grid -->
            </div>
            <!-- End Col -->

            <div class="mt-5 sm:mt-10 lg:mt-0 lg:col-span-5">
                <div class="space-y-6 sm:space-y-8 mt-5 relative overflow-hidden">
                    <Login v-if="authState === 'login'" @toggle="setState" />
                    <Register v-else-if="authState === 'register'" @toggle="setState" />
                    <ForgotPassword v-else-if="authState === 'forgot-password'" @toggle="setState" />
                </div>
            </div>
            <!-- End Col -->
        </div>
        <!-- End Grid -->
    </div>
</template>

<script setup lang="ts">
import {ref, type Ref} from "vue";
import Login from "@/views/authentication/Login.vue";
import Register from "@/views/authentication/Register.vue";
import ForgotPassword from "@/views/authentication/ForgotPassword.vue";

const props = defineProps({
    show: {
        type: String,
        default: 'login',
    }
});

const authState: Ref = ref(props.show);

function setState(state: any) {
    authState.value = state;
}
</script>

<style scoped>

</style>