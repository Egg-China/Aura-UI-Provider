<script setup lang="ts">
import { computed, ref } from 'vue';
import { Puzzle, Search, Plus, Minus, FolderOpen, ArrowDownToLine } from 'lucide-vue-next';
import BedrockButton from '../BedrockButton.vue';
import type { MinecraftInstance, ModItem } from '../../types/launcher';

const props = defineProps<{
  currentInstance: MinecraftInstance;
  mods: ModItem[];
}>();

const emit = defineEmits<{
  (event: 'toggle-mod', id: string): void;
  (event: 'install-mod', id: string): void;
  (event: 'open-mods-folder'): void;
}>();

const searchQuery = ref('');
const selectedCategory = ref('全部');

const categories = ['全部', '优化 / 渲染', '辅助 / 界面', '科技 / 工业', '玩法 / 冒险', '地图 / 探索'];

const activeMods = computed(() => props.mods.filter((m) => m.installed && m.enabled));
const inactiveMods = computed(() => props.mods.filter((m) => !m.installed || !m.enabled));

const filteredInactive = computed(() =>
  inactiveMods.value.filter((m) => {
    const matchSearch =
      m.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      m.summary.toLowerCase().includes(searchQuery.value.toLowerCase());
    const matchCategory = selectedCategory.value === '全部' || m.category.includes(selectedCategory.value);
    return matchSearch && matchCategory;
  }),
);
</script>

<template>
  <div class="space-y-4 select-none pb-4">
    <div class="flex items-center justify-between border-b border-[#313233] pb-3">
      <div>
        <h1 class="text-xl font-bold text-white flex items-center gap-2">
          <Puzzle class="w-5 h-5 text-emerald-400" />
          <span>模组与材质资源包管理 (Global Resources)</span>
        </h1>
        <p class="text-xs text-slate-400 mt-0.5">
          当前配置实例: {{ currentInstance.name }} ({{ currentInstance.loader }} {{ currentInstance.version }})
        </p>
      </div>

      <BedrockButton variant="grey" size="sm" @click="emit('open-mods-folder')">
        <FolderOpen class="w-3.5 h-3.5 mr-1 text-slate-300" />
        <span>打开 mods 文件夹</span>
      </BedrockButton>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 h-[calc(100vh-13rem)]">
      <div class="mc-panel rounded-lg p-3 flex flex-col justify-between overflow-hidden">
        <div class="flex items-center justify-between border-b border-[#353638] pb-2 mb-2">
          <div class="flex items-center gap-2">
            <span class="w-2.5 h-2.5 rounded-full bg-emerald-500" />
            <span class="text-xs font-bold text-white uppercase tracking-wide">
              已激活模组 (Active: {{ activeMods.length }})
            </span>
          </div>
          <span class="text-[11px] text-slate-400 font-mono">优先级: 顶部最高</span>
        </div>

        <div class="flex-1 overflow-y-auto space-y-2 pr-1">
          <div v-for="mod in activeMods" :key="mod.id" class="mc-card p-2.5 rounded flex items-center justify-between group">
            <div class="flex items-center gap-3 min-w-0">
              <div class="w-9 h-9 rounded bg-[#1c1d1e] border border-[#3e3f41] flex items-center justify-center text-lg shrink-0">
                {{ mod.iconUrl }}
              </div>
              <div class="min-w-0">
                <div class="flex items-center gap-1.5">
                  <span class="font-bold text-xs text-white truncate">{{ mod.name }}</span>
                  <span class="text-[10px] px-1 rounded bg-[#202122] text-emerald-400 font-mono border border-[#3b3c3e]">
                    {{ mod.version }}
                  </span>
                </div>
                <p class="text-[10px] text-slate-400 truncate">{{ mod.summary }}</p>
              </div>
            </div>

            <div class="flex items-center gap-1 shrink-0">
              <button
                class="p-1.5 rounded bg-[#353638] hover:bg-[#454648] text-slate-300 hover:text-white border border-[#48494b] transition-colors cursor-pointer"
                title="从已激活列表中停用"
                @click="emit('toggle-mod', mod.id)"
              >
                <Minus class="w-3.5 h-3.5" />
              </button>
            </div>
          </div>
          <div v-if="activeMods.length === 0" class="text-center py-10 text-xs text-slate-500">
            暂无已激活的模组，请从右侧可用列表中添加。
          </div>
        </div>
      </div>

      <div class="mc-panel rounded-lg p-3 flex flex-col justify-between overflow-hidden">
        <div class="space-y-2 border-b border-[#353638] pb-2.5 mb-2">
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-white uppercase tracking-wide">
              可用资源与模组库 (Available)
            </span>
            <span class="text-[10px] text-slate-400">CurseForge / Modrinth</span>
          </div>

          <div class="flex items-center gap-2">
            <div class="relative flex-1">
              <Search class="w-3.5 h-3.5 text-slate-400 absolute left-2.5 top-1/2 -translate-y-1/2" />
              <input
                v-model="searchQuery"
                type="text"
                placeholder="搜索模组..."
                class="w-full bg-[#1c1d1e] border border-[#3e3f41] rounded px-2 pl-7 py-1 text-xs text-white focus:outline-none focus:border-emerald-500"
              />
            </div>

            <select
              v-model="selectedCategory"
              class="bg-[#1c1d1e] border border-[#3e3f41] rounded px-2 py-1 text-xs text-slate-300 focus:outline-none cursor-pointer"
            >
              <option v-for="c in categories" :key="c" :value="c">{{ c }}</option>
            </select>
          </div>
        </div>

        <div class="flex-1 overflow-y-auto space-y-2 pr-1">
          <div v-for="mod in filteredInactive" :key="mod.id" class="mc-card p-2.5 rounded flex items-center justify-between">
            <div class="flex items-center gap-3 min-w-0">
              <div class="w-9 h-9 rounded bg-[#1c1d1e] border border-[#3e3f41] flex items-center justify-center text-lg shrink-0">
                {{ mod.iconUrl }}
              </div>
              <div class="min-w-0">
                <div class="flex items-center gap-1.5">
                  <span class="font-bold text-xs text-white truncate">{{ mod.name }}</span>
                  <span class="text-[10px] px-1 rounded bg-[#202122] text-slate-400 font-mono border border-[#3b3c3e]">
                    {{ mod.category }}
                  </span>
                </div>
                <p class="text-[10px] text-slate-400 truncate">{{ mod.summary }}</p>
              </div>
            </div>

            <div class="flex items-center gap-1 shrink-0">
              <BedrockButton v-if="mod.installed" variant="green" size="sm" class="text-xs" @click="emit('toggle-mod', mod.id)">
                <Plus class="w-3.5 h-3.5 mr-1" />
                <span>激活</span>
              </BedrockButton>
              <BedrockButton v-else variant="grey" size="sm" class="text-xs" @click="emit('install-mod', mod.id)">
                <ArrowDownToLine class="w-3.5 h-3.5 mr-1" />
                <span>安装</span>
              </BedrockButton>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
