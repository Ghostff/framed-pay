<template>
  <div :data-ref='name'>
    <Label
      :id="id ? `label-${id}` : ''"
      :label='label'
      :required='required'
    />
    <div class='relative' :class='`input-size-${size}`'>
      <input
        :id='id'
        ref='input'
        :aria-describedby='`${id}-error`'
        :data-validatable='isValidatable'
        :max='max'
        :maxlength='maxlength'
        :min='min'
        :minlength='minlength'
        :name='name'
        :pattern='pattern'
        :autofocus="autofocus"
        :required='required'
        :type="type === 'date' ? 'hidden' : type"
        :placeholder='placeholder'
        :class="{
            'py-3 px-4': size === undefined,
            'py-2 px-3': size === 'md',
            'ps-[5.2rem]': isPhone && !regionCode,
            'block w-full border-gray-200 rounded-md text-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400': true,
            [classes]: true,
        }"
        @focus="(e) => inputFocus(e, true)"
        @blur="(e) => inputFocus(e, false)"
        @input='emitAndValidate'
      >

      <div v-if="prepend" class="absolute inset-y-0 start-0 flex items-center pointer-events-none z-20 ps-3">
        <span class="text-gray-500">{{ prepend.text }}</span>
      </div>
      <div v-if="append" class="absolute inset-y-0 end-0 flex items-center pointer-events-none z-20 pe-3">
        <span class="text-gray-500">{{ append.text }}</span>
      </div>

      <div v-if='isPhone && !regionCode' class='absolute inset-y-0 start-0 flex items-center text-gray-500 ps-px'>
        <label for='hs-inline-leading-select-country' class='sr-only'>Country</label>
        <SelectOption
          @on:select='onCountrySelect'
          :model-value="selectedCountry?.getValueAsString() || ''"
          size='md'
          is-borderless
          is-searchable
          :options='countries'
        />
      </div>

      <div
        :class='{ hidden: !inputError }'
        class='absolute error-var inset-y-0 right-0 flex items-center pointer-events-none pr-3'
      >
        <font-awesome-icon
          class='text-lg text-red-500'
          icon='circle-exclamation'
        />
      </div>
    </div>
    <p
      :id='`${id}-error`'
      :class='{ hidden: !inputError }'
      class='text-xs error-var err-msg text-red-600 mt-2'
    >
      {{ inputError }}
    </p>
  </div>
</template>

<script lang="ts" setup>
import {computed, onMounted, type Ref, ref} from 'vue'
import {type ValidatableInput, Validator} from '@/utilities/validator'
import dateFormat from 'dateformat'
import Label from '@/components/form/Label.vue'
import {AsYouTypeFormatter} from 'google-libphonenumber'
import CountryList, {Country} from 'country-list-with-dial-code-and-flag'
import {OptionItem} from '@/models/optionItem'
import SelectOption from '@/components/form/SelectOption.vue'

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'on:country-change', value: Country): void,
}>()

interface Addons {
  text: unknown;
  pad: string,
}

interface Props {
  modelValue: unknown
  type?: string,
  label?: string,
  name?: string,
  id?: string,
  required?: boolean,
  autofocus?: boolean,
  error?: string | null
  minlength?: string | number,
  maxlength?: string | number,
  min?: string | number | Date,
  max?: string | number | Date,
  startDate?: string | Date,
  pattern?: string,
  regionCode?: string,
  hidden?: boolean,
  placeholder?: string,
  classes?: string,
  size?: string,
  leadingText?: string;
  isCurrency?: boolean;
  append?: Addons;
  prepend?: Addons;
}

const props = withDefaults(defineProps<Props>(), {
  type: 'text',
  label: '',
  classes: '',
  // id: 'input-',
  error: null,
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

const isPhone              = props.type === 'tel'
const value                = props.modelValue || props.value || ''
const validationError: Ref = ref<Ref>()
const input: Ref           = ref<HTMLInputElement>()
const inputError           = computed(() => validationError.value || props.error)
Validator.makeValidatable(input, validationError, props.name ?? '')

let formatter: AsYouTypeFormatter
const areaCode      = ref<string>('')
let selectedCountry = ref<OptionItem<Country> | null>(null)
let countries: OptionItem<Country>[]

if (isPhone) {
  countries = CountryList.getAll().map(c => {
    const item = new OptionItem(c, `${c.flag} ${c.code}`)
    if (!selectedCountry.value) {
      if (props.modelValue?.startsWith(c.dialCode)) {
        selectedCountry.value = item
      } else if (props.regionCode && c.code === props.regionCode) {
        selectedCountry.value = item
        return
      }
    }

    return item
  })

  // @ts-ignore
  if (!selectedCountry.value) {
    selectedCountry.value = countries[0]
  }

  if (props.regionCode) {
    formatter = new AsYouTypeFormatter(props.regionCode)
  }
}

onMounted(() => {
  input.value.value = props.isCurrency ? moneyFormat(value || '0.0') : value;
  if (props.prepend !== void 0) {
    input.value.style.paddingLeft = props.prepend.pad;
  }

  if (props.append !== void 0) {
    input.value.style.paddingRight = props.append.pad;
  }


  if (isPhone) {
    // @ts-ignore
    onCountrySelect(selectedCountry.value)
  }
})

function inputFocus(e: { target: HTMLInputElement }, isFocus: boolean) {
  if (!props.isCurrency) {
    return;
  }

  const value    = e.target.value.replace(/[^\d.]/, '');
  e.target.value = isFocus ? value : moneyFormat(value || 0);
}

function setInputDate(date: Date): void {
  input.value.setAttribute('value', (input.value.value = dateFormat(date, 'yyyy-mm-dd')))
  input.value.dispatchEvent(new Event('input', {bubbles: true}))
}

function emitAndValidate(e: Event): void {
  if (isPhone && e.target) {
    formatter.clear()
    e.target.value.replace(/^[^\s]*\s/, '').replace(/\D/, '').split('').map((digit: string) => {
      //@ts-ignore
      e.target.value = areaCode.value + ' ' + formatter.inputDigit(digit)
    })
  }

  let value = (e.target as HTMLInputElement).value;
  if (props.isCurrency) {
    value = value.replace(/[^\d.]/, '');
  }

  emit('update:modelValue', value)
  if (isValidatable.value) {
    (e.target as ValidatableInput).validate()
  }
}

function onCountrySelect({value: country}: { country: Country }) {
  formatter = new AsYouTypeFormatter(country.code)
  input.value.setAttribute('placeholder', (areaCode.value = (country.dialCode || '')) + ' (000) 000-0000')
  emit('on:country-change', country.value)
}
</script>

<style lang='scss'>
.input-size-md .dp-custom-input {
  @apply py-2;
}
</style>
