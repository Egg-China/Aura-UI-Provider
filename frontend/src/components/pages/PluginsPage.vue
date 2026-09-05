<script setup lang="ts">
import { computed, ref } from 'vue';
import {
  Puzzle,
  Search,
  ArrowUp,
  ArrowDown,
  RefreshCw,
  UploadCloud,
  FolderPlus,
  Check,
  ShoppingBag,
  Layers,
  Download,
} from 'lucide-vue-next';
import type { LauncherPlugin } from '../../types/launcher';

const props = defineProps<{
  plugins: LauncherPlugin[];
}>();

const emit = defineEmits<{
  (event: 'toggle-plugin', id: string): void;
  (event: 'install-plugin', id: string): void;
  (event: 'uninstall-plugin', id: string): void;
  (event: 'show-toast', message: string): void;
}>();

const activeSubTab = ref<'installed' | 'store'>('installed');
const search = ref('');
const selectedCategory = ref('all');
const autoCheckUpdate = ref(true);
const showUnofficial = ref(true);
const isCheckingUpdate = ref(false);

const installedPlugins = computed(() => props.plugins.filter((p) => p.installed));
const storePlugins = computed(() =>
  props.plugins.filter((p) => {
    const matchSearch =
      p.name.toLowerCase().includes(search.value.toLowerCase()) ||
      p.description.toLowerCase().includes(search.value.toLowerCase()) ||
      p.author.toLowerCase().includes(search.value.toLowerCase());
    const matchCategory = selectedCategory.value === 'all' || p.category === selectedCategory.value;
    return matchSearch && matchCategory;
  }),
);

function handleCheckUpdate() {
  isCheckingUpdate.value = true;
  window.setTimeout(() => {
    isCheckingUpdate.value = false;
    emit('show-toast', '所有启动器插件均已是最新版本！');
  }, 1000);
}

function handleImportPackage() {
  emit('show-toast', '已打开插件包导入窗口，支持 .aurax / .zip 文件');
}

function handleRemoteInstall() {
  const url = window.prompt('请输入远程插件 Manifest 地址 (URL):', 'https://plugins.aura.io/manifest/speed-boost.json');
  if (url) {
    emit('show-toast', '正在解析远程插件配置清单并开始安装...');
  }
}
</script>

