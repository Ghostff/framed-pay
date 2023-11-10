<template>
    <div class="auth-container">
        <div v-if="success" class="p-1 sm:p-10 text-center overflow-y-auto">
                    <span class="mb-4 inline-flex justify-center items-center w-[62px] h-[62px] rounded-full border-4 border-green-50 bg-green-100 text-green-500 dark:bg-green-700 dark:border-green-600 dark:text-green-100">
                        <FontCheck2 class="w-7 h-7" />
                    </span>
            <h3 class="mb-2 text-2xl font-bold text-gray-800 dark:text-gray-200">
                Success!
            </h3>
            <p class="text-gray-500">
                We have received your request and are processing it.
                If your email address exists in our system, you will receive an email shortly with instructions on how to reset your password.
            </p>

            <div class="mt-4 flex justify-center gap-x-4 text-sm">
                <a
                    class="text-blue-600 decoration-2 hover:underline font-medium"
                    href="#"
                    @click.stop="$emit('toggle', 'login')">
                    Go back to login page
                </a>
            </div>
        </div>
        <div v-else class="p-2 sm:p-7">
            <div class="text-center">
                <h1 class="block text-2xl font-bold text-gray-800 dark:text-white">Forgot password?</h1>
                <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
                    Remember your password?
                    <a
                        class="text-blue-600 decoration-2 hover:underline font-medium"
                        href="#"
                        @click.stop="$emit('toggle', 'login')"
                    >
                        Sign in here
                    </a>
                </p>
            </div>

            <div class="mt-5">
                <Form ref="formElement" @submit="submit" @honey-pot="v => formData.confirm_email = v">
                    <div class="grid gap-y-4">
                        <Input v-model="formData.email" required label="Email address" type="email" minlength="4" maxlength="255"/>
                        <Button :is-loading="isProcessing" type="submit">Reset password</Button>
                    </div>
                </Form>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import {ref, type Ref} from "vue";
import Input from "@/components/form/Input.vue";
import Button from "@/components/form/Button.vue";
import Form from "@/components/form/Form.vue";
import FontCheck2 from "@/components/icons/IconCheck2.vue";
import {resettableReactive} from "@/utilities/utility";
import {UserInterface} from "@/models/user";
import {useAuthStore} from "@/stores/auth";
import axios from "axios";
import {recaptchaRequest} from "@/utilities/request";

defineEmits<{
    (e: 'toggle', value: string): void
}>();

const isProcessing: Ref = ref<boolean>(false);
const formElement: Ref<HTMLFormElement | null> = ref(null);
const formData = resettableReactive({
  email: 'frankchris@hotmail.com',
  confirm_email: '',
});
const success: Ref = ref<boolean>(false);

function submit(): void  {
  isProcessing.value = true;
  recaptchaRequest({ uri: '/reset-password', data: formData })
      .finally(() => (isProcessing.value = false))
      .then(response => {
        success.value = true;
        formData.reset();
        console.log(response)
      })
      .catch((error) => {
        if (error.response) {
          formElement.value?.setErrors(error.response.data);
          return;
        }

        console.error(error);
      });
}
</script>

<style scoped>

</style>