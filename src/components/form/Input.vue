<template>
    <div>
        <label v-if="label" :for="id" class="block text-sm mb-2 dark:text-white">{{ label }}</label>
        <div class="relative">
            <input
                ref="input"
                :type="type"
                :id="id"
                :name="name"
                :maxlength="maxlength"
                :minlength="minlength"
                :min="min"
                :max="max"
                :value="modelValue"
                :pattern="pattern"
                :data-validatable="isValidatable"
                class="py-3 px-4 block w-full border-gray-200 rounded-md text-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400"
                :required="required"
                :aria-describedby="`${id}-error`"
                @input="emitAndValidate"
            >
            <div v-show="inputError" class="absolute inset-y-0 right-0 flex items-center pointer-events-none pr-3">
                <FontExclamationCircleFill class="h-5 w-5 text-red-500" />
            </div>
        </div>
        <p v-show="inputError" class="text-xs text-red-600 mt-2" :id="`${id}-error`">{{ inputError }}</p>
    </div>
</template>

<script setup lang="ts">
import { v4 as uuid } from 'uuid';
import {computed, onMounted, type Ref, ref} from "vue";
import FontExclamationCircleFill from "@/components/icons/IconExclamationCircleFill.vue";
import {type ValidatableInput, Validator} from "@/utility";

const emit = defineEmits<{
    (e: 'update:modelValue', value: string): void
}>()

interface InputProps {
    modelValue: any;
    type: string;
    label: string;
    name: string;
    id: string;
    required: boolean;
    error: string;
    minlength: string|number;
    maxlength: string|number;
    min: string|number;
    max: string|number;
    pattern: string;
}
const props = defineProps({
    modelValue: {
        required: true
    },
    type: {
        type: String,
        default: 'text',
    },
    label: {
        type: String,
        default: null,
    },
    name: {
        type: String,
        default: null,
    },
    id: {
        type: String,
        default: () => 'input-' + uuid(),
    },
    required: {
        type: Boolean,
    },
    error: {
        type: String,
        default: null,
    },
    minlength: {
        type: [Number, String],
    },
    maxlength: {
        type: [Number, String],
    },
    min: {
        type: [Number, String],
    },
    max: {
        type: [Number, String],
    },
    pattern: {
        type: String,
    },
});
const validationError: Ref = ref<Ref>();
const validationKeys: Array<String> = ['required', 'minlength', 'maxlength', 'min', 'max', 'pattern'];
const isValidatable = computed(() => {
    for (const property in props) {
        if (validationKeys.includes(property) && props[property as keyof InputProps]) {
            return true;
        }
    }
    return false;
});
const inputError = computed(() => validationError.value || props.error);
const input: Ref = ref<HTMLInputElement>();

Validator.makeValidatable(input, validationError, props.name ?? '');

function emitAndValidate(e: Event): void {
    emit('update:modelValue', (e.target as HTMLInputElement).value);
    (e.target as ValidatableInput).validate();
}
</script>

<style scoped>

</style>