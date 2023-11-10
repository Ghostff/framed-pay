<template>
  <div
      ref="modalElement"
      class="hs-overlay hidden w-full h-full fixed top-0 left-0 z-[60] overflow-x-hidden overflow-y-auto"
      :class="{'[--overlay-backdrop:static]': closeOption === '0' }"
  >
    <div
        class="hs-overlay-open:mt-7 hs-overlay-open:opacity-100 hs-overlay-open:duration-500 mt-0 opacity-0 ease-out transition-all"
        :class="{
                'sm:max-w-xs sm:w-full m-3 sm:mx-auto': size === 'xs',
                'sm:max-w-sm sm:w-full m-3 sm:mx-auto': size === 'sm',
                'md:max-w-2xl md:w-full m-3 md:mx-auto': size === 'md',
                'lg:max-w-4xl lg:w-full m-3 lg:mx-auto': size === 'lg',
                'min-h-[calc(100%-3.5rem)] flex items-center': position === 'center',
            }"
    >
      <div class="relative flex flex-col bg-white shadow-sm rounded-xl dark:bg-gray-800 dark:border-gray-700">
        <div :class="{
                    'flex justify-between items-center py-3 px-4 border-b dark:border-gray-700': hasHeader,
                    'absolute top-2 right-2 z-10': !hasHeader
                }">
          <h3 v-if="hasHeader" class="font-bold text-gray-800 dark:text-white">
            <slot name="header">{{ title }}</slot>
          </h3>
          <template v-if="closeOption !== '0'">
            <button
                type="button"
                @click="toggle(false)"
                :class="{
                             'inline-flex flex-shrink-0 justify-center items-center h-8 w-8 rounded-md bg-gray-100 dark:bg-slate-700 hover:opacity-70 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-white transition-all text-sm dark:focus:ring-gray-700 dark:focus:ring-offset-gray-800': closeOption === '2',
                             'hs-dropdown-toggle inline-flex flex-shrink-0 justify-center items-center h-8 w-8 rounded-md text-gray-500 hover:text-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-white transition-all text-sm dark:focus:ring-gray-700 dark:focus:ring-offset-gray-800': closeOption === '1',
                            }"
            >
              <span class="sr-only">Close</span>
              <FontX/>
            </button>
          </template>
        </div>

        <slot></slot>

        <div v-if="hasFooter" class="flex justify-end items-center gap-x-2 py-3 px-4 border-t dark:border-gray-700">
          <slot name="footer">
            <Button v-if="closeOption !== '0'" is="white" @click="toggle(false)">Close</Button>
            <Button is="primary">{{ actionName }}</Button>
          </slot>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, type PropType, ref, type Ref, useSlots, watch} from "vue";
import Button from "@/components/form/Button.vue";
import FontX from "@/components/icons/IconX.vue";
import {HSOverlay} from "preline";

const emit = defineEmits<{
  (e: 'on:open', value: HTMLElement): void
  (e: 'on:close', value: HTMLElement): void
}>();

const props = defineProps({
  size: {
    type: String,
    default: 'md',
  },
  closeOption: {
    type: String,
    default: '2',
    validator(value: string): boolean {
      return ['0', '1', '2'].includes(value)
    }
  },
  title: {
    type: String,
  },
  actionName: {
    type: String,
  },
  position: {
    type: String,
    default: 'top',
    validator(value: string): boolean {
      return ['top', 'center'].includes(value);
    }
  },
  isOpen: {
    type: Boolean as PropType<boolean>,
    default: false,
  }
});

const modalElement: Ref = ref<Element>();
const slots = useSlots();
const hasHeader = computed(() => slots.header !== undefined || props.title !== undefined);
const hasFooter = computed(() => slots.footer !== undefined || props.actionName !== undefined);
let modal: HSOverlay;

if (!window.$hsOverlayCollection) {
  window.$hsOverlayCollection = [];
}

watch(() => props.isOpen, toggle);

onMounted(() => {
  // @todo: check for update and inspect HSOverlay to see if the fix the workaround bellow.
  modal = new HSOverlay(document.createElement('button'));
  modal.createCollection(window.$hsOverlayCollection, modal.overlay = modalElement.value);

  modal.on('open', () => emit('on:open', modalElement.value));
  modal.on('close', () => emit('on:close', modalElement.value));
  toggle(props.isOpen);
});

function toggle(show: boolean): void {
  if (show) {
    modal.open();
  } else {
    modal.close();
  }
}

defineExpose({
  open: () => toggle(true),
  close: () => toggle(false),
  toggle: (flag: boolean) => toggle(flag),
});
</script>

<style scoped>

</style>