<template>
  <div class="flex">
      <input
          :id="id"
          ref="input"
          :name="name"
          type="checkbox"
          :required="required"
          :checked="isChecked"
          :data-validatable="required"
          @input="onChange"
          class="shrink-0 mt-0.5 border-gray-200 rounded text-blue-600 cursor-pointer focus:ring-blue-500 dark:bg-gray-800 dark:border-gray-700 dark:checked:bg-blue-500 dark:checked:border-blue-500 dark:focus:ring-offset-gray-800"
      />
      <label :for="id" class="text-sm dark:text-white ml-2" role="button">
          <slot>{{ label }}</slot>
      </label>
      <p v-show="validationError" class="text-xs text-red-600 mt-0" :id="`${id}-error`">{{ validationError }}</p>
  </div>
</template>
<script setup lang="ts">
import { v4 as uuid } from 'uuid';
import {computed, ref, type Ref} from "vue";
import Input from "@/components/form/Input.vue";
import {type ValidatableInput, Validator} from "@/utility";

const emit = defineEmits<{
    (e: 'update:modelValue', value: string|Array<any>): void
}>()

const props = defineProps({
    modelValue: {
        required: true
    },
    value: {
        required: false,
    },
    label: {
        type: String,
        required: false,
    },
    name: {
        type: String,
        default: null,
    },
    id: {
        type: String,
        default: () => 'checkbox-' + uuid(),
    },
    required: {
        type: Boolean,
        default: false,
    },
    error: {
        type: String,
        default: null,
    }
});
const input: Ref = ref<Ref>();
const validationError: Ref = ref<string|null>(props.error);
const isChecked = computed(() => {
    if (props.value !== undefined) {
        return props.value === props.modelValue;
    }

    return ['true', 'TRUE', '1', 1, true, 'on', 'ON'].includes(<any>props.modelValue);
});


Validator.makeValidatable(input, validationError, props.name ?? '');

function onChange(e: Event): void {
    const input: ValidatableInput = e.target as ValidatableInput;

    emit('update:modelValue', input.value);
    input.validate();
}
</script>

<style scoped>

</style>