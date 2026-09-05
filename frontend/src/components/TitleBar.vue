<script setup lang="ts">
import { Minus, Square, X, PanelLeft, Sun, Moon } from 'lucide-vue-next';

defineProps<{
  isSidebarCollapsed: boolean;
  colorMode: 'dark' | 'light';
}>();

const emit = defineEmits<{
  (event: 'toggle-sidebar'): void;
  (event: 'toggle-color-mode'): void;
  (event: 'notify', message: string): void;
}>();
</script>

<template>
  <header
    class="h-10 w-full flex items-center justify-between px-3 bg-[#121315] border-b border-[#222428] select-none z-50 transition-colors"
  >
    <div class="flex items-center gap-2.5">
      <button
        class="w-7 h-7 flex items-center justify-center rounded text-slate-400 hover:bg-[#1f2125] transition-colors cursor-pointer"
        :title="isSidebarCollapsed ? '展开侧边栏' : '收起侧边栏'"
        @click="emit('toggle-sidebar')"
      >
        <PanelLeft class="w-4 h-4" />
      </button>

      <div class="flex items-center gap-2">
        <img
          src="/IMG_20260827_125438_128x128.png"
          alt="Aura Launcher"
          class="w-4.5 h-4.5 object-contain"
          @error="($event.target as HTMLImageElement).src = '/IMG_20260827_125438_128x128.svg'"
        />
        <span class="font-bold text-xs text-slate-200 tracking-wide">
          Aura Launcher Modern
        </span>
      </div>
    </div>

    <div class="flex items-center space-x-1">
      <button
        class="w-7 h-7 flex items-center justify-center rounded text-slate-400 hover:text-white hover:bg-[#202226] transition-colors cursor-pointer mr-1"
        :title="colorMode === 'dark' ? '切换为浅色主题模式' : '切换为深色主题模式'"
        @click="emit('toggle-color-mode')"
      >
        <Sun v-if="colorMode === 'dark'" class="w-3.5 h-3.5 text-amber-400 hover:rotate-45 transition-transform" />
        <Moon v-else class="w-3.5 h-3.5 text-indigo-500 hover:-rotate-12 transition-transform" />
      </button>

      <button
        class="w-7 h-7 flex items-center justify-center rounded text-slate-400 hover:text-white hover:bg-[#202226] transition-colors cursor-pointer"
        title="最小化"
        @click="emit('notify', '窗口最小化')"
      >
        <Minus class="w-3.5 h-3.5" />
      </button>
      <button
        class="w-7 h-7 flex items-center justify-center rounded text-slate-400 hover:text-white hover:bg-[#202226] transition-colors cursor-pointer"
        title="最大化"
        @click="emit('notify', '切换窗口大小')"
      >
        <Square class="w-3 h-3" />
      </button>
      <button
        class="w-7 h-7 flex items-center justify-center rounded text-slate-400 hover:text-white hover:bg-rose-700 transition-colors cursor-pointer"
        title="关闭"
        @click="emit('notify', '关闭窗口')"
      >
        <X class="w-3.5 h-3.5" />
      </button>
    </div>
  </header>
</template>
