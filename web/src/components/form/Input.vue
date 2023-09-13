<template>
  <div :data-ref="name">
    <Label
        :id="id"
        :label="label"
        :required="required"
    />
    <div class="relative">
<!--      <DateTimePicker
          v-if="type === 'date'"
          :enable-time-picker="false"
          :max-date="max"
          :min-date="min"
          :required="required"
          :start-date="startDate"
          :value="modelValue"
          @update:model-value="setInputDate"
      />-->
      <input
          :id="id"
          ref="input"
          :aria-describedby="`${id}-error`"
          :data-validatable="isValidatable"
          :max="max?.toString()"
          :maxlength="maxlength"
          :min="min?.toString()"
          :minlength="minlength"
          :name="name"
          :pattern="pattern"
          :required="required"
          :type="type === 'date' ? 'hidden' : type"
          :value="modelValue"
          :placeholder="placeholder || label"
          class="py-3 px-4 block w-full border-gray-200 rounded-md text-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400"
          @input="emitAndValidate"
      >
      <div
          :class="{ hidden: !inputError }"
          class="absolute error-var inset-y-0 right-0 flex items-center pointer-events-none pr-3"
      >
        <font-awesome-icon
            class="text-lg text-red-500"
            icon="circle-exclamation"
        />
      </div>
    </div>
    <p
        :id="`${id}-error`"
        :class="{ hidden: !inputError }"
        class="text-xs error-var err-msg text-red-600 mt-2"
    >
      {{ inputError }}
    </p>
  </div>
</template>

<script lang="ts" setup>
import {v4 as uuid} from 'uuid'
import {computed, type Ref, ref} from 'vue'
import {type ValidatableInput, Validator} from '@/utilities/validator'
import dateFormat from 'dateformat'
import Label from '@/components/form/Label.vue'

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

interface InputProps {
  modelValue: unknown
  type: string
  label: string
  name: string
  id: string
  required: boolean
  error: string
  minlength: string | number
  maxlength: string | number
  min?: string | number | Date
  max?: string | number | Date
  pattern: string
}

const props = defineProps({
  modelValue: {
    required: true
  },
  type: {
    type: String,
    default: 'text'
  },
  label: {
    type: String,
    default: null
  },
  name: {
    type: String,
    default: null
  },
  id: {
    type: String,
    default: () => 'input-' + uuid()
  },
  required: {
    type: Boolean
  },
  error: {
    type: String,
    default: null
  },
  minlength: {
    type: [Number, String]
  },
  maxlength: {
    type: [Number, String]
  },
  min: {
    type: [Number, String, Date]
  },
  max: {
    type: [Number, String, Date]
  },
  startDate: {
    type: [Date, String]
  },
  pattern: {
    type: String
  },
  hidden: {
    type: Boolean
  },
  placeholder: {
    type: String
  }
})

// input is validated if any of the bellow attributes are specified.
const isValidatable = computed(() => {
  const validationKeys = ['required', 'minlength', 'maxlength', 'min', 'max', 'pattern']
  for (const property in props) {
    if (validationKeys.includes(property) && props[property as keyof InputProps]) {
      return true
    }
  }
  return false
})

const validationError: Ref = ref<Ref>()
const input: Ref           = ref<HTMLInputElement>()
const inputError           = computed(() => validationError.value || props.error)
Validator.makeValidatable(input, validationError, props.name ?? '')

function setInputDate(date: Date): void {
  input.value.setAttribute('value', (input.value.value = dateFormat(date, 'yyyy-mm-dd')))
  input.value.dispatchEvent(new Event('input', {bubbles: true}))
}

function emitAndValidate(e: Event): void {
  emit('update:modelValue', (e.target as HTMLInputElement).value)
  if (isValidatable.value) {
    (e.target as ValidatableInput).validate()
  }
}
</script>

<style scoped></style>
