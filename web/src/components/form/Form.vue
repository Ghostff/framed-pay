<template>
  <form ref="element" @submit.prevent.stop="formSubmit">
    <slot />
    <div class="text-red-600 text-center text-sm">{{ formError }}</div>
  </form>
</template>

<script setup lang="ts">
import { type Ref, ref } from 'vue';
import type { ValidatableInput } from '@/utilities/validator';
import type { stringKey } from '@/models/generic';

const emit = defineEmits<{
  (e: 'submit', event: Event): void;
}>();

const formError: Ref<string> = ref('');
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
    error.error = 'An error occurred.';
    for (const name in error.errors) {
      toggleErrors(error.errors, name);
    }
  }

  formError.value = error.error;
  return;
}

function toggleErrors(errors?: stringKey<string[]>, name?: string): void {
  const className = errors ? `[data-ref="${name}"] .error-var` : '.error-var';
  element.value.querySelectorAll(className).forEach((el: HTMLElement) => {
    if (errors) {
      el.classList.remove('hidden');
    } else {
      el.classList.add('hidden');
    }

    console.log(el, el.getAttribute('placeholder'));

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

<style scoped></style>