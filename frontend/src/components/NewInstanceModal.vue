<script setup lang="ts">
import { ref, watch } from 'vue';
import { X, Layers } from 'lucide-vue-next';
import BedrockButton from './BedrockButton.vue';
import { AVAILABLE_MC_VERSIONS } from '../data/mockData';
import type { MinecraftInstance, ModLoader } from '../types/launcher';

const props = defineProps<{
  open: boolean;
}>();

const emit = defineEmits<{
  (event: 'close'): void;
  (event: 'create-instance', instance: MinecraftInstance): void;
}>();

const name = ref('新建世界 / 实例');
const version = ref('1.21.4');
const loader = ref<ModLoader>('Fabric');
const icon = ref('⛏️');
const isCreating = ref(false);

const loaders: ModLoader[] = ['Fabric', 'Forge', 'NeoForge', 'Vanilla'];

watch(
  () => props.open,
  (open) => {
    if (open) {
      name.value = '新建世界 / 实例';
      version.value = '1.21.4';
      loader.value = 'Fabric';
      icon.value = '⛏️';
      isCreating.value = false;
    }
  },
);

function handleSubmit() {
  isCreating.value = true;

  window.setTimeout(() => {
    const newInst: MinecraftInstance = {
      id: `inst-${Date.now()}`,
      name: name.value.trim() || `${version.value} ${loader.value}`,
      version: version.value,
      loader: loader.value,
      loaderVersion:
        loader.value === 'Fabric' ? '0.16.9' : loader.value === 'Forge' ? '47.2.20' : undefined,
      icon: icon.value,
      lastPlayed: '从未',
      playTime: '0.0 小时',
      modCount: 0,
      bannerImage: 'https://images.unsplash.com/photo-1542751371-adc38448a05e?w=800&auto=format&fit=crop&q=80',
      description: `基于 Minecraft ${version.value} (${loader.value}) 的自定义实例。`,
      isFavorite: false,
      javaVersion: version.value.startsWith('1.21') ? 'Java 21' : 'Java 17',
      memoryMin: 2,
      memoryMax: 8,
    };

    emit('create-instance', newInst);
    isCreating.value = false;
    emit('close');
  }, 800);
}
</script>

<template>
  <div
    v-if="open"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm select-none"
    @click.self="emit('close')"
  >
    <div class="relative w-full max-w-md bg-[#262729] border border-[#3b3c3e] rounded-lg shadow-2xl overflow-hidden flex flex-col">
      <div class="flex items-center justify-between px-4 py-3 bg-[#1e1f20] border-b border-[#353638]">
        <div class="flex items-center gap-2">
          <Layers class="w-4 h-4 text-emerald-400" />
          <span class="font-bold text-sm text-white">创建新 Minecraft 实例 (Create Instance)</span>
        </div>
        <button class="text-slate-400 hover:text-white p-1 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4" />
        </button>
      </div>

      <form class="p-4 space-y-3" @submit.prevent="handleSubmit">
        <div class="space-y-1">
          <label class="text-xs font-bold text-white block">实例名称</label>
          <input
            v-model="name"
            type="text"
            required
            class="w-full bg-[#1b1c1d] border border-[#3e3f41] rounded p-2 text-xs text-white focus:outline-none focus:border-emerald-500"
          />
        </div>

        <div class="space-y-1">
          <label class="text-xs font-bold text-white block">游戏版本</label>
          <select
            v-model="version"
            class="w-full bg-[#1b1c1d] border border-[#3e3f41] rounded p-2 text-xs text-white focus:outline-none"
          >
            <option v-for="v in AVAILABLE_MC_VERSIONS" :key="v.version" :value="v.version">
              Minecraft {{ v.version }} ({{ v.type }})
            </option>
          </select>
        </div>

        <div class="space-y-1">
          <label class="text-xs font-bold text-white block">模组加载器 (Mod Loader)</label>
          <div class="grid grid-cols-2 gap-2">
            <button
              v-for="ld in loaders"
              :key="ld"
              type="button"
              class="p-2 rounded text-xs font-bold border transition-all cursor-pointer"
              :class="loader === ld
                ? 'bg-[#3c8527] border-[#52a535] text-white'
                : 'bg-[#1e1f20] border-[#38393b] text-slate-300 hover:bg-[#28292b]'"
              @click="loader = ld"
            >
              {{ ld }}
            </button>
          </div>
        </div>

        <div class="flex gap-2 pt-3 border-t border-[#353638]">
          <BedrockButton type="button" variant="grey" size="sm" class="flex-1" @click="emit('close')">
            取消
          </BedrockButton>
          <BedrockButton type="submit" variant="green" size="sm" class="flex-1" :disabled="isCreating">
            {{ isCreating ? '创建中...' : '确认创建' }}
          </BedrockButton>
        </div>
      </form>
    </div>
  </div>
</template>
