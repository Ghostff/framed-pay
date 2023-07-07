<template>
    <form @submit.prevent.stop="formSubmit">
        <slot/>
    </form>
</template>

<script setup lang="ts">
import type {ValidatableInput} from "@/utility";

const emit = defineEmits<{
    (e: 'submit', event: Event): void
}>()

const props = defineProps({
    validatable: {
        type: Boolean,
        default: true,
    },
});


function formSubmit(e: Event): void {
    if (props.validatable) {
        const inputs: NodeList = (e.target as HTMLFormElement).querySelectorAll('input[data-validatable="true"]');
        for (let i = 0; i < inputs.length; ++i) {
            if (!(inputs[i] as ValidatableInput).validate()) {
                return;
            }
        }
    }
    emit('submit', e);
}
</script>

<style scoped>

</style>