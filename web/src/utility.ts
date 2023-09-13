import {onMounted, reactive, type Ref} from "vue";

export function resettableReactive<T extends object>(obj: T) {
    const react = reactive({
        ...obj,
        reset() {
            Object.assign(react, obj)
        }
    });

    return react;
}
