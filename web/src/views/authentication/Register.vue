<template>
  <div class="p-4 sm:p-7 auth-container">
    <div class="text-center">
      <h1 class="block text-2xl font-bold text-gray-800 dark:text-white">Sign up</h1>
      <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
        Already have an account?
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
      <button
        type="button"
        class="w-full py-3 px-4 inline-flex justify-center items-center gap-2 rounded-md border font-medium bg-white text-gray-700 shadow-sm align-middle hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-blue-600 transition-all text-sm dark:bg-gray-800 dark:hover:bg-slate-800 dark:border-gray-700 dark:text-gray-400 dark:hover:text-white dark:focus:ring-offset-gray-800"
      >
        <svg class="w-4 h-auto" width="46" height="47" viewBox="0 0 46 47" fill="none">
          <path
            d="M46 24.0287C46 22.09 45.8533 20.68 45.5013 19.2112H23.4694V27.9356H36.4069C36.1429 30.1094 34.7347 33.37 31.5957 35.5731L31.5663 35.8669L38.5191 41.2719L38.9885 41.3306C43.4477 37.2181 46 31.1669 46 24.0287Z"
            fill="#4285F4"
          />
          <path
            d="M23.4694 47C29.8061 47 35.1161 44.9144 39.0179 41.3012L31.625 35.5437C29.6301 36.9244 26.9898 37.8937 23.4987 37.8937C17.2793 37.8937 12.0281 33.7812 10.1505 28.1412L9.88649 28.1706L2.61097 33.7812L2.52296 34.0456C6.36608 41.7125 14.287 47 23.4694 47Z"
            fill="#34A853"
          />
          <path
            d="M10.1212 28.1413C9.62245 26.6725 9.32908 25.1156 9.32908 23.5C9.32908 21.8844 9.62245 20.3275 10.0918 18.8588V18.5356L2.75765 12.8369L2.52296 12.9544C0.909439 16.1269 0 19.7106 0 23.5C0 27.2894 0.909439 30.8731 2.49362 34.0456L10.1212 28.1413Z"
            fill="#FBBC05"
          />
          <path
            d="M23.4694 9.07688C27.8699 9.07688 30.8622 10.9863 32.5344 12.5725L39.1645 6.11C35.0867 2.32063 29.8061 0 23.4694 0C14.287 0 6.36607 5.2875 2.49362 12.9544L10.0918 18.8588C11.9987 13.1894 17.25 9.07688 23.4694 9.07688Z"
            fill="#EB4335"
          />
        </svg>
        Sign up with Google
      </button>

      <Separator>Or</Separator>

      <!-- Form -->
      <Form ref="formElement" @submit="submit" @honey-pot="v => formData.confirm_email = v">
        <div class="grid gap-y-4">
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 lg:gap-4">
            <div>
              <Input
                v-model="formData.first_name"
                required
                label="First Name"
                type="text"
                name="first_name"
                maxlength="255"
              />
            </div>
            <div>
              <Input
                v-model="formData.last_name"
                required
                label="Last Name"
                type="text"
                name="last_name"
                maxlength="255"
              />
            </div>
          </div>

          <Input
            v-model="formData.email"
            required
            label="Email address"
            name="email"
            type="email"
            minlength="4"
            maxlength="255"
          />

          <Input
            v-model="formData.password"
            required
            label="Password"
            name="password"
            type="password"
            minlength="6"
            maxlength="255"
          />

          <Checkbox v-model="formData.remember_me" required>
            I accept the
            <Link name="login">Terms and Conditions</Link>
          </Checkbox>
          <Button :loading="isProcessing" type="submit">Sign up</Button>
        </div>
      </Form>
      <!-- End Form -->
    </div>
  </div>
</template>

<script setup lang="ts">
import Separator from '@/components/form/Separator.vue';
import Link from '@/components/Link.vue';
import Form from '@/components/form/Form.vue';
import Input from '@/components/form/Input.vue';
import Checkbox from '@/components/form/Checkbox.vue';
import Button from '@/components/form/Button.vue';
import { resettableReactive } from '@/utility';
import axios from 'axios';
import type { Ref } from 'vue';
import { ref } from 'vue';
import { UserInterface } from '@/models/user';
import { useAuthStore } from '@/stores/auth';
import {recaptchaRequest} from "@/utilities/request";

defineEmits<{
  (e: 'toggle', value: string): void;
}>();

const formElement: Ref<HTMLFormElement | null> = ref(null);
const isProcessing: Ref = ref<boolean>(false);
const formData = resettableReactive({
  first_name: 'Chrys',
  last_name: 'Ugwu',
  email: 'frankchris@hotmail.com',
  confirm_email: '',
  password: 'p@ssw0rd!',
  remember_me: true
});

function submit(): void {
  isProcessing.value = true;
  recaptchaRequest({ uri: '/register', data: formData })
      .finally(() => (isProcessing.value = false))
      .then(({ data }: { data: { data: UserInterface } }) => useAuthStore().login(data.data))
      .catch((error) => {
        if (error.response) {
          formElement.value?.setErrors(error.response.data);
          return;
        }

        console.error(error);
      });



}
</script>

<style scoped></style>