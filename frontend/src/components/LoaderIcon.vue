<script setup lang="ts">
import { computed } from 'vue';
import type { ModLoader } from '../types/launcher';

const props = defineProps<{ loader: ModLoader; class?: string }>();

const badge = computed(() => {
  switch (props.loader) {
    case 'Fabric':
      return { text: 'F', background: '#8b6c3f', title: 'Fabric' };
    case 'Forge':
      return { text: 'F', background: '#52525b', title: 'Forge' };
    case 'NeoForge':
      return { text: 'N', background: '#c2703d', title: 'NeoForge' };
    case 'Quilt':
      return { text: 'Q', background: '#7c5cbf', title: 'Quilt' };
    default:
      return { text: 'V', background: '#2ea44f', title: 'Vanilla' };
  }
});
</script>

<template>
  <span
    class="inline-flex items-center justify-center rounded-[3px] text-white font-bold shrink-0"
    :class="props.class"
    :style="{ backgroundColor: badge.background }"
    :title="badge.title"
  >
    {{ badge.text }}
  </span>
</template>
