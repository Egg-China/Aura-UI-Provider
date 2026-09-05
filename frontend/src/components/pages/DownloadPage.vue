<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue';
import anime from 'animejs';
import {
  Download,
  Search,
  ChevronLeft,
  Check,
  Zap,
  Wrench,
  Sparkles,
  ArrowDownToLine,
  Cpu,
} from 'lucide-vue-next';
import BedrockButton from '../BedrockButton.vue';
import LoaderIcon from '../LoaderIcon.vue';
import { AVAILABLE_MC_VERSIONS } from '../../data/mockData';
import type { MinecraftInstance, ModLoader } from '../../types/launcher';

interface LoaderOption {
  id: ModLoader;
  name: string;
  desc: string;
  icon: string;
  versions: string[];
}

const emit = defineEmits<{
  (event: 'create-instance', instance: MinecraftInstance): void;
  (event: 'show-toast', message: string): void;
}>();

const LOADER_OPTIONS: LoaderOption[] = [
  {
    id: 'Vanilla',
    name: '纯净原版 (Vanilla)',
    desc: '官方原汁原味核心，不包含任何第三方模组加载器',
    icon: '⛏️',
    versions: ['官方核心 (Latest)'],
  },
  {
    id: 'Fabric',
    name: 'Fabric Loader',
    desc: '极速、轻量级的现代模组加载器，支持 Sodium、Iris 光影',
    icon: '⚡',
    versions: ['0.16.9 (推荐稳定版)', '0.16.8', '0.16.7', '0.16.5'],
  },
  {
    id: 'NeoForge',
    name: 'NeoForge',
    desc: '专为 1.20.2+ 深度优化的下一代现代化 Forge 架构',
    icon: '🔥',
    versions: ['20.4.80 (推荐稳定版)', '20.4.70', '20.4.50'],
  },
  {
    id: 'Forge',
    name: 'Minecraft Forge',
    desc: '成熟经典的大型科技、工业、魔法生态模组加载器',
    icon: '⚙️',
    versions: ['47.2.20 (推荐稳定版)', '47.2.0', '47.1.0'],
  },
  {
    id: 'Quilt',
    name: 'Quilt Loader',
    desc: '社区驱动的开源模组加载器，兼容绝大部分 Fabric 模组',
    icon: '🧵',
    versions: ['0.26.1 (稳定版)', '0.25.0'],
  },
];

const currentStep = ref<'versions' | 'config'>('versions');
const selectedVersion = ref('1.21.4');
const versionTypeFilter = ref<'all' | 'release' | 'snapshot'>('release');
const search = ref('');

const instanceName = ref('');
const selectedLoader = ref<ModLoader>('Fabric');
const selectedLoaderVersion = ref('0.16.9 (推荐稳定版)');
const isInstalling = ref(false);

const containerRef = ref<HTMLDivElement | null>(null);

watch(
  currentStep,
  async () => {
    await nextTick();
    if (containerRef.value) {
      anime({
        targets: containerRef.value,
        opacity: [0.6, 1],
        translateX: currentStep.value === 'config' ? [20, 0] : [-20, 0],
        duration: 220,
        easing: 'easeOutCubic',
      });
    }
  },
);

function handleSelectVersion(version: string) {
  selectedVersion.value = version;
  instanceName.value = `Minecraft ${version}`;
  if (version.startsWith('1.21') || version.startsWith('1.20')) {
    selectedLoader.value = 'Fabric';
    selectedLoaderVersion.value = '0.16.9 (推荐稳定版)';
  } else {
    selectedLoader.value = 'Forge';
    selectedLoaderVersion.value = '47.2.20 (推荐稳定版)';
  }
  currentStep.value = 'config';
}

function handleLoaderChange(loaderId: ModLoader) {
  selectedLoader.value = loaderId;
  const opt = LOADER_OPTIONS.find((l) => l.id === loaderId);
  if (opt && opt.versions.length > 0) {
    selectedLoaderVersion.value = opt.versions[0];
  }
  if (loaderId === 'Vanilla') {
    instanceName.value = `Minecraft ${selectedVersion.value} 纯净版`;
  } else {
    instanceName.value = `${selectedVersion.value} (${loaderId})`;
  }
}

