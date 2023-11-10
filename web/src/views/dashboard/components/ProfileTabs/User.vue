<template>
  <div class="bg-white rounded-xl shadow p-4 sm:p-7 dark:bg-slate-900">
    <div class="mb-8 text-center">
      <h2 class="text-xl font-bold text-gray-800 dark:text-gray-200">{{title}}</h2>
      <p class="text-sm text-gray-600 dark:text-gray-400">{{ description }}</p>
    </div>

    <Form ref="formElement" @submit="save" @honey-pot="v => formData.confirm_email = v">
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
            <Input v-model="formData.first_name" maxlength="255" required classes="rounded-tr-none rounded-br-none" size="md" />
            <Input v-model="formData.last_name" maxlength="255" required classes="rounded-tl-none rounded-bl-none -ml-px" size="md" />
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
          <Input v-model="formData.email" required type="email" size="md" minlength="4" maxlength="255" />
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
            <Input v-model="formData.current_password" minlength="6" maxlength="255" type="password" size="md" placeholder="Enter current password" />
            <Input v-model="formData.new_password" minlength="6" maxlength="255" type="password" size="md" placeholder="Enter new password" />
          </div>
        </div>
        <!-- End Col -->

        <div class="sm:col-span-3">
          <div class="inline-block">
            <label for="af-account-phone" class="inline-block text-sm text-gray-800 mt-2.5 dark:text-gray-200">
              Phone
            </label>
            <span class="text-xs text-gray-400 dark:text-gray-600">
              (Optional)
            </span>
          </div>
        </div>
        <!-- End Col -->

        <div class="sm:col-span-9">
          <div class="sm:flex">
            <Input v-model="formData.phone" minlength="10" size="md" type="tel" placeholder="+x (xxx) xxx-xxxx" />
          </div>
        </div>
      </div>
      <!-- End Grid -->

      <div class="mt-5 flex justify-end gap-x-2">
        <Button :is-loading="isProcessing" size="md" type="submit">Save Changes</Button>
      </div>


    </Form>
  </div>
</template>
<script setup lang="ts">
import {onMounted, ref, Ref} from "vue";
import {EditableUserInterface} from "@/models/user";
import {useAuthStore} from "@/stores/auth";
import Input from "@/components/form/Input.vue";
import Form from "@/components/form/Form.vue";
import Button from "@/components/form/Button.vue";
import {recaptchaRequest} from "@/utilities/request";
import DropDown from "@/components/DropDown.vue";
import {HSSelect} from "preline";
import Options from "@/components/form/Options.vue";
import {OptionItem} from "@/models/optionItem";

defineProps<{
  title: string,
  description: string
}>();

const store = useAuthStore();
const isProcessing: Ref<boolean> = ref(false);
const formElement: Ref<HTMLFormElement|null> = ref(null);
// @ts-ignore
const formData: Ref<EditableUserInterface> = ref({
  current_password: null,
  new_password: null,
  confirm_email: null,
  ...store.user
});

function save(): void {
  // isProcessing.value = true;
  /*recaptchaRequest({ uri: '', data: formData }).then(() => {

  })*/
}
</script>
<style scoped>

</style>