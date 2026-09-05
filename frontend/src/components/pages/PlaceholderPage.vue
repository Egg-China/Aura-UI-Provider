<script setup lang="ts">
import type { NavTab } from '../../types/launcher';

defineProps<{
  title: string;
  activeTab: NavTab;
}>();

const emit = defineEmits<{
  (event: 'navigate', tab: NavTab): void;
}>();
</script>

<template>
  <div class="h-full w-full flex items-center justify-center p-6">
    <div class="mc-panel p-8 max-w-md text-center">
      <div class="text-sm font-bold text-white mb-2">{{ title }}</div>
      <p class="text-xs text-slate-400 leading-relaxed">
        该页面正在从 React 原型移植为 Vue 3，按 ROADMAP W3-W6 逐页交付。
        当前波次已完成启动器壳、侧边栏、标题栏、背景动效与主页。
      </p>
      <button
        class="mc-btn-green mt-5 px-4 py-1.5 text-xs"
        @click="emit('navigate', 'home')"
      >
        返回主页
      </button>
    </div>
  </div>
</template>