function handleStartInstall() {
  isInstalling.value = true;
  emit('show-toast', `开始下载 Minecraft ${selectedVersion.value} 核心与依赖...`);

  window.setTimeout(() => {
    const isFabric = selectedLoader.value === 'Fabric';
    const isForge = selectedLoader.value === 'Forge' || selectedLoader.value === 'NeoForge';

    const newInst: MinecraftInstance = {
      id: `inst-${Date.now()}`,
      name: instanceName.value.trim() || `Minecraft ${selectedVersion.value}`,
      version: selectedVersion.value,
      loader: selectedLoader.value,
      loaderVersion:
        selectedLoader.value !== 'Vanilla' ? selectedLoaderVersion.value.split(' ')[0] : undefined,
      icon: isFabric ? '⚡' : isForge ? '⚙️' : '⛏️',
      lastPlayed: '从未',
      playTime: '0.0 小时',
      modCount: isFabric ? 2 : 0,
      bannerImage: 'https://images.unsplash.com/photo-1627856014754-2907e2355d54?w=1200&auto=format&fit=crop&q=80',
      description: `${selectedVersion.value} 游戏实例，已配置 ${selectedLoader.value} 环境。`,
      isFavorite: false,
      javaVersion: selectedVersion.value.startsWith('1.21')
        ? 'Java 21 (推荐)'
        : selectedVersion.value.startsWith('1.12')
          ? 'Java 8'
          : 'Java 17',
      memoryMin: 2,
      memoryMax: 8,
    };

    emit('create-instance', newInst);
    isInstalling.value = false;
    emit('show-toast', `Minecraft ${selectedVersion.value} 实例部署完成！已加入实例列表。`);
    currentStep.value = 'versions';
  }, 1400);
}

const filteredVersions = computed(() =>
  AVAILABLE_MC_VERSIONS.filter((v) => {
    const matchSearch = v.version.includes(search.value);
    if (versionTypeFilter.value === 'all') return matchSearch;
    if (versionTypeFilter.value === 'release') return matchSearch && v.type === 'Release';
    return matchSearch && v.type !== 'Release';
  }),
);

const loaderVersions = computed(() => LOADER_OPTIONS.find((l) => l.id === selectedLoader.value)?.versions ?? []);
</script>

