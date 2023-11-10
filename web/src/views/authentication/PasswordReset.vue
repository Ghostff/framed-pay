<template>
    <div class="auth-container">
        <div v-if="success" class="p-1 sm:p-10 text-center overflow-y-auto">
                    <span class="mb-4 inline-flex justify-center items-center w-[62px] h-[62px] rounded-full border-4 border-green-50 bg-green-100 text-green-500 dark:bg-green-700 dark:border-green-600 dark:text-green-100">
                        <FontCheck2 class="w-7 h-7" />
                    </span>
            <h3 class="mb-2 text-2xl font-bold text-gray-800 dark:text-gray-200">
                Success!
            </h3>
            <p class="text-gray-500">Your password has been successfully changed! You can now log in with your new password</p>

            <div class="mt-4 flex justify-center gap-x-4 text-sm">
                <a
                    class="text-blue-600 decoration-2 hover:underline font-medium"
                    href="#"
                    @click.stop="$emit('toggle', 'login')">
                    Go to login page
                </a>
            </div>
        </div>
        <div v-else class="p-2 sm:p-7">
            <div class="text-center">
                <h1 class="block text-2xl font-bold text-gray-800 dark:text-white">Set Your New Password</h1>
                <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
                    Set your new login password
                </p>
            </div>

            <div v-if="isValid === null" class="text-center mt-5">
              <Loading size="md" />
            </div>
            <div v-else-if="isValid === false" class="text-center text-red-500 mt-5">
              Invalid or expired token
            </div>
            <div v-else class="mt-5">
                <Form ref="formElement" @submit="submit" @honey-pot="v => formData.confirm_email = v">
                    <div class="grid gap-y-4">
                      <Input
                          v-model="formData.password"
                          required
                          label="New Password"
                          name="password"
                          type="password"
                          minlength="6"
                          maxlength="255"
                      />

                      <Input
                          v-model="formData.repeat_password"
                          required
                          label="Repeat New Password"
                          name="password_repeat"
                          type="password"
                          minlength="6"
                          maxlength="255"
                      />
                        <Button
                            :is-loading="isProcessing"
                            :disabled="formData.password.length === 0 || formData.repeat_password.length === 0"
                            type="submit"
                        >Change Password</Button>
                    </div>
                </Form>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import {reactive, ref, type Ref} from "vue";
import Input from "@/components/form/Input.vue";
import Button from "@/components/form/Button.vue";
import Form from "@/components/form/Form.vue";
import FontCheck2 from "@/components/icons/IconCheck2.vue";
import axios from "axios";
import {useRoute, useRouter} from "vue-router";
import Loading from "@/views/Loading.vue";
import {recaptchaRequest} from "@/utilities/request";

defineEmits<{
    (e: 'toggle', value: string): void
}>();

const route = useRoute();
const isProcessing: Ref = ref<boolean>(false);
const formElement: Ref<HTMLFormElement | null> = ref(null);
const isValid: Ref<boolean|null> = ref(null);
const success: Ref<boolean> = ref(false);
const formData = reactive({
  confirm_email: '',
  password: '',
  repeat_password: ''
});


axios.post('verify-password-reset-token', route.query).then(() => isValid.value = true).catch(() => isValid.value = false)

function submit(e: Event): boolean  {
  if (formData.password !== formData.repeat_password) {
    formElement.value.setErrors({errors: {password_repeat: ['Does not match "New Password"']}, error: 'Passwords do not match'});
    e.preventDefault();
    return false;
  }

  isProcessing.value = true;
  recaptchaRequest({ method:'PUT', uri: '/change-password', data: Object.assign({}, formData, route.query) })
      .finally(() => isProcessing.value = false)
      .then(() => success.value = true)
      .catch((error) => {
        if (error.response) {
          formElement.value?.setErrors(error.response.data);
          return;
        }
        console.error(error);
      });

  return true;
}
</script>

<style scoped>

</style>