<template>
  <div class="h-full flex flex-col select-none">
    <div class="border-b border-[#24262b] pb-2.5 mb-3 flex items-center justify-between shrink-0">
      <div>
        <h1 class="text-sm font-bold text-white flex items-center gap-2">
          <Puzzle class="w-4 h-4 text-[#2ea44f]" />
          <span>启动器插件管理 (Launcher Plugins)</span>
        </h1>
        <p class="text-[11px] text-slate-400">
          管理启动器功能扩展、窗口材质、联机穿透协议与个性化主题
        </p>
      </div>

      <div class="flex items-center gap-1 bg-[#161719] border border-[#2c2f35] rounded-md p-0.5 text-xs">
        <button
          class="px-3 py-1 rounded text-xs font-semibold transition-colors cursor-pointer flex items-center gap-1.5"
          :class="activeSubTab === 'installed' ? 'bg-[#2ea44f] text-white shadow-sm' : 'text-slate-400 hover:text-white'"
          @click="activeSubTab = 'installed'"
        >
          <Layers class="w-3.5 h-3.5" />
          <span>已安装插件 ({{ installedPlugins.length }})</span>
        </button>

        <button
          class="px-3 py-1 rounded text-xs font-semibold transition-colors cursor-pointer flex items-center gap-1.5"
          :class="activeSubTab === 'store' ? 'bg-[#2ea44f] text-white shadow-sm' : 'text-slate-400 hover:text-white'"
          @click="activeSubTab = 'store'"
        >
          <ShoppingBag class="w-3.5 h-3.5" />
          <span>插件商店</span>
        </button>
      </div>
    </div>

    <div class="flex-1 overflow-y-auto pr-1 space-y-4">
      <div v-if="activeSubTab === 'installed'" class="space-y-4">
        <div class="mc-panel p-3.5 space-y-3">
          <div class="flex items-center justify-between border-b border-[#292b30] pb-2">
            <span class="text-xs font-bold text-slate-200 uppercase tracking-wider">已安装插件</span>
            <span class="text-[11px] text-slate-400 font-mono">
              共 {{ installedPlugins.length }} 个扩展已激活
            </span>
          </div>

          <div class="space-y-2">
            <div
              v-for="plugin in installedPlugins"
              :key="plugin.id"
              class="p-3 rounded-md bg-[#161719] border border-[#2c2f35] hover:border-[#383c44] transition-all flex items-center justify-between gap-3"
            >
              <div class="flex items-center gap-3 min-w-0">
                <div class="w-9 h-9 rounded-lg bg-[#1f2125] border border-[#33363e] flex items-center justify-center text-base shrink-0 shadow-inner">
                  {{ plugin.icon }}
                </div>

                <div class="min-w-0">
                  <div class="flex items-center gap-2">
                    <span class="font-bold text-xs text-white truncate">{{ plugin.name }}</span>
                    <span class="text-[10px] text-emerald-400 font-mono font-semibold">{{ plugin.version }}</span>
                  </div>
                  <div class="text-[10px] text-slate-400 truncate flex items-center gap-2 mt-0.5">
                    <span>作者: {{ plugin.author }}</span>
                    <span>•</span>
                    <span class="text-slate-500 font-mono truncate">{{ plugin.sourceUrl }}</span>
                    <span>•</span>
                    <span class="font-semibold" :class="plugin.enabled ? 'text-[#34b558]' : 'text-slate-500'">
                      状态: {{ plugin.enabled ? 'Running' : 'Disabled' }}
                    </span>
                  </div>
                </div>
              </div>

              <div class="flex items-center gap-1.5 shrink-0">
                <button
                  class="p-1 rounded bg-[#202226] hover:bg-[#282a2e] text-slate-400 hover:text-white border border-[#2e3137] transition-colors cursor-pointer"
                  title="上移优先级"
                  @click="emit('show-toast', `已将 ${plugin.name} 优先级上移`)"
                >
                  <ArrowUp class="w-3.5 h-3.5" />
                </button>
                <button
                  class="p-1 rounded bg-[#202226] hover:bg-[#282a2e] text-slate-400 hover:text-white border border-[#2e3137] transition-colors cursor-pointer"
                  title="下移优先级"
                  @click="emit('show-toast', `已将 ${plugin.name} 优先级下移`)"
                >
                  <ArrowDown class="w-3.5 h-3.5" />
                </button>

                <button
                  class="px-2.5 py-1 rounded text-xs font-semibold transition-colors cursor-pointer border"
                  :class="plugin.enabled
                    ? 'bg-[#202821] text-emerald-400 border-[#2c6d44] hover:bg-[#253227]'
                    : 'bg-[#222428] text-slate-400 border-[#383b42] hover:bg-[#2b2e34]'"
                  @click="emit('toggle-plugin', plugin.id)"
                >
                  {{ plugin.enabled ? '已启用' : '已禁用' }}
                </button>

                <button
                  class="px-2 py-1 rounded bg-rose-950/40 hover:bg-rose-900/60 text-rose-300 border border-rose-900/50 text-xs font-semibold transition-colors cursor-pointer"
                  @click="emit('uninstall-plugin', plugin.id)"
                >
                  卸载
                </button>
              </div>
            </div>
          </div>
        </div>

        <div class="mc-panel p-3.5 space-y-2.5">
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-slate-200">插件更新</span>
            <span class="text-[10px] text-slate-500 font-mono">尚未检测到可用更新</span>
          </div>

          <div class="flex items-center justify-between pt-1">
            <label class="flex items-center gap-2 cursor-pointer text-xs text-slate-300">
              <input v-model="autoCheckUpdate" type="checkbox" class="accent-[#2ea44f] rounded" />
              <span>启动后自动检查插件更新</span>
            </label>

            <button
              :disabled="isCheckingUpdate"
              class="flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-[#24262b] hover:bg-[#2c2e35] text-slate-200 text-xs font-semibold border border-[#373a42] transition-colors cursor-pointer"
              @click="handleCheckUpdate"
            >
              <RefreshCw class="w-3.5 h-3.5" :class="isCheckingUpdate ? 'animate-spin text-emerald-400' : ''" />
              <span>检查更新</span>
            </button>
          </div>
        </div>

        <div class="mc-panel p-3.5 space-y-2.5">
          <div>
            <span class="text-xs font-bold text-slate-200">安装插件</span>
            <p class="text-[11px] text-slate-400 mt-0.5">
              可通过远程 manifest 安装，也可从本地 .aurax、.pclx 或 .zip 插件包导入。
            </p>
          </div>

          <div class="flex items-center gap-2 pt-1">
            <button
              class="flex items-center gap-1.5 px-3.5 py-1.5 rounded-md bg-[#24262b] hover:bg-[#2d3037] text-slate-200 text-xs font-semibold border border-[#383c44] transition-colors cursor-pointer"
              @click="handleRemoteInstall"
            >
              <UploadCloud class="w-3.5 h-3.5 text-blue-400" />
              <span>远程安装</span>
            </button>

            <button
              class="flex items-center gap-1.5 px-3.5 py-1.5 rounded-md bg-[#24262b] hover:bg-[#2d3037] text-slate-200 text-xs font-semibold border border-[#383c44] transition-colors cursor-pointer"
              @click="handleImportPackage"
            >
              <FolderPlus class="w-3.5 h-3.5 text-emerald-400" />
              <span>导入插件包</span>
            </button>
          </div>
        </div>
      </div>

      <div v-else class="space-y-3">
        <div class="space-y-2">
          <div class="flex items-center gap-2">
            <div class="relative flex-1">
              <Search class="w-3.5 h-3.5 text-slate-400 absolute left-2.5 top-1/2 -translate-y-1/2" />
              <input
                v-model="search"
                type="text"
                placeholder="搜索插件名称、作者、ID、分组或标签..."
                class="w-full bg-[#161719] border border-[#2c2f35] rounded-md px-2 pl-7 py-1.5 text-xs text-white placeholder-slate-500 focus:outline-none focus:border-[#2ea44f]"
              />
            </div>
            <button
              class="px-3 py-1.5 rounded-md bg-[#2ea44f] hover:bg-[#34b558] text-white text-xs font-semibold transition-colors cursor-pointer"
              @click="emit('show-toast', '正在从官方插件索引库同步最新列表...')"
            >
              搜索
            </button>
          </div>

          <div class="flex items-center justify-between text-xs text-slate-400 pt-1">
            <div class="flex items-center gap-2">
              <span class="text-[11px]">分类:</span>
              <select
                v-model="selectedCategory"
                class="bg-[#161719] border border-[#2c2f35] rounded px-2 py-0.5 text-xs text-white focus:outline-none cursor-pointer"
              >
                <option value="all">全部分类</option>
                <option value="system">系统拓展</option>
                <option value="theme">主题与界面</option>
                <option value="multiplayer">联机网络</option>
                <option value="tool">辅助工具</option>
              </select>
            </div>

            <label class="flex items-center gap-2 cursor-pointer text-[11px] text-slate-400 hover:text-slate-200">
              <input v-model="showUnofficial" type="checkbox" class="accent-[#2ea44f] rounded" />
              <span>显示来自非官方开发者的内容</span>
            </label>
          </div>
        </div>

        <div class="space-y-2 pt-1">
          <div v-for="plugin in storePlugins" :key="plugin.id" class="p-3 rounded-md mc-card flex items-start justify-between gap-3 group">
            <div class="flex items-start gap-3 min-w-0">
              <div class="w-10 h-10 rounded-lg bg-[#161719] border border-[#2c2f35] flex items-center justify-center text-lg shrink-0 shadow-inner group-hover:border-[#2ea44f] transition-colors">
                {{ plugin.icon }}
              </div>

              <div class="min-w-0">
                <div class="flex items-center gap-2">
                  <span class="font-bold text-xs text-white truncate">{{ plugin.name }}</span>
                  <span class="text-[10px] text-slate-400 font-mono">{{ plugin.version }}</span>
                  <span class="text-[9px] px-1.5 py-[2px] rounded bg-[#161719] border border-[#2c2f35] text-slate-400">
                    {{ plugin.category === 'theme' ? '主题' : plugin.category === 'multiplayer' ? '联机' : '系统' }}
                  </span>
                </div>

                <p class="text-[11px] text-slate-400 mt-1 line-clamp-2 leading-relaxed">{{ plugin.description }}</p>

                <div class="text-[10px] text-slate-500 font-mono mt-1.5 flex items-center gap-3">
                  <span>作者: {{ plugin.author }}</span>
                  <span>下载量: {{ plugin.downloads }}</span>
                </div>
              </div>
            </div>

            <div class="shrink-0 self-center">
              <span
                v-if="plugin.installed"
                class="flex items-center gap-1 text-xs font-semibold px-3 py-1.5 rounded-md bg-[#1d271f] text-[#34b558] border border-[#27683c]"
              >
                <Check class="w-3.5 h-3.5" /> 已安装
              </span>
              <button
                v-else
                class="flex items-center gap-1.5 px-3.5 py-1.5 rounded-md bg-[#2ea44f] hover:bg-[#34b558] text-white text-xs font-semibold shadow-sm transition-colors cursor-pointer"
                @click="emit('install-plugin', plugin.id)"
              >
                <Download class="w-3.5 h-3.5" />
                <span>一键安装</span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