<template>
  <div class="h-full flex flex-col select-none">
    <div class="border-b border-[#24262b] pb-2.5 mb-3 flex items-center justify-between shrink-0">
      <div class="flex items-center gap-2">
        <button
          v-if="currentStep === 'config'"
          class="p-1 rounded bg-[#202226] hover:bg-[#282a2e] text-slate-300 hover:text-white transition-colors cursor-pointer mr-1"
          title="返回版本列表"
          @click="currentStep = 'versions'"
        >
          <ChevronLeft class="w-4 h-4" />
        </button>
        <div>
          <h1 class="text-sm font-bold text-white flex items-center gap-2">
            <Download class="w-4 h-4 text-[#2ea44f]" />
            <span>
              {{ currentStep === 'versions' ? '下载游戏版本 (Select Version)' : `配置安装项 (Minecraft ${selectedVersion})` }}
            </span>
          </h1>
          <p class="text-[11px] text-slate-400">
            {{ currentStep === 'versions'
              ? '选择官方 Minecraft 基础核心版本'
              : '自定义实例名称、选择 Mod 加载器及其详细构建版本' }}
          </p>
        </div>
      </div>

      <div class="flex items-center gap-1.5 text-[11px] font-mono">
        <span class="px-2 py-0.5 rounded" :class="currentStep === 'versions' ? 'bg-[#2ea44f] text-white font-bold' : 'bg-[#1f2125] text-slate-400'">
          1. 选择版本
        </span>
        <span class="text-slate-600">→</span>
        <span class="px-2 py-0.5 rounded" :class="currentStep === 'config' ? 'bg-[#2ea44f] text-white font-bold' : 'bg-[#1f2125] text-slate-400'">
          2. 选择加载器与配置
        </span>
      </div>
    </div>

    <div ref="containerRef" class="flex-1 overflow-y-auto pr-1">
      <div v-if="currentStep === 'versions'" class="space-y-3">
        <div class="flex items-center gap-2">
          <div class="relative flex-1 max-w-xs">
            <Search class="w-3.5 h-3.5 text-slate-400 absolute left-2.5 top-1/2 -translate-y-1/2" />
            <input
              v-model="search"
              type="text"
              placeholder="搜索版本号 (例如: 1.21.4)..."
              class="w-full bg-[#161719] border border-[#2c2f35] rounded-md px-2 pl-7 py-1.5 text-xs text-white placeholder-slate-500 focus:outline-none focus:border-[#2ea44f]"
            />
          </div>

          <div class="flex items-center gap-1 bg-[#161719] border border-[#2c2f35] rounded-md p-0.5 text-xs">
            <button
              class="px-2.5 py-1 rounded text-[11px] font-medium transition-colors cursor-pointer"
              :class="versionTypeFilter === 'release' ? 'bg-[#2ea44f] text-white' : 'text-slate-400 hover:text-white'"
              @click="versionTypeFilter = 'release'"
            >
              正式版 (Releases)
            </button>
            <button
              class="px-2.5 py-1 rounded text-[11px] font-medium transition-colors cursor-pointer"
              :class="versionTypeFilter === 'all' ? 'bg-[#2ea44f] text-white' : 'text-slate-400 hover:text-white'"
              @click="versionTypeFilter = 'all'"
            >
              全部版本
            </button>
          </div>
        </div>

        <div class="space-y-2">
          <div
            v-for="v in filteredVersions"
            :key="v.version"
            class="mc-card p-3 rounded-md flex items-center justify-between cursor-pointer group hover:border-[#2ea44f] transition-all"
            @click="handleSelectVersion(v.version)"
          >
            <div class="flex items-center gap-3">
              <div class="w-8 h-8 rounded bg-[#161719] border border-[#2c2f35] flex items-center justify-center text-xs font-bold text-[#2ea44f] font-mono group-hover:bg-[#2ea44f] group-hover:text-white transition-colors">
                MC
              </div>
              <div>
                <div class="font-bold text-xs text-white flex items-center gap-2">
                  <span>Minecraft {{ v.version }}</span>
                  <span class="text-[10px] px-1.5 py-[2px] rounded bg-[#161719] text-slate-400 font-mono border border-[#2c2f35]">
                    {{ v.type }}
                  </span>
                </div>
                <div class="text-[10px] text-slate-400 font-mono mt-0.5">
                  发布时间: {{ v.releaseDate }} • 官方源
                </div>
              </div>
            </div>

            <div class="flex items-center gap-2">
              <span class="text-xs text-slate-400 group-hover:text-[#34b558] font-medium transition-colors">
                选择此版本 →
              </span>
            </div>
          </div>
        </div>
      </div>

      <div v-else class="space-y-4">
        <div class="mc-panel p-3.5 space-y-1.5">
          <label class="text-xs font-bold text-slate-300 flex items-center gap-1.5">
            <Wrench class="w-3.5 h-3.5 text-[#2ea44f]" />
            <span>实例自定义名称</span>
          </label>
          <input
            v-model="instanceName"
            type="text"
            :placeholder="`Minecraft ${selectedVersion}`"
            class="w-full bg-[#161719] border border-[#2c2f35] rounded-md px-3 py-1.5 text-xs text-white focus:outline-none focus:border-[#2ea44f]"
          />
        </div>

        <div class="mc-panel p-3.5 space-y-2.5">
          <label class="text-xs font-bold text-slate-300 flex items-center justify-between">
            <div class="flex items-center gap-1.5">
              <Zap class="w-3.5 h-3.5 text-amber-400" />
              <span>选择模组加载器 (Mod Loader)</span>
            </div>
            <span class="text-[10px] text-slate-400 font-mono font-normal">
              当前适配 Minecraft {{ selectedVersion }}
            </span>
          </label>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
            <div
              v-for="ld in LOADER_OPTIONS"
              :key="ld.id"
              class="p-2.5 rounded-md border cursor-pointer transition-all flex items-start gap-2.5"
              :class="selectedLoader === ld.id
                ? 'bg-[#1b261e] border-[#2ea44f] text-white shadow-sm'
                : 'bg-[#161719] border-[#2c2f35] text-slate-300 hover:bg-[#1e2024] hover:text-white'"
              @click="handleLoaderChange(ld.id)"
            >
              <div class="w-6 h-6 rounded bg-[#101113] border border-[#27292f] flex items-center justify-center shrink-0 mt-0.5">
                <LoaderIcon :loader="ld.id" class="w-4 h-4" />
              </div>
              <div class="min-w-0 flex-1">
                <div class="flex items-center justify-between">
                  <span class="font-bold text-xs truncate">{{ ld.name }}</span>
                  <Check v-if="selectedLoader === ld.id" class="w-3.5 h-3.5 text-[#2ea44f] shrink-0" />
                </div>
                <p class="text-[10px] text-slate-400 line-clamp-2 mt-0.5 leading-snug">{{ ld.desc }}</p>
              </div>
            </div>
          </div>
        </div>

        <div v-if="selectedLoader !== 'Vanilla'" class="mc-panel p-3.5 space-y-2">
          <label class="text-xs font-bold text-slate-300 flex items-center justify-between">
            <div class="flex items-center gap-1.5">
              <Sparkles class="w-3.5 h-3.5 text-cyan-400" />
              <span>{{ selectedLoader }} 详细版本选择</span>
            </div>
            <span class="text-[10px] text-slate-400 font-mono">BMCLAPI 镜像加速</span>
          </label>

          <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
            <button
              v-for="ver in loaderVersions"
              :key="ver"
              type="button"
              class="p-2 rounded border text-left text-xs font-mono transition-all cursor-pointer"
              :class="selectedLoaderVersion === ver
                ? 'bg-[#222c24] border-[#2ea44f] text-emerald-400 font-bold'
                : 'bg-[#161719] border-[#2c2f35] text-slate-300 hover:bg-[#1e2024]'"
              @click="selectedLoaderVersion = ver"
            >
              <div class="truncate">{{ ver }}</div>
            </button>
          </div>
        </div>

        <div class="p-2.5 rounded-md bg-[#161719] border border-[#2c2f35] flex items-center justify-between text-xs text-slate-400 font-mono">
          <div class="flex items-center gap-2">
            <Cpu class="w-4 h-4 text-[#2ea44f]" />
            <span>推荐 Java 环境:</span>
            <span class="text-white font-semibold">
              {{ selectedVersion.startsWith('1.21') ? 'Java 21' : selectedVersion.startsWith('1.12') ? 'Java 8' : 'Java 17' }}
            </span>
          </div>
          <div>内存分配建议: 4GB ~ 8GB</div>
        </div>

        <div class="flex items-center justify-between pt-2 border-t border-[#24262b]">
          <BedrockButton variant="grey" size="sm" :disabled="isInstalling" @click="currentStep = 'versions'">
            ← 返回重新选版本
          </BedrockButton>

          <button
            :disabled="isInstalling"
            class="flex items-center justify-center gap-2 px-6 py-2 rounded-md bg-[#2ea44f] hover:bg-[#34b558] active:bg-[#279044] text-white font-semibold text-xs transition-colors cursor-pointer disabled:opacity-50"
            @click="handleStartInstall"
          >
            <template v-if="isInstalling">
              <div class="w-3.5 h-3.5 border-2 border-white border-t-transparent rounded-full animate-spin" />
              <span>正在下载并部署实例...</span>
            </template>
            <template v-else>
              <ArrowDownToLine class="w-4 h-4" />
              <span>开始安装并创建实例</span>
            </template>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
