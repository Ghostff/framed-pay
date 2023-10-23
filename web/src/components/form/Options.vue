<template>
  <select
      :class="{ 'py-3 px-4': size === undefined, 'py-2 px-3': size === 'md' }"
      :required="required"
      class="pr-9 block w-full border-gray-200 rounded-md text-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400"
      @change="onChange"
  >
    <option :selected="modelValue !== 0 && !modelValue" :disabled="required" :value="null">{{ placeholder }}</option>
    <option
        v-for="(opt, i) in list"
        :value="opt.value"
        :selected="opt.value.toString() === modelValue?.toString()"
        :key="i"
    >{{ opt.label }}</option>
  </select>
</template>
<script setup lang="ts">
import {MapItem} from "@/models/mapItem";
import {Stringable} from "@/models/generic";

const emit = defineEmits<{
  (e: 'on:select', value: MapItem<Stringable>): void
  (e: 'update:modelValue', value: string): void
}>();

const props = withDefaults(defineProps<{
  modelValue?: string|number,
  placeholder?: string,
  required?: boolean,
  size?: string,
  options: Array<(MapItem<Stringable|string|number>)|string|number>
}>(), {
  required: true,
  placeholder: 'Select an option',
});

const list = props.options.map(m => {
  return typeof m !== "object" ? new MapItem<Stringable>(m, m.toString()) : m;
});

function onChange(e: InputEvent): void {
  const value = e.target?.value;
  emit('update:modelValue', value);
  emit('on:select', list.find((m: MapItem<Stringable>) => m.value.toString() === value))
}

</script>
<style scoped>

</style>