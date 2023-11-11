<template>
  <form ref="element" @submit.prevent.stop="formSubmit">
    <div class="email-container">
      <div class="confirm-container">
        <div class="confirm-email-container">
          <Input
              v-model="honeyPot"
              type="email"
              name="confirm_email"
              tabindex="-1"
              autocomplete="off"
              placeholder="Confirm Email"
          />
        </div>
      </div>
    </div>

    <slot />
    <div class="text-red-600 text-center text-sm">{{ formError }}</div>
  </form>
</template>

<script setup lang="ts">
import {type Ref, ref, watch} from 'vue';
import type { ValidatableInput } from '@/utilities/validator';
import type { stringKey } from '@/models/generic';
import Input from "@/components/form/Input.vue";

const emit = defineEmits<{
  (e: 'submit', event: Event): void;
  (e: 'honey-pot', value: string): void;
}>();

const formError: Ref<string> = ref('');
const honeyPot: Ref<string> = ref('');
const element: Ref = ref<Array<HTMLElement>>([]);
const props = defineProps({
  validatable: {
    type: Boolean,
    default: true
  }
});

interface Error {
  error: string;
  errors: stringKey<string[]>;
}

watch(honeyPot, (value) => emit('honey-pot', value))

function formSubmit(e: Event): void {
  formError.value = '';

  if (props.validatable) {
    const inputs: NodeList = (e.target as HTMLFormElement).querySelectorAll(
      'input[data-validatable="true"],select[data-validatable="true"]'
    );
    for (let i = 0; i < inputs.length; ++i) {
      if (!(inputs[i] as ValidatableInput).validate()) {
        return;
      }
    }
  }
  emit('submit', e);
}

function clear(): void {
  toggleErrors();
}

function setErrors(error: Error): void {
  clear();
  if (error.errors) {
    error.error = error.error || 'An error occurred.';
    for (const name in error.errors) {
      console.log(name, error.errors)
      toggleErrors(error.errors, name);
    }
  }


  formError.value = error.error;
  return;
}

function toggleErrors(errors?: stringKey<string[]>, name?: string): void {
  const className = errors ? `[data-ref="${name}"] .error-var` : '.error-var';
  console.log(className, element, element.value.querySelectorAll(className))
  element.value.querySelectorAll(className).forEach((el: HTMLElement) => {
    if (errors) {
      el.classList.remove('hidden');
    } else {
      el.classList.add('hidden');
    }

    if (el.classList.contains('err-msg')) {
      el.textContent = errors && name ? errors[name][0] : '';
    }
  });
}

defineExpose({
  setErrors,
  clear
});
</script>

<style scoped>
.email-container {
  display: none;
}
</style>