<template>
  <div class="relative">
    <select
        ref="selectElement"
        :multiple="multiple"
        :class="{
           'py-3 px-4': size === undefined && !isAdvanced,
           'py-2 px-3': size === 'md' && !isAdvanced,
           'hidden': isAdvanced,
           'pr-9 block w-full border-gray-200 rounded-md text-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400': !isAdvanced,
        }"
        :required="required"
        @change="e => onChange(e.target.value)"
    >
      <option v-if="!isAdvanced" :selected="modelValue !== 0 && !modelValue" :disabled="required" :value="null">
        {{ placeholder }}
      </option>
      <option
          v-for="(opt, i) in list"
          :value="opt.getValueAsString()"
          :selected="opt.getValueAsString() === modelValue?.toString()"
          :data-hs-select-option='opt.description ? `{"description": "${opt.description}"}` : void 0'
          :key="i"
      >{{ opt.label }}{{ opt.description && !isAdvanced ? ` (${opt.description})` : '' }}
      </option>
    </select>

    <div v-if="isAdvanced" class="absolute top-1/2 end-2.5 -translate-y-1/2">
      <svg
          class="flex-shrink-0 w-4 h-4 text-gray-500"
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
      >
        <path d="m7 15 5 5 5-5"/>
        <path d="m7 9 5-5 5 5"/>
      </svg>
    </div>
  </div>
</template>
<script setup lang="ts">
import {Stringable} from "@/models/generic";
import {onMounted, ref} from "vue";
import {HSSelect} from "preline";
import {OptionItem} from "@/models/optionItem";

const emit = defineEmits<{
  (e: 'on:select', value: OptionItem<any>): void
  (e: 'update:modelValue', value: string): void
}>();

const props = withDefaults(defineProps<{
  modelValue?: string | number,
  placeholder?: string,
  required?: boolean,
  size?: string,
  isAdvanced?: boolean,
  multiple?: boolean,
  countSelection?: boolean,
  isSearchable?: boolean,
  useTags?: boolean,
  options: Array<(OptionItem<Stringable|string|number>)|string|number>,
  isBorderless?: boolean,
}>(), {
  required: true,
  isAdvanced: true,
  placeholder: 'Select an option',
});

const selectElement = ref<HTMLElement>()
const list = props.options.map(m => {
  return typeof m !== 'object' ? new OptionItem<Stringable>(m, m.toString()) : m;
}) as Array<OptionItem<Stringable>>;

onMounted(() => {
  if (!props.isAdvanced) {
    return;
  }

  const options = {
    dropdownSpace: 10,
    hasSearch: props.isSearchable,
    toggleCountText: props.multiple && props.countSelection ? 'selected' : void 0,
    placeholder: `Select ${props.multiple ? 'multiple ' : ''}option...`,
    toggleTag: '<button type="button"></button>',
    mode: props.useTags ? 'tags' : void 0,
    toggleClasses: 'hs-select-disabled:pointer-events-none hs-select-disabled:opacity-50 relative pe-9 flex text-nowrap w-full cursor-pointer bg-white rounded-lg text-start text-sm focus:border-blue-500 focus:ring-blue-500 before:absolute before:inset-0 before:z-[1] dark:bg-slate-900 dark:text-gray-400 dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600',
    dropdownClasses: 'mt-2 z-50 w-full max-h-[300px] p-1 space-y-0.5 bg-white border border-gray-200 rounded-lg overflow-hidden overflow-y-auto dark:bg-slate-900 dark:border-gray-700',
    optionClasses: 'py-2 px-4 w-full text-sm text-gray-800 cursor-pointer hover:bg-gray-100 rounded-lg focus:outline-none focus:bg-gray-100 dark:bg-slate-900 dark:hover:bg-slate-800 dark:text-gray-200 dark:focus:bg-slate-800',
    //optionTemplate: '<div class="flex justify-between items-center w-full"><span data-title></span><span class="hidden hs-selected:block"><svg class="flex-shrink-0 w-3.5 h-3.5 text-blue-600 dark:text-blue-500" xmlns="http:.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg></span></div>',
    optionTemplate: '<div><div class="flex items-center"><div class="font-semibold text-gray-800 dark:text-gray-200" data-title></div></div><div class="mt-1.5 text-sm text-gray-500" data-description></div></div>',
    searchClasses: 'block w-full text-sm border-gray-200 rounded-md focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400 py-2 px-1 my-1 mr-2',
  };

  if (props.size === 'md') {
    options.toggleClasses += ' py-2 px-3'
  } else {
    options.toggleClasses += ' py-3 px-4'
  }

  if (props.useTags) {
    options.mode = 'tags';
    options.tagsClasses = 'relative ps-0.5 pe-9 min-h-[46px] flex items-center flex-wrap text-nowrap w-full';
    options.tagsItemTemplate = '<div class="flex flex-nowrap items-center relative z-10 bg-white border border-gray-200 rounded-full p-1 m-1 dark:bg-slate-900 dark:border-gray-700"><div class="h-6 w-6 me-1" data-icon></div><div class="whitespace-nowrap" data-title></div><div class="inline-flex flex-shrink-0 justify-center items-center h-5 w-5 ms-2 rounded-full text-gray-800 bg-gray-200 hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-gray-400 text-sm dark:bg-gray-700/50 dark:hover:bg-gray-700 dark:text-gray-400 cursor-pointer" data-remove><svg class="flex-shrink-0 w-3 h-3" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg></div></div>';
    options.tagsInputClasses = 'absolute w-full py-3 px-4 pe-9 flex-1 border-gray-200 rounded-md text-sm focus:border-blue-500 focus:ring-0 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400';
  }

  if (!props.isBorderless) {
    options.toggleClasses += ' border border-gray-200 dark:border-gray-700';
  }

  const select = new HSSelect(selectElement.value as HTMLElement, options);
  select.on('change', onChange)
})

function onChange(value: string): void {
  emit('update:modelValue', value);
  emit('on:select', list.find((m: OptionItem<Stringable>) => {
    if (typeof m.value === 'object') {
      return m.getValueAsString() === value;
    }

    return m.value.toString() === value;
  }) as OptionItem<any>)
}

</script>
<style scoped>

</style>