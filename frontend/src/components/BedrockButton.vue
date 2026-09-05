<script setup lang="ts">
import { computed, ref } from 'vue';
import anime from 'animejs';

const props = withDefaults(
  defineProps<{
    variant?: 'green' | 'grey' | 'danger' | 'subtle';
    size?: 'sm' | 'md' | 'lg';
    type?: 'button' | 'submit';
    disabled?: boolean;
  }>(),
  { variant: 'grey', size: 'md', type: 'button', disabled: false },
);

const buttonRef = ref<HTMLButtonElement | null>(null);

const variantClass = computed(() => {
  switch (props.variant) {
    case 'green':
      return 'mc-btn-green';
    case 'danger':
      return 'bg-rose-600 hover:bg-rose-500 active:bg-rose-700 text-white font-medium rounded-md transition-colors';
    case 'subtle':
      return 'mc-btn-subtle';
    default:
      return 'mc-btn-grey';
  }
});

const sizeClass = computed(() => {
  switch (props.size) {
    case 'sm':
      return 'px-2.5 py-1.5 text-xs';
    case 'lg':
      return 'px-5 py-2.5 text-sm font-semibold';
    default:
      return 'px-3.5 py-2 text-xs';
  }
});

function handleClick() {
  if (props.disabled) return;
  if (buttonRef.value) {
    anime({
      targets: buttonRef.value,
      scale: [1, 0.96, 1],
      duration: 140,
      easing: 'easeOutQuad',
    });
  }
}
</script>

<template>
  <button
    ref="buttonRef"
    :type="type"
    :disabled="disabled"
    class="inline-flex items-center justify-center gap-1.5 select-none cursor-pointer tracking-normal"
    :class="[variantClass, sizeClass]"
    @click="handleClick"
  >
    <slot />
  </button>
</template>
