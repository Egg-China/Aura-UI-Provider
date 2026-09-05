<script setup lang="ts">
import { ref } from 'vue';
import { Play, ChevronUp, Check } from 'lucide-vue-next';
import LoaderIcon from '../LoaderIcon.vue';
import type { MinecraftInstance } from '../../types/launcher';

defineProps<{
  currentInstance: MinecraftInstance;
  instances: MinecraftInstance[];
  isLaunching: boolean;
}>();

const emit = defineEmits<{
  (event: 'select-instance', instance: MinecraftInstance): void;
  (event: 'launch'): void;
}>();

const showInstanceMenu = ref(false);

function selectInstance(instance: MinecraftInstance) {
  emit('select-instance', instance);
  showInstanceMenu.value = false;
}
</script>

<template>
  <div class="relative h-full w-full flex flex-col justify-end select-none p-6">
    <div class="flex-1" />

    <div class="relative self-end flex items-center gap-3 z-30">
      <div v-if="showInstanceMenu" class="absolute bottom-12 right-0 w-64 mc-panel border border-[#2e323b] rounded-md shadow-lg overflow-hidden py-1 z-50">
        <div class="px-3 py-1.5 border-b border-[#24262b] text-[10px] font-bold text-slate-400 uppercase tracking-wider flex items-center justify-between">
          <span>选择安装实例</span>
          <span class="font-mono text-[9px] text-slate-500">{{ instances.length }} 个</span>
        </div>
        <div class="max-h-52 overflow-y-auto p-1 space-y-0.5">
          <div
            v-for="inst in instances"
            :key="inst.id"
            class="flex items-center justify-between px-2.5 py-1.5 rounded cursor-pointer transition-colors"
            :class="inst.id === currentInstance.id
              ? 'bg-[#22c55e] text-white font-semibold'
              : 'hover:bg-[#1f2125] text-slate-300'"
            @click="selectInstance(inst)"
          >
            <div class="flex items-center gap-2 min-w-0">
              <LoaderIcon :loader="inst.loader" class="w-3.5 h-3.5 text-[9px]" />
              <div class="min-w-0">
                <div class="text-xs truncate">{{ inst.name }}</div>
                <div class="text-[10px] font-mono" :class="inst.id === currentInstance.id ? 'text-white/80' : 'text-slate-400'">
                  {{ inst.loader }} {{ inst.version }}
                </div>
              </div>
            </div>
            <Check v-if="inst.id === currentInstance.id" class="w-3.5 h-3.5 text-white shrink-0" />
          </div>
        </div>
      </div>

      <div class="text-right hidden sm:block">
        <div class="text-xs font-bold text-white flex items-center gap-1.5 justify-end">
          <LoaderIcon :loader="currentInstance.loader" class="w-3.5 h-3.5 text-[9px]" />
          <span class="truncate max-w-[180px]">{{ currentInstance.name }}</span>
        </div>
        <div class="text-[10px] text-slate-400 font-mono">
          {{ currentInstance.loader }} {{ currentInstance.version }}
        </div>
      </div>

      <div class="flex items-stretch rounded-md overflow-hidden bg-[#22c55e] hover:bg-[#16a34a] transition-colors border border-[#16a34a]">
        <button
          :disabled="isLaunching"
          class="flex items-center justify-center gap-2 px-6 py-2 text-white font-semibold text-xs tracking-wide cursor-pointer border-r border-white/20 transition-all active:scale-[0.98] disabled:opacity-50"
          @click="emit('launch')"
        >
          <template v-if="isLaunching">
            <div class="w-3.5 h-3.5 border-2 border-white border-t-transparent rounded-full animate-spin" />
            <span>启动中...</span>
          </template>
          <template v-else>
            <Play class="w-3.5 h-3.5 fill-current" />
            <span>开始游戏</span>
          </template>
        </button>

        <button
          class="px-2.5 text-white flex items-center justify-center hover:bg-black/10 active:bg-black/20 cursor-pointer transition-colors"
          title="选择安装实例"
          @click="showInstanceMenu = !showInstanceMenu"
        >
          <ChevronUp class="w-3.5 h-3.5 transition-transform duration-150" :class="showInstanceMenu ? 'rotate-180' : ''" />
        </button>
      </div>
    </div>
  </div>
</template>
