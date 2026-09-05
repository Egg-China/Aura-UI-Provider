<script setup lang="ts">
import { computed, ref } from 'vue';
import {
  Plus,
  Search,
  Play,
  Trash2,
  Copy,
  FolderOpen,
  Star,
  Layers,
  Package,
  RefreshCw,
  Folder,
  ChevronDown,
  Boxes,
  Check,
  PlusCircle,
  LayoutGrid,
  List,
} from 'lucide-vue-next';
import LoaderIcon from '../LoaderIcon.vue';
import BedrockButton from '../BedrockButton.vue';
import type { MinecraftInstance, ModLoader, NavTab } from '../../types/launcher';

interface GameDirectory {
  id: string;
  name: string;
  path: string;
}

const props = defineProps<{
  instances: MinecraftInstance[];
  currentInstance: MinecraftInstance;
}>();

const emit = defineEmits<{
  (event: 'select-instance', instance: MinecraftInstance): void;
  (event: 'delete-instance', id: string): void;
  (event: 'duplicate-instance', instance: MinecraftInstance): void;
  (event: 'toggle-favorite', id: string): void;
  (event: 'open-new-instance'): void;
  (event: 'open-folder', instance: MinecraftInstance): void;
  (event: 'launch-instance', instance: MinecraftInstance): void;
  (event: 'navigate', tab: NavTab): void;
  (event: 'show-toast', message: string): void;
}>();

const directories = ref<GameDirectory[]>([
  { id: 'default', name: '默认主目录 (.minecraft)', path: 'C:\\Users\\ACX\\AppData\\Roaming\\.minecraft' },
  { id: 'isolated', name: '独立整合包隔离目录', path: 'D:\\Minecraft\\Isolated' },
]);
const activeDirId = ref('default');
const isDirDropdownOpen = ref(false);

const searchQuery = ref('');
const selectedLoader = ref('all');
const showOnlyFavorites = ref(false);
const viewMode = ref<'grid' | 'list'>('grid');
const isRefreshing = ref(false);

const activeDir = computed(() => directories.value.find((d) => d.id === activeDirId.value));

const loaderFilters: { id: string; label: string; loader: ModLoader | null }[] = [
  { id: 'all', label: '全部', loader: null },
  { id: 'vanilla', label: '原版', loader: 'Vanilla' },
  { id: 'fabric', label: 'Fabric', loader: 'Fabric' },
  { id: 'forge', label: 'Forge', loader: 'Forge' },
  { id: 'neoforge', label: 'NeoForge', loader: 'NeoForge' },
  { id: 'quilt', label: 'Quilt', loader: 'Quilt' },
];

function handleRefresh() {
  isRefreshing.value = true;
  window.setTimeout(() => {
    isRefreshing.value = false;
    emit('show-toast', '实例库已同步刷新');
  }, 450);
}

function handleAddCustomDir() {
  const newDir: GameDirectory = {
    id: `dir-${Date.now()}`,
    name: `自定义游戏目录 ${directories.value.length + 1}`,
    path: `D:\\Minecraft\\Games\\Dir_${directories.value.length + 1}`,
  };
  directories.value = [...directories.value, newDir];
  activeDirId.value = newDir.id;
  isDirDropdownOpen.value = false;
  emit('show-toast', `已创建并切换至目录: ${newDir.name}`);
}

const filtered = computed(() =>
  props.instances.filter((inst) => {
    const matchSearch =
      inst.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      inst.version.includes(searchQuery.value) ||
      inst.loader.toLowerCase().includes(searchQuery.value.toLowerCase());
    const matchLoader =
      selectedLoader.value === 'all' || inst.loader.toLowerCase() === selectedLoader.value.toLowerCase();
    const matchFavorite = !showOnlyFavorites.value || inst.isFavorite;
    return matchSearch && matchLoader && matchFavorite;
  }),
);

function selectAndLaunch(instance: MinecraftInstance) {
  emit('select-instance', instance);
  emit('launch-instance', instance);
}
</script>

