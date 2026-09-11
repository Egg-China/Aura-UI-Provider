<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import anime from 'animejs';
import LoaderIcon from './LoaderIcon.vue';
import type { MinecraftInstance, Account } from '../types/launcher';
import type { AuraCoreTaskStatus } from '../bridge';

const props = defineProps<{
  open: boolean;
  instance: MinecraftInstance;
  account: Account | undefined;
  taskStatus?: AuraCoreTaskStatus | null;
}>();

const emit = defineEmits<{
  (event: 'close'): void;
}>();

const progress = ref(0);
const barRef = ref<HTMLElement | null>(null);

const liveStatus = computed(() => props.taskStatus ?? null);
const displayProgress = computed(() => {
  if (liveStatus.value && liveStatus.value.total > 0) {
    return Math.min(100, Math.round((liveStatus.value.progress / liveStatus.value.total) * 100));
  }
  return progress.value;
});
const launchPhase = computed(() => {
  if (!liveStatus.value) return '正在准备启动环境...';
  if (liveStatus.value.state === 'succeeded') return '启动完成，游戏进程运行中';
  if (liveStatus.value.state === 'failed') return `启动失败: ${liveStatus.value.error ?? '未知错误'}`;
  if (liveStatus.value.state === 'aborted') return '启动任务已中止';
  return liveStatus.value.status || 'AuraCore 原生核心正在启动...';
});

watch(
  () => props.open,
  (open) => {
    progress.value = 0;
    if (open && !props.taskStatus) {
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

watch(
  liveStatus,
  (status) => {
    if (status && (status.state === 'succeeded' || status.state === 'failed' || status.state === 'aborted')) {
      window.setTimeout(() => emit('close'), status.state === 'succeeded' ? 900 : 2600);
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
        <span>{{ launchPhase }}</span>
        <span class="font-mono">{{ displayProgress }}%</span>
      </div>
      <div class="h-1.5 rounded-full bg-[#282a2e] overflow-hidden">
        <div
          ref="barRef"
          class="h-full bg-[#22c55e] rounded-full transition-[width] duration-150"
          :style="{ width: `${displayProgress}%` }"
        />
      </div>

      <div class="mt-3 text-[10px] text-slate-500 flex justify-between">
        <span>账户: {{ account?.username ?? '未选择' }}</span>
        <span>Java {{ instance.javaVersion }}</span>
      </div>
    </div>
  </div>
</template>
