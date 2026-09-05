<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import anime from 'animejs';
import LoaderIcon from './LoaderIcon.vue';
import type { MinecraftInstance, Account } from '../types/launcher';

const props = defineProps<{
  open: boolean;
  instance: MinecraftInstance;
  account: Account | undefined;
}>();

const emit = defineEmits<{
  (event: 'close'): void;
}>();

const progress = ref(0);
const barRef = ref<HTMLElement | null>(null);

watch(
  () => props.open,
  (open) => {
    progress.value = 0;
    if (open) {
      anime({
        targets: { value: 0 },
        value: 100,
        duration: 3200,
        easing: 'easeInOutQuad',
        update: (animation) => {
          progress.value = Math.round(animation.progress);
        },
      });
      window.setTimeout(() => emit('close'), 3600);
    }
  },
);

onMounted(() => void barRef.value);
</script>

<template>
  <div
    v-if="open"
    class="fixed inset-0 z-[9998] flex items-center justify-center bg-black/60 backdrop-blur-sm"
    @click.self="emit('close')"
  >
    <div class="mc-panel w-80 p-5 rounded-lg shadow-2xl">
      <div class="flex items-center gap-3 mb-4">
        <LoaderIcon :loader="instance.loader" class="w-9 h-9 text-sm rounded" />
        <div class="min-w-0 flex-1">
          <div class="text-sm font-bold text-white truncate">{{ instance.name }}</div>
          <div class="text-[10px] text-slate-400 font-mono">
            {{ instance.loader }} {{ instance.version }}
          </div>
        </div>
      </div>

      <div class="text-[10px] text-slate-400 mb-1.5 flex justify-between">
        <span>正在准备启动环境...</span>
        <span class="font-mono">{{ progress }}%</span>
      </div>
      <div class="h-1.5 rounded-full bg-[#282a2e] overflow-hidden">
        <div
          ref="barRef"
          class="h-full bg-[#22c55e] rounded-full transition-[width] duration-150"
          :style="{ width: `${progress}%` }"
        />
      </div>

      <div class="mt-3 text-[10px] text-slate-500 flex justify-between">
        <span>账户: {{ account?.username ?? '未选择' }}</span>
        <span>Java {{ instance.javaVersion }}</span>
      </div>
    </div>
  </div>
</template>
