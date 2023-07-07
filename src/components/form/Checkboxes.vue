<template>
    <div>
        <div v-for="(cb, i) in list" class="flex items-center">
            <div class="flex">
                <input
                        :id="id + i"
                        :name="name"
                        type="checkbox"
                        :required="required"
                        :data-validatable="required"
                        @input="onChange"
                        :value="cb.value"
                        :data-index="i"
                        class="shrink-0 mt-0.5 border-gray-200 rounded text-blue-600 cursor-pointer focus:ring-blue-500 dark:bg-gray-800 dark:border-gray-700 dark:checked:bg-blue-500 dark:checked:border-blue-500 dark:focus:ring-offset-gray-800"
                />
            </div>
            <div class="ml-3">
                <label :for="id + i" class="text-sm dark:text-white" role="button">
                    <slot>{{ cb.label }}</slot>
                </label>
                <div v-show="error" class="text-xs text-red-600 mt-0" :id="`${id}-error`">{{ error }}</div>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import { v4 as uuid } from 'uuid';
import {MapItem} from "@/models/mapItem";
import {computed, type PropType, ref, type Ref} from "vue";
import Input from "@/components/form/Input.vue";

const emit = defineEmits<{
    (e: 'update:modelValue', value: string|Array<any>): void
}>()

const props = defineProps({
    modelValue: {
        required: true
    },
    value: {
        type: [String, Array<MapItem>, MapItem],
        required: true,
    },
    name: {
        type: String,
        default: null,
    },
    id: {
        type: String as PropType<string>,
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
const list = computed(() => {
    if (typeof props.value === 'string') {
        return [new MapItem(props.value, props.value)]
    } else if (props.value instanceof MapItem) {
        return [props.value]
    } else {
        return props.value;
    }
});
const checked: Array<string> = [];
const input: Ref = ref<Ref>();

function onChange(e: Event): void {
    const value: string = (e.target as HTMLInputElement).value;
    const index: number = checked.indexOf(value);
    if (index > -1) {
        checked.splice(index, 1);
    } else {
        checked.push(value);
    }

    emit('update:modelValue', Array.isArray(props.value) ? checked : checked[0] ?? '');
}
</script>

<style scoped>

</style>