<template>
  <div class="h-full flex flex-col select-none space-y-3">
    <!-- 顶部全景头部 -->
    <div class="flex items-center justify-between border-b border-[#24262b] pb-2.5 shrink-0">
      <div class="flex items-center gap-3">
        <div>
          <div class="flex items-center gap-2">
            <h1 class="text-sm font-bold text-white tracking-wide flex items-center gap-1.5">
              <Layers class="w-4 h-4 text-[#2ea44f]" />
              <span>游戏实例库</span>
            </h1>

            <div class="relative">
              <button
                class="flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-[#1b1d22] hover:bg-[#23262c] border border-[#2d3038] text-[11px] text-slate-300 font-medium transition-colors cursor-pointer"
                @click="isDirDropdownOpen = !isDirDropdownOpen"
              >
                <Folder class="w-3 h-3 text-[#2ea44f]" />
                <span class="truncate max-w-[130px]">{{ activeDir?.name ?? '未选择目录' }}</span>
                <ChevronDown class="w-3 h-3 text-slate-400" />
              </button>

              <div
                v-if="isDirDropdownOpen"
                class="absolute left-0 top-full mt-1.5 w-80 bg-[#17181c] border border-[#2c2f35] rounded-lg shadow-xl p-1.5 z-40"
              >
                <div class="px-2 py-1 mb-1 text-[10px] font-bold text-slate-400 uppercase tracking-wider border-b border-[#24262b]">
                  切换游戏目录
                </div>
                <div
                  v-for="d in directories"
                  :key="d.id"
                  class="p-2 rounded-md flex items-center justify-between cursor-pointer transition-colors"
                  :class="d.id === activeDirId ? 'bg-[#222b24] text-white font-semibold' : 'text-slate-300 hover:bg-[#202227]'"
                  @click="activeDirId = d.id; isDirDropdownOpen = false"
                >
                  <div class="min-w-0 flex-1">
                    <div class="text-xs truncate">{{ d.name }}</div>
                    <div class="text-[10px] text-slate-500 font-mono truncate">{{ d.path }}</div>
                  </div>
                  <Check v-if="d.id === activeDirId" class="w-3.5 h-3.5 text-[#2ea44f] shrink-0" />
                </div>

                <button
                  class="w-full flex items-center gap-2 p-2 rounded-md border border-dashed border-[#343842] text-xs text-slate-400 hover:text-white hover:border-slate-400 transition-colors cursor-pointer text-left mt-1"
                  @click="handleAddCustomDir"
                >
                  <PlusCircle class="w-3.5 h-3.5 text-slate-400" />
                  <span>添加隔离游戏文件夹</span>
                </button>
              </div>
            </div>
          </div>
          <p class="text-[11px] text-slate-400 mt-0.5">
            共管理 <span class="font-mono text-slate-200 font-semibold">{{ instances.length }}</span> 个游戏实例客户端
          </p>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <BedrockButton variant="grey" size="sm" @click="emit('show-toast', '请将 .mrpack / .zip 整合包拖入窗口')">
          <Package class="w-3.5 h-3.5 mr-1 text-slate-300" />
          <span>导入整合包</span>
        </BedrockButton>

        <BedrockButton variant="green" size="sm" @click="emit('open-new-instance')">
          <Plus class="w-3.5 h-3.5 mr-1" />
          <span>新建实例</span>
        </BedrockButton>
      </div>
    </div>

    <!-- 工具栏与筛选行 -->
    <div class="flex items-center justify-between gap-2 bg-[#161719] border border-[#24262b] px-3 py-1.5 rounded-lg shrink-0 overflow-x-auto">
      <div class="relative w-44 shrink-0">
        <Search class="w-3.5 h-3.5 text-slate-400 absolute left-2.5 top-1/2 -translate-y-1/2" />
        <input
          v-model="searchQuery"
          type="text"
          placeholder="搜索版本或名称..."
          class="w-full bg-[#111214] border border-[#272a30] rounded-md px-2 pl-7 py-1 text-xs text-white placeholder-slate-500 focus:outline-none focus:border-[#2ea44f]"
        />
      </div>

      <div class="flex items-center gap-1.5 shrink-0 whitespace-nowrap">
        <button
          v-for="ld in loaderFilters"
          :key="ld.id"
          class="flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs font-medium transition-colors cursor-pointer shrink-0 whitespace-nowrap"
          :class="selectedLoader === ld.id
            ? 'bg-[#2ea44f] text-white font-semibold shadow-sm'
            : 'bg-[#121315] text-slate-400 hover:text-white hover:bg-[#1a1c1f]'"
          @click="selectedLoader = ld.id"
        >
          <LoaderIcon v-if="ld.loader" :loader="ld.loader" class="w-3.5 h-3.5 shrink-0" />
          <span class="whitespace-nowrap">{{ ld.label }}</span>
        </button>

        <button
          class="px-2.5 py-1 rounded-md text-xs font-medium transition-colors cursor-pointer flex items-center gap-1 shrink-0 whitespace-nowrap"
          :class="showOnlyFavorites
            ? 'bg-amber-500/20 border border-amber-500/40 text-amber-300 font-semibold'
            : 'bg-[#121315] text-slate-400 hover:text-white hover:bg-[#1a1c1f]'"
          @click="showOnlyFavorites = !showOnlyFavorites"
        >
          <Star class="w-3 h-3" :class="showOnlyFavorites ? 'fill-amber-300 text-amber-300' : ''" />
          <span class="whitespace-nowrap">收藏</span>
        </button>

        <button
          class="p-1.5 rounded-md bg-[#121315] text-slate-400 hover:text-white hover:bg-[#1a1c1f] transition-colors cursor-pointer shrink-0"
          title="刷新"
          @click="handleRefresh"
        >
          <RefreshCw class="w-3.5 h-3.5" :class="isRefreshing ? 'animate-spin' : ''" />
        </button>

        <div class="flex items-center bg-[#111214] border border-[#272a30] rounded-md p-0.5 shrink-0">
          <button
            class="p-1 rounded cursor-pointer"
            :class="viewMode === 'grid' ? 'bg-[#24262b] text-white' : 'text-slate-500 hover:text-white'"
            title="网格视图"
            @click="viewMode = 'grid'"
          >
            <LayoutGrid class="w-3 h-3" />
          </button>
          <button
            class="p-1 rounded cursor-pointer"
            :class="viewMode === 'list' ? 'bg-[#24262b] text-white' : 'text-slate-500 hover:text-white'"
            title="紧凑列表"
            @click="viewMode = 'list'"
          >
            <List class="w-3 h-3" />
          </button>
        </div>
      </div>
    </div>

    <!-- 实例展示区 -->
    <div class="flex-1 overflow-y-auto pr-1">
      <div v-if="filtered.length === 0" class="h-64 flex flex-col items-center justify-center text-center space-y-3">
        <div class="w-12 h-12 rounded-full bg-[#18191c] border border-[#2b2e35] flex items-center justify-center text-slate-400">
          <Boxes class="w-6 h-6" />
        </div>
        <div>
          <div class="text-sm font-bold text-white">未找到匹配的游戏实例</div>
          <p class="text-xs text-slate-400 mt-0.5">你可以调整筛选条件，或者直接创建新的游戏实例。</p>
        </div>
        <BedrockButton variant="green" size="sm" @click="emit('open-new-instance')">
          <Plus class="w-3.5 h-3.5 mr-1" />
          <span>新建 Minecraft 实例</span>
        </BedrockButton>
      </div>

      <div v-else-if="viewMode === 'grid'" class="grid grid-cols-1 md:grid-cols-2 gap-3 pb-2">
        <div
          v-for="inst in filtered"
          :key="inst.id"
          class="relative rounded-xl border p-3.5 flex flex-col justify-between cursor-pointer transition-all"
          :class="inst.id === currentInstance.id
            ? 'bg-[#18211b] border-[#2ea44f] shadow-md ring-1 ring-[#2ea44f]/30'
            : 'bg-[#141517] border-[#24262b] hover:bg-[#181a1e] hover:border-[#323640]'"
          @click="emit('select-instance', inst)"
        >
          <div>
            <div class="flex items-start gap-3">
              <div class="w-12 h-12 rounded-lg bg-[#111214] border border-[#27292f] flex items-center justify-center shrink-0 text-xl overflow-hidden shadow-inner relative">
                <img
                  :src="inst.bannerImage || 'https://images.unsplash.com/photo-1627856014754-2907e2355d54?w=400&auto=format&fit=crop&q=80'"
                  :alt="inst.name"
                  class="w-full h-full object-cover"
                />
                <div class="absolute bottom-0.5 right-0.5 bg-black/80 rounded p-0.5 border border-white/20">
                  <LoaderIcon :loader="inst.loader" class="w-3.5 h-3.5" />
                </div>
              </div>

              <div class="min-w-0 flex-1 space-y-1">
                <div class="flex items-center justify-between">
                  <h3 class="font-bold text-xs text-white truncate">{{ inst.name }}</h3>
                  <button
                    class="p-1 text-slate-400 hover:text-amber-400 transition-colors cursor-pointer"
                    title="收藏实例"
                    @click.stop="emit('toggle-favorite', inst.id)"
                  >
                    <Star class="w-3.5 h-3.5" :class="inst.isFavorite ? 'text-amber-400 fill-amber-400' : ''" />
                  </button>
                </div>

                <div class="flex items-center gap-1.5 text-[10px] font-mono">
                  <span class="inline-flex items-center gap-1 px-1.5 py-[2px] rounded bg-[#101113] border border-[#272a30] text-[#34b558] font-bold">
                    <LoaderIcon :loader="inst.loader" class="w-3 h-3" />
                    <span>{{ inst.loader }} {{ inst.version }}</span>
                  </span>
                  <span class="text-slate-400">•</span>
                  <span class="text-slate-300">{{ inst.modCount }} 模组</span>
                  <span class="text-slate-400">•</span>
                  <span class="text-slate-300">{{ inst.memoryMax }}GB RAM</span>
                </div>
              </div>
            </div>

            <p class="text-[10px] text-slate-400 line-clamp-1 mt-2 font-medium">{{ inst.description }}</p>
          </div>

          <div class="flex items-center justify-between pt-2.5 mt-3 border-t border-[#222428]">
            <div class="flex items-center gap-1">
              <button
                class="p-1.5 rounded-md bg-[#101113] hover:bg-[#202227] text-slate-400 hover:text-white border border-[#24262b] transition-colors cursor-pointer"
                title="打开目录"
                @click.stop="emit('open-folder', inst)"
              >
                <FolderOpen class="w-3.5 h-3.5" />
              </button>

              <button
                class="p-1.5 rounded-md bg-[#101113] hover:bg-[#202227] text-slate-400 hover:text-white border border-[#24262b] transition-colors cursor-pointer"
                title="克隆实例"
                @click.stop="emit('duplicate-instance', inst)"
              >
                <Copy class="w-3.5 h-3.5" />
              </button>

              <button
                class="p-1.5 rounded-md bg-[#101113] hover:bg-rose-950/40 text-slate-400 hover:text-rose-400 border border-[#24262b] transition-colors cursor-pointer"
                title="删除实例"
                @click.stop="emit('delete-instance', inst.id)"
              >
                <Trash2 class="w-3.5 h-3.5" />
              </button>
            </div>

            <BedrockButton
              :variant="inst.id === currentInstance.id ? 'green' : 'grey'"
              size="sm"
              @click.stop="inst.id === currentInstance.id ? emit('launch-instance', inst) : emit('select-instance', inst)"
            >
              <Play class="w-3 h-3 fill-current mr-1" />
              <span>{{ inst.id === currentInstance.id ? '直接启动' : '设为当前' }}</span>
            </BedrockButton>
          </div>
        </div>
      </div>

      <div v-else class="space-y-1.5 pb-2">
        <div
          v-for="inst in filtered"
          :key="inst.id"
          class="p-2.5 rounded-lg border flex items-center justify-between cursor-pointer transition-all"
          :class="inst.id === currentInstance.id
            ? 'bg-[#18211b] border-[#2ea44f]'
            : 'bg-[#141517] border-[#24262b] hover:bg-[#181a1e]'"
          @click="emit('select-instance', inst)"
        >
          <div class="flex items-center gap-3 min-w-0 flex-1">
            <div class="w-8 h-8 rounded bg-[#101113] border border-[#27292f] flex items-center justify-center shrink-0">
              <LoaderIcon :loader="inst.loader" class="w-4 h-4" />
            </div>

            <div class="min-w-0 flex-1">
              <div class="flex items-center gap-2">
                <span class="font-bold text-xs text-white truncate">{{ inst.name }}</span>
                <span class="inline-flex items-center gap-1 px-1.5 py-[2px] rounded bg-[#101113] border border-[#272a30] text-[#34b558] text-[9px] font-mono">
                  <LoaderIcon :loader="inst.loader" class="w-2.5 h-2.5" />
                  <span>{{ inst.loader }} {{ inst.version }}</span>
                </span>
              </div>
              <div class="text-[10px] text-slate-400 font-mono truncate">
                {{ inst.modCount }} 模组 • {{ inst.memoryMax }}GB RAM • 上次游玩: {{ inst.lastPlayed }}
              </div>
            </div>
          </div>

          <div class="flex items-center gap-1.5 shrink-0 ml-3">
            <button
              class="p-1 text-slate-400 hover:text-amber-400 cursor-pointer"
              @click.stop="emit('toggle-favorite', inst.id)"
            >
              <Star class="w-3.5 h-3.5" :class="inst.isFavorite ? 'text-amber-400 fill-amber-400' : ''" />
            </button>

            <button class="p-1 text-slate-400 hover:text-white cursor-pointer" @click.stop="emit('open-folder', inst)">
              <FolderOpen class="w-3.5 h-3.5" />
            </button>

            <BedrockButton
              :variant="inst.id === currentInstance.id ? 'green' : 'grey'"
              size="sm"
              @click.stop="selectAndLaunch(inst)"
            >
              <Play class="w-3 h-3 fill-current mr-1" />
              <span>{{ inst.id === currentInstance.id ? '启动' : '选择' }}</span>
            </BedrockButton>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
