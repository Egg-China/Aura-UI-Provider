<script setup lang="ts">
import { ref } from 'vue';
import {
  Settings,
  Cpu,
  Download,
  Palette,
  Info,
  RefreshCw,
  Folder,
  FolderOpen,
  FileText,
  Sliders,
  Check,
  Trash2,
  Sun,
  Moon,
  Monitor,
} from 'lucide-vue-next';
import BedrockButton from '../BedrockButton.vue';
import type { LauncherSettings } from '../../types/launcher';

type SettingSection = 'game' | 'java' | 'download' | 'general' | 'appearance' | 'about';

defineProps<{
  settings: LauncherSettings;
}>();

const emit = defineEmits<{
  (event: 'update-settings', patch: Partial<LauncherSettings>): void;
  (event: 'show-toast', message: string): void;
}>();

const activeSection = ref<SettingSection>('game');
const isScanningJava = ref(false);
const isCleaningCache = ref(false);

const sections: { id: SettingSection; label: string; icon: unknown }[] = [
  { id: 'game', label: '全局游戏设置', icon: Sliders },
  { id: 'java', label: 'Java 运行环境', icon: Cpu },
  { id: 'download', label: '下载与镜像源', icon: Download },
  { id: 'general', label: '启动器常规', icon: Settings },
  { id: 'appearance', label: '个性化与外观', icon: Palette },
  { id: 'about', label: '关于与诊断', icon: Info },
];

const gcPresets: { id: LauncherSettings['gcPreset']; name: string; desc: string }[] = [
  { id: 'G1GC', name: 'G1GC (推荐默认)', desc: '兼顾高帧率与低停顿，适合大多数现代版本' },
  { id: 'ZGC', name: 'Generational ZGC', desc: '超低毫秒级微顿，适合 Java 21 高内存整合包' },
  { id: 'Shenandoah', name: 'Shenandoah GC', desc: '超低停顿垃圾收集器，减少帧率骤降' },
  { id: 'ParallelGC', name: 'Parallel GC', desc: '追求极高吞吐量，适合老旧低配置环境' },
];

const resolutions = [
  { label: '1080P', w: 1920, h: 1080 },
  { label: '2K', w: 2560, h: 1440 },
  { label: '720P', w: 1280, h: 720 },
  { label: '854×480', w: 854, h: 480 },
];

const launcherActions: { id: LauncherSettings['launcherActionAfterLaunch']; label: string }[] = [
  { id: 'hide', label: '隐藏启动器并在游戏退出后恢复' },
  { id: 'keep', label: '保持启动器开启' },
  { id: 'close', label: '直接关闭启动器' },
  { id: 'console', label: '打开游戏实时日志控制台' },
];

const javaRuntimes = [
  { name: 'Eclipse Adoptium JDK 21.0.3 (64-Bit)', path: 'C:\\Program Files\\Eclipse Adoptium\\jdk-21.0.3.9-hotspot\\bin\\javaw.exe', rec: '1.21.x 首选' },
  { name: 'Azul Zulu JDK 17.48 (64-Bit)', path: 'C:\\Program Files\\Zulu\\zulu-17\\bin\\javaw.exe', rec: '1.20.x 首选' },
  { name: 'AdoptOpenJDK 8u382 (64-Bit)', path: 'C:\\Program Files\\Java\\jre1.8.0_382\\bin\\javaw.exe', rec: '1.12.2 / 1.7.10 首选' },
];

const downloadSources: { id: LauncherSettings['downloadSource']; name: string; desc: string }[] = [
  { id: 'bmclapi', name: 'BMCLAPI 镜像源', desc: '国内高速多线 CDN 加速' },
  { id: 'auto', name: '智能负载均衡', desc: '自动选择最快节点' },
  { id: 'official', name: 'Mojang 官方源', desc: '直连官方服务器' },
];

const addonSources: { id: LauncherSettings['defaultAddonSource']; name: string; desc: string }[] = [
  { id: 'modrinth', name: 'Modrinth (推荐)', desc: '开放现代模组平台，下载极速' },
  { id: 'curseforge', name: 'CurseForge', desc: '经典传统大型 Mod 平台' },
];

const updateChannels: { id: LauncherSettings['updateChannel']; name: string; desc: string }[] = [
  { id: 'stable', name: '正式稳定版 (Stable)', desc: '推荐大部分普通玩家使用' },
  { id: 'dev', name: '开发快照版 (Development)', desc: '抢先体验最新重构功能' },
];

const languages: { id: LauncherSettings['language']; name: string }[] = [
  { id: 'zh_CN', name: '简体中文 (zh-CN)' },
  { id: 'zh_TW', name: '繁體中文 (zh-TW)' },
  { id: 'en_US', name: 'English (en-US)' },
  { id: 'ja_JP', name: '日本語 (ja-JP)' },
  { id: 'ru_RU', name: 'Русский (ru-RU)' },
  { id: 'de_DE', name: 'Deutsch (de-DE)' },
  { id: 'fr_FR', name: 'Français (fr-FR)' },
  { id: 'es_ES', name: 'Español (es-ES)' },
];

const proxyTypes: { id: LauncherSettings['proxyType']; label: string }[] = [
  { id: 'none', label: '不使用代理' },
  { id: 'system', label: '系统代理' },
  { id: 'http', label: 'HTTP 代理' },
  { id: 'socks5', label: 'SOCKS5' },
];

const themeColors = [
  { label: '基岩翡翠绿', color: '#2ea44f' },
  { label: '极光霓虹紫', color: '#8b5cf6' },
  { label: '深海群青蓝', color: '#0284c7' },
  { label: '熔岩炽焰橙', color: '#f97316' },
];

const backgroundStyles: { id: LauncherSettings['backgroundStyle']; name: string; desc: string }[] = [
  { id: 'particles', name: '粒子动效背景', desc: 'Aura 灵动粒子 + 视差' },
  { id: 'gradient', name: '纯渐变背景', desc: '低负载渐变晕染' },
  { id: 'plain', name: '纯色极简背景', desc: '极致性能，无装饰' },
];

const uiFrontends: { id: LauncherSettings['selectedUiFrontend']; name: string; desc: string }[] = [
  { id: 'javafx', name: '内建 JavaFX 界面', desc: 'HMCL 经典界面，内建不可卸载，永远可作恢复界面' },
  { id: 'modern-ui', name: 'Aura Modern UI', desc: 'Tauri 2 + Vue 3 新界面，以 .npl UI-Provider 插件分发' },
];

const DEFAULT_JVM_ARGS =
  '-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200 -XX:+UnlockExperimentalVMOptions -XX:+DisableExplicitGC -XX:+AlwaysPreTouch';

function handleScanJava() {
  isScanningJava.value = true;
  window.setTimeout(() => {
    isScanningJava.value = false;
    emit('show-toast', '扫描完成：已检测到 3 个可用 Java 运行时环境');
  }, 800);
}

function handleCleanCache() {
  isCleaningCache.value = true;
  window.setTimeout(() => {
    isCleaningCache.value = false;
    emit('show-toast', '公共缓存清理完成，已释放 428 MB 磁盘空间');
  }, 900);
}
</script>

<template>
  <div class="h-full flex flex-col select-none">
    <div class="border-b border-[#24262b] pb-2.5 mb-3 flex items-center justify-between shrink-0">
      <div>
        <h1 class="text-sm font-bold text-white flex items-center gap-2">
          <Settings class="w-4 h-4 text-[#2ea44f]" />
          <span>启动器设置中心 (Launcher Settings)</span>
        </h1>
        <p class="text-[11px] text-slate-400">
          遵循 HMCL-CE 核心标准：管理全局 JVM 内存、Java 运行时、BMCLAPI 镜像源与常规配置
        </p>
      </div>
    </div>

    <div class="flex-1 flex gap-3 overflow-hidden">
      <div class="w-44 bg-[#141517] border border-[#24262b] rounded-lg p-1.5 space-y-1 shrink-0 overflow-y-auto">
        <button
          v-for="sec in sections"
          :key="sec.id"
          class="w-full flex items-center gap-2.5 px-3 py-2 rounded-md text-xs font-medium transition-colors text-left cursor-pointer"
          :class="activeSection === sec.id
            ? 'bg-[#2ea44f] text-white font-semibold shadow-sm'
            : 'text-slate-300 hover:bg-[#1f2125] hover:text-white'"
          @click="activeSection = sec.id"
        >
          <component :is="sec.icon" class="w-3.5 h-3.5 shrink-0" />
          <span>{{ sec.label }}</span>
        </button>
      </div>

      <div class="flex-1 bg-[#161719] border border-[#24262b] rounded-lg p-4 overflow-y-auto space-y-4">
        <!-- 1. 全局游戏设置 -->
        <div v-if="activeSection === 'game'" class="space-y-4 max-w-xl">
          <div class="mc-panel p-3.5 space-y-3">
            <div class="flex items-center justify-between">
              <span class="text-xs font-bold text-white">JVM 运行内存分配</span>
              <label class="flex items-center gap-1.5 text-xs text-slate-300 cursor-pointer">
                <input
                  type="checkbox"
                  :checked="settings.autoMemory"
                  class="accent-[#2ea44f] rounded"
                  @change="emit('update-settings', { autoMemory: ($event.target as HTMLInputElement).checked })"
                />
                <span class="text-[11px]">自动分配内存</span>
              </label>
            </div>

            <div v-if="!settings.autoMemory" class="space-y-2 pt-1">
              <div class="flex justify-between items-center text-xs">
                <span class="text-slate-300">最小内存 (Initial Heap):</span>
                <span class="font-mono font-bold text-emerald-400 bg-[#141517] px-2 py-0.5 rounded border border-[#2c2f35]">
                  {{ settings.minMemoryGB }} GB ({{ settings.minMemoryGB * 1024 }} MB)
                </span>
              </div>
              <input
                type="range"
                min="1"
                max="8"
                step="1"
                :value="settings.minMemoryGB"
                class="w-full mc-slider cursor-pointer accent-[#2ea44f]"
                @input="emit('update-settings', { minMemoryGB: Number(($event.target as HTMLInputElement).value) })"
              />

              <div class="flex justify-between items-center text-xs pt-1">
                <span class="text-slate-300">最大内存 (Max Heap):</span>
                <span class="font-mono font-bold text-emerald-400 bg-[#141517] px-2 py-0.5 rounded border border-[#2c2f35]">
                  {{ settings.maxMemoryGB }} GB ({{ settings.maxMemoryGB * 1024 }} MB)
                </span>
              </div>
              <input
                type="range"
                min="2"
                max="24"
                step="1"
                :value="settings.maxMemoryGB"
                class="w-full mc-slider cursor-pointer accent-[#2ea44f]"
                @input="emit('update-settings', { maxMemoryGB: Number(($event.target as HTMLInputElement).value) })"
              />

              <div class="flex justify-between text-[10px] text-slate-400 font-mono">
                <span>2 GB (纯净原版)</span>
                <span>4 GB</span>
                <span>8 GB (推荐中型包)</span>
                <span>16 GB (大型光影)</span>
                <span>24 GB</span>
              </div>

              <div class="mt-2 p-2 bg-[#121315] border border-[#24262b] rounded flex items-center justify-between text-[11px] text-slate-400">
                <span>系统物理总内存: 32 GB</span>
                <span class="text-slate-300 font-mono">已预分配给 MC: {{ ((settings.maxMemoryGB / 32) * 100).toFixed(0) }}%</span>
              </div>
            </div>
            <div v-else class="p-2.5 bg-[#121315] border border-[#24262b] rounded text-xs text-slate-400 leading-relaxed">
              已开启自动内存分配：启动器将根据当前电脑物理可用内存与安装的模组数量，在启动时动态调配最优内存。
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2">
            <label class="text-xs font-bold text-white block">JVM 垃圾回收器 (GC) 优化预设</label>
            <div class="grid grid-cols-2 gap-2">
              <button
                v-for="gc in gcPresets"
                :key="gc.id"
                class="p-2 rounded border text-left cursor-pointer transition-colors"
                :class="settings.gcPreset === gc.id
                  ? 'bg-[#1e2920] border-[#2ea44f] text-white'
                  : 'bg-[#121315] border-[#24262b] text-slate-300 hover:bg-[#1a1c1f]'"
                @click="emit('update-settings', { gcPreset: gc.id })"
              >
                <div class="font-semibold text-xs text-white">{{ gc.name }}</div>
                <div class="text-[10px] text-slate-400 mt-0.5 line-clamp-1">{{ gc.desc }}</div>
              </button>
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2.5">
            <div class="flex items-center justify-between">
              <span class="text-xs font-bold text-white">游戏窗口与分辨率</span>
              <label class="flex items-center gap-1.5 text-xs text-slate-300 cursor-pointer">
                <input
                  type="checkbox"
                  :checked="settings.fullscreen"
                  class="accent-[#2ea44f] rounded"
                  @change="emit('update-settings', { fullscreen: ($event.target as HTMLInputElement).checked })"
                />
                <span class="text-[11px]">全屏启动</span>
              </label>
            </div>

            <div class="grid grid-cols-4 gap-2">
              <button
                v-for="res in resolutions"
                :key="res.label"
                class="p-2 rounded border text-center text-xs font-mono transition-colors cursor-pointer"
                :class="settings.gameResolution.width === res.w && settings.gameResolution.height === res.h
                  ? 'bg-[#2ea44f] text-white font-bold'
                  : 'bg-[#121315] border-[#24262b] text-slate-300 hover:bg-[#1a1c1f]'"
                @click="emit('update-settings', { gameResolution: { width: res.w, height: res.h } })"
              >
                {{ res.label }}
              </button>
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2">
            <label class="text-xs font-bold text-white block">启动游戏后启动器动作</label>
            <div class="grid grid-cols-2 gap-2">
              <button
                v-for="act in launcherActions"
                :key="act.id"
                class="p-2 rounded border text-xs text-left cursor-pointer transition-colors"
                :class="settings.launcherActionAfterLaunch === act.id
                  ? 'bg-[#1e2920] border-[#2ea44f] text-emerald-400 font-semibold'
                  : 'bg-[#121315] border-[#24262b] text-slate-300 hover:bg-[#1a1c1f]'"
                @click="emit('update-settings', { launcherActionAfterLaunch: act.id })"
              >
                {{ act.label }}
              </button>
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2">
            <label class="text-xs font-bold text-white block">版本隔离策略 (Instance Isolation)</label>
            <div class="grid grid-cols-2 gap-2">
              <button
                class="p-2.5 rounded border text-left cursor-pointer transition-colors"
                :class="settings.versionIsolation === 'isolated'
                  ? 'bg-[#1e2920] border-[#2ea44f] text-emerald-400 font-semibold'
                  : 'bg-[#121315] border-[#24262b] text-slate-300 hover:bg-[#1a1c1f]'"
                @click="emit('update-settings', { versionIsolation: 'isolated' })"
              >
                <div class="font-semibold text-xs text-white">实例独立隔离</div>
                <div class="text-[10px] text-slate-400 mt-0.5">每个实例独立 versions/saves/mods，互不污染</div>
              </button>
              <button
                class="p-2.5 rounded border text-left cursor-pointer transition-colors"
                :class="settings.versionIsolation === 'global'
                  ? 'bg-[#1e2920] border-[#2ea44f] text-emerald-400 font-semibold'
                  : 'bg-[#121315] border-[#24262b] text-slate-300 hover:bg-[#1a1c1f]'"
                @click="emit('update-settings', { versionIsolation: 'global' })"
              >
                <div class="font-semibold text-xs text-white">全局共享目录</div>
                <div class="text-[10px] text-slate-400 mt-0.5">HMCL 经典 .minecraft 共享存档结构</div>
              </button>
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2">
            <label class="text-xs font-bold text-white block">游戏环境变量 (Environment Variables)</label>
            <textarea
              rows="2"
              placeholder="例如: GLFW_LIBRARY=D:\native\glfw3.dll"
              class="w-full bg-[#121315] border border-[#24262b] rounded p-2 text-xs font-mono text-slate-300 focus:outline-none focus:border-[#2ea44f]"
              :value="settings.gameEnvVars"
              @input="emit('update-settings', { gameEnvVars: ($event.target as HTMLTextAreaElement).value })"
            />
            <div class="flex items-center justify-between">
              <span class="text-[10px] text-slate-500 font-mono">每行一个 KEY=VALUE，注入游戏进程环境</span>
              <label class="flex items-center gap-1.5 text-xs text-slate-300 cursor-pointer">
                <input
                  type="checkbox"
                  :checked="settings.autoCrashReport"
                  class="accent-[#2ea44f] rounded"
                  @change="emit('update-settings', { autoCrashReport: ($event.target as HTMLInputElement).checked })"
                />
                <span class="text-[11px]">崩溃自动上传分析报告</span>
              </label>
            </div>
          </div>
        </div>
        <!-- 2. Java 运行环境 -->
        <div v-else-if="activeSection === 'java'" class="space-y-4 max-w-xl">
          <div class="mc-panel p-3.5 space-y-2">
            <div class="flex items-center justify-between">
              <div>
                <span class="text-xs font-bold text-white block">自动匹配最优 Java 环境</span>
                <span class="text-[10px] text-slate-400">1.20.5+ 自动匹配 Java 21；1.17+ 匹配 Java 17；1.16- 匹配 Java 8</span>
              </div>
              <div
                class="mc-toggle"
                :class="settings.javaAutoDetect ? 'active' : ''"
                @click="emit('update-settings', { javaAutoDetect: !settings.javaAutoDetect })"
              >
                <div class="mc-toggle-thumb" />
              </div>
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2.5">
            <div class="flex items-center justify-between">
              <span class="text-xs font-bold text-white">已检测到的 Java 运行时列表</span>
              <button
                :disabled="isScanningJava"
                class="flex items-center gap-1 text-xs text-[#34b558] hover:underline cursor-pointer"
                @click="handleScanJava"
              >
                <RefreshCw class="w-3 h-3" :class="isScanningJava ? 'animate-spin' : ''" />
                <span>重新扫描</span>
              </button>
            </div>

            <div class="space-y-2">
              <div
                v-for="j in javaRuntimes"
                :key="j.name"
                class="p-2.5 rounded border cursor-pointer transition-colors"
                :class="settings.javaPath === j.path
                  ? 'bg-[#1e2920] border-[#2ea44f]'
                  : 'bg-[#121315] border-[#24262b] hover:bg-[#1a1c1f]'"
                @click="emit('update-settings', { javaPath: j.path })"
              >
                <div class="flex items-center justify-between">
                  <span class="font-semibold text-xs text-white">{{ j.name }}</span>
                  <span class="text-[10px] font-mono text-emerald-400 bg-[#172218] px-1.5 py-0.5 rounded border border-[#27683c]">
                    {{ j.rec }}
                  </span>
                </div>
                <div class="text-[10px] text-slate-400 font-mono truncate mt-1">{{ j.path }}</div>
              </div>
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2.5">
            <div class="flex items-center justify-between">
              <span class="text-xs font-bold text-white">内置 Java 运行时管理器</span>
              <span class="text-[10px] text-slate-500 font-mono">W8 接入 core.java.install</span>
            </div>
            <div class="grid grid-cols-3 gap-2">
              <BedrockButton
                v-for="ver in ['Java 8', 'Java 17', 'Java 21']"
                :key="ver"
                variant="grey"
                size="sm"
                @click="emit('show-toast', `开始下载并安装 ${ver} 运行时...`)"
              >
                下载 {{ ver }}
              </BedrockButton>
            </div>
            <span class="text-[10px] text-slate-500 block">自动下载 Adoptium 发行版并注册到运行时列表，支持离线缓存复用。</span>
          </div>

          <div class="mc-panel p-3.5 space-y-2">
            <label class="text-xs font-bold text-white block">自定义 JVM 启动参数</label>
            <textarea
              rows="2"
              class="w-full bg-[#121315] border border-[#24262b] rounded p-2 text-xs font-mono text-slate-300 focus:outline-none focus:border-[#2ea44f]"
              :value="settings.jvmArgs"
              @input="emit('update-settings', { jvmArgs: ($event.target as HTMLTextAreaElement).value })"
            />
            <div class="flex justify-between items-center pt-1">
              <button
                class="text-xs text-[#34b558] hover:underline cursor-pointer"
                @click="emit('update-settings', { jvmArgs: DEFAULT_JVM_ARGS }); emit('show-toast', '已恢复默认推荐 JVM 优化参数')"
              >
                恢复推荐参数
              </button>
              <span class="text-[10px] text-slate-500 font-mono">已应用 Aikar 优化标准</span>
            </div>
          </div>
        </div>

        <!-- 3. 下载与镜像源 -->
        <div v-else-if="activeSection === 'download'" class="space-y-4 max-w-xl">
          <div class="mc-panel p-3.5 space-y-3">
            <label class="text-xs font-bold text-white block">Minecraft 核心下载源</label>
            <div class="grid grid-cols-3 gap-2">
              <button
                v-for="src in downloadSources"
                :key="src.id"
                class="p-2.5 rounded border text-left cursor-pointer transition-colors"
                :class="settings.downloadSource === src.id
                  ? 'bg-[#1e2920] border-[#2ea44f] text-emerald-400 font-semibold'
                  : 'bg-[#121315] border-[#24262b] text-slate-300 hover:bg-[#1a1c1f]'"
                @click="emit('update-settings', { downloadSource: src.id })"
              >
                <div class="font-semibold text-xs text-white">{{ src.name }}</div>
                <div class="text-[10px] text-slate-400 mt-0.5">{{ src.desc }}</div>
              </button>
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2">
            <label class="text-xs font-bold text-white block">默认模组 / 整合包分发源</label>
            <div class="grid grid-cols-2 gap-2">
              <button
                v-for="ad in addonSources"
                :key="ad.id"
                class="p-2.5 rounded border text-left cursor-pointer transition-colors"
                :class="settings.defaultAddonSource === ad.id
                  ? 'bg-[#1e2920] border-[#2ea44f] text-emerald-400 font-semibold'
                  : 'bg-[#121315] border-[#24262b] text-slate-300 hover:bg-[#1a1c1f]'"
                @click="emit('update-settings', { defaultAddonSource: ad.id })"
              >
                <div class="font-semibold text-xs text-white">{{ ad.name }}</div>
                <div class="text-[10px] text-slate-400 mt-0.5">{{ ad.desc }}</div>
              </button>
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2.5">
            <div class="flex items-center justify-between">
              <span class="text-xs font-bold text-white">下载并发线程数</span>
              <span class="font-mono text-xs text-emerald-400 font-semibold">
                {{ settings.autoDownloadThreads ? '自动并发' : `${settings.downloadThreads} 线程` }}
              </span>
            </div>

            <div class="flex items-center gap-3">
              <input
                type="range"
                min="8"
                max="128"
                step="8"
                :disabled="settings.autoDownloadThreads"
                :value="settings.downloadThreads"
                class="flex-1 mc-slider cursor-pointer accent-[#2ea44f] disabled:opacity-40"
                @input="emit('update-settings', { downloadThreads: Number(($event.target as HTMLInputElement).value) })"
              />
              <label class="flex items-center gap-1 text-xs text-slate-300 cursor-pointer shrink-0">
                <input
                  type="checkbox"
                  :checked="settings.autoDownloadThreads"
                  class="accent-[#2ea44f] rounded"
                  @change="emit('update-settings', { autoDownloadThreads: ($event.target as HTMLInputElement).checked })"
                />
                <span>自动调节</span>
              </label>
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2">
            <div class="flex items-center justify-between">
              <div>
                <span class="text-xs font-bold text-white block">下载文件完整性校验</span>
                <span class="text-[10px] text-slate-400">SHA-1 / SHA-256 校验官方清单，损坏自动重下</span>
              </div>
              <div
                class="mc-toggle"
                :class="settings.fileVerification ? 'active' : ''"
                @click="emit('update-settings', { fileVerification: !settings.fileVerification })"
              >
                <div class="mc-toggle-thumb" />
              </div>
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2">
            <div class="flex items-center justify-between">
              <span class="text-xs font-bold text-white">公共资源缓存目录</span>
              <button
                :disabled="isCleaningCache"
                class="flex items-center gap-1 text-xs text-rose-400 hover:underline cursor-pointer"
                @click="handleCleanCache"
              >
                <Trash2 class="w-3 h-3" />
                <span>{{ isCleaningCache ? '正在清理...' : '清理缓存' }}</span>
              </button>
            </div>
            <div class="flex items-center gap-2">
              <input
                type="text"
                class="flex-1 bg-[#121315] border border-[#24262b] rounded p-2 text-xs font-mono text-slate-300 focus:outline-none focus:border-[#2ea44f]"
                :value="settings.commonCacheDir"
                @input="emit('update-settings', { commonCacheDir: ($event.target as HTMLInputElement).value })"
              />
              <button
                class="p-2 rounded bg-[#1f2125] hover:bg-[#272a2f] border border-[#2e3137] text-slate-300 transition-colors cursor-pointer"
                title="打开文件夹"
                @click="emit('show-toast', `已打开缓存目录: ${settings.commonCacheDir}`)"
              >
                <FolderOpen class="w-4 h-4" />
              </button>
            </div>
          </div>
        </div>
        <!-- 4. 启动器常规 -->
        <div v-else-if="activeSection === 'general'" class="space-y-4 max-w-xl">
          <div class="mc-panel p-3.5 space-y-2">
            <span class="text-xs font-bold text-white block">游戏全局主目录 (.minecraft)</span>
            <div class="flex items-center gap-2">
              <input
                type="text"
                class="flex-1 bg-[#121315] border border-[#24262b] rounded p-2 text-xs font-mono text-slate-300 focus:outline-none focus:border-[#2ea44f]"
                :value="settings.gameDir"
                @input="emit('update-settings', { gameDir: ($event.target as HTMLInputElement).value })"
              />
              <button
                class="p-2 rounded bg-[#1f2125] hover:bg-[#272a2f] border border-[#2e3137] text-slate-300 transition-colors cursor-pointer"
                title="浏览"
                @click="emit('show-toast', `已打开游戏根目录: ${settings.gameDir}`)"
              >
                <Folder class="w-4 h-4" />
              </button>
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2.5">
            <div class="flex items-center justify-between">
              <span class="text-xs font-bold text-white">更新频道 (Update Channel)</span>
              <span class="text-[11px] font-mono text-emerald-400">当前版本: v2.4.0-next</span>
            </div>

            <div class="grid grid-cols-2 gap-2">
              <button
                v-for="ch in updateChannels"
                :key="ch.id"
                class="p-2.5 rounded border text-left cursor-pointer transition-colors"
                :class="settings.updateChannel === ch.id
                  ? 'bg-[#1e2920] border-[#2ea44f] text-emerald-400 font-semibold'
                  : 'bg-[#121315] border-[#24262b] text-slate-300 hover:bg-[#1a1c1f]'"
                @click="emit('update-settings', { updateChannel: ch.id })"
              >
                <div class="font-semibold text-xs text-white">{{ ch.name }}</div>
                <div class="text-[10px] text-slate-400 mt-0.5">{{ ch.desc }}</div>
              </button>
            </div>

            <label class="flex items-center gap-2 cursor-pointer text-xs text-slate-300 pt-1">
              <input
                type="checkbox"
                :checked="settings.autoCheckUpdate"
                class="accent-[#2ea44f] rounded"
                @change="emit('update-settings', { autoCheckUpdate: ($event.target as HTMLInputElement).checked })"
              />
              <span>启动器启动时自动检查更新</span>
            </label>
          </div>

          <div class="mc-panel p-3.5 space-y-2">
            <div class="flex items-center justify-between">
              <div>
                <span class="text-xs font-bold text-white block">界面前端 (UI Frontend)</span>
                <span class="text-[10px] text-slate-400">Aura 双 UI 架构：内建 JavaFX 永远可作恢复界面</span>
              </div>
              <Monitor class="w-4 h-4 text-[#2ea44f] shrink-0" />
            </div>
            <div class="grid grid-cols-2 gap-2">
              <button
                v-for="fe in uiFrontends"
                :key="fe.id"
                class="p-2.5 rounded border text-left cursor-pointer transition-colors"
                :class="settings.selectedUiFrontend === fe.id
                  ? 'bg-[#1e2920] border-[#2ea44f] text-emerald-400 font-semibold'
                  : 'bg-[#121315] border-[#24262b] text-slate-300 hover:bg-[#1a1c1f]'"
                @click="emit('update-settings', { selectedUiFrontend: fe.id })"
              >
                <div class="font-semibold text-xs text-white">{{ fe.name }}</div>
                <div class="text-[10px] text-slate-400 mt-0.5">{{ fe.desc }}</div>
              </button>
            </div>
            <span class="text-[10px] text-slate-500 block font-mono">重启启动器后生效；Modern UI 未安装时自动回退 JavaFX</span>
          </div>

          <div class="mc-panel p-3.5 space-y-2">
            <label class="text-xs font-bold text-white block">启动器语言 (Language)</label>
            <div class="grid grid-cols-3 gap-2">
              <button
                v-for="l in languages"
                :key="l.id"
                class="p-2 rounded border text-center text-xs font-medium transition-colors cursor-pointer"
                :class="settings.language === l.id
                  ? 'bg-[#2ea44f] text-white font-bold'
                  : 'bg-[#121315] border-[#24262b] text-slate-300 hover:bg-[#1a1c1f]'"
                @click="emit('update-settings', { language: l.id })"
              >
                {{ l.name }}
              </button>
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2.5">
            <span class="text-xs font-bold text-white block">网络代理设置</span>
            <div class="grid grid-cols-4 gap-2">
              <button
                v-for="p in proxyTypes"
                :key="p.id"
                class="p-1.5 rounded border text-center text-xs transition-colors cursor-pointer"
                :class="settings.proxyType === p.id
                  ? 'bg-[#2ea44f] text-white font-semibold'
                  : 'bg-[#121315] border-[#24262b] text-slate-300 hover:bg-[#1a1c1f]'"
                @click="emit('update-settings', { proxyType: p.id })"
              >
                {{ p.label }}
              </button>
            </div>

            <div v-if="settings.proxyType === 'http' || settings.proxyType === 'socks5'" class="grid grid-cols-2 gap-2 pt-1">
              <div class="space-y-1">
                <label class="text-[10px] text-slate-400 block">代理主机</label>
                <input
                  type="text"
                  class="w-full bg-[#121315] border border-[#24262b] rounded p-2 text-xs font-mono text-slate-300 focus:outline-none focus:border-[#2ea44f]"
                  :value="settings.proxyHost"
                  @input="emit('update-settings', { proxyHost: ($event.target as HTMLInputElement).value })"
                />
              </div>
              <div class="space-y-1">
                <label class="text-[10px] text-slate-400 block">代理端口</label>
                <input
                  type="number"
                  class="w-full bg-[#121315] border border-[#24262b] rounded p-2 text-xs font-mono text-slate-300 focus:outline-none focus:border-[#2ea44f]"
                  :value="settings.proxyPort"
                  @input="emit('update-settings', { proxyPort: Number(($event.target as HTMLInputElement).value) })"
                />
              </div>
              <div class="space-y-1">
                <label class="text-[10px] text-slate-400 block">认证用户名 (可选)</label>
                <input
                  type="text"
                  class="w-full bg-[#121315] border border-[#24262b] rounded p-2 text-xs font-mono text-slate-300 focus:outline-none focus:border-[#2ea44f]"
                  :value="settings.proxyUsername"
                  @input="emit('update-settings', { proxyUsername: ($event.target as HTMLInputElement).value })"
                />
              </div>
              <div class="space-y-1">
                <label class="text-[10px] text-slate-400 block">认证密码 (可选)</label>
                <input
                  type="password"
                  class="w-full bg-[#121315] border border-[#24262b] rounded p-2 text-xs font-mono text-slate-300 focus:outline-none focus:border-[#2ea44f]"
                  :value="settings.proxyPassword"
                  @input="emit('update-settings', { proxyPassword: ($event.target as HTMLInputElement).value })"
                />
              </div>
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2">
            <div class="flex items-center justify-between">
              <div>
                <span class="text-xs font-semibold text-white block">调试日志 (Debug Log)</span>
                <span class="text-[10px] text-slate-400">输出协议帧与插件生命周期详细日志到 stderr</span>
              </div>
              <div
                class="mc-toggle"
                :class="settings.debugLog ? 'active' : ''"
                @click="emit('update-settings', { debugLog: !settings.debugLog })"
              >
                <div class="mc-toggle-thumb" />
              </div>
            </div>
            <div class="flex items-center justify-between pt-2 border-t border-[#24262b]">
              <div>
                <span class="text-xs font-semibold text-white block">四月愚人节彩蛋 (April Fools)</span>
                <span class="text-[10px] text-slate-400">每年 4 月 1 日启用限定主题与整活动效</span>
              </div>
              <div
                class="mc-toggle"
                :class="settings.aprilFools ? 'active' : ''"
                @click="emit('update-settings', { aprilFools: !settings.aprilFools })"
              >
                <div class="mc-toggle-thumb" />
              </div>
            </div>
          </div>
        </div>
        <!-- 5. 个性化与外观 -->
        <div v-else-if="activeSection === 'appearance'" class="space-y-4 max-w-xl">
          <div class="mc-panel p-3.5 space-y-2.5">
            <span class="text-xs font-bold text-white block">界面配色明暗模式</span>
            <div class="grid grid-cols-2 gap-3">
              <div
                class="p-3 rounded-lg border cursor-pointer transition-all flex items-center gap-3"
                :class="settings.colorMode === 'dark'
                  ? 'bg-[#1b241e] border-[#2ea44f] text-white shadow-sm'
                  : 'bg-[#141517] border-[#27292f] text-slate-300 hover:bg-[#191b1f]'"
                @click="emit('update-settings', { colorMode: 'dark' })"
              >
                <div class="w-9 h-9 rounded-md bg-[#101113] border border-[#26282e] flex items-center justify-center text-slate-300 shrink-0">
                  <Moon class="w-4 h-4 text-indigo-400" />
                </div>
                <div class="min-w-0 flex-1">
                  <div class="flex items-center justify-between">
                    <span class="font-bold text-xs">深色沉浸模式</span>
                    <Check v-if="settings.colorMode === 'dark'" class="w-3.5 h-3.5 text-[#2ea44f]" />
                  </div>
                  <span class="text-[10px] text-slate-400">暗黑黑曜石基岩配色，护眼沉浸</span>
                </div>
              </div>

              <div
                class="p-3 rounded-lg border cursor-pointer transition-all flex items-center gap-3"
                :class="settings.colorMode === 'light'
                  ? 'bg-[#1b241e] border-[#2ea44f] text-white shadow-sm'
                  : 'bg-[#141517] border-[#27292f] text-slate-300 hover:bg-[#191b1f]'"
                @click="emit('update-settings', { colorMode: 'light' })"
              >
                <div class="w-9 h-9 rounded-md bg-[#ffffff] border border-[#cbd5e1] flex items-center justify-center text-amber-500 shrink-0 shadow-sm">
                  <Sun class="w-4 h-4 text-amber-500" />
                </div>
                <div class="min-w-0 flex-1">
                  <div class="flex items-center justify-between">
                    <span class="font-bold text-xs">纯白明亮模式</span>
                    <Check v-if="settings.colorMode === 'light'" class="w-3.5 h-3.5 text-[#2ea44f]" />
                  </div>
                  <span class="text-[10px] text-slate-400">现代极简亮白配色，清晰高对比</span>
                </div>
              </div>
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2.5">
            <span class="text-xs font-bold text-white block">主题基调色彩</span>
            <div class="grid grid-cols-4 gap-2">
              <button
                v-for="thm in themeColors"
                :key="thm.color"
                class="p-2 rounded-md bg-[#121315] border border-[#24262b] hover:border-[#383b42] flex items-center gap-2 cursor-pointer transition-all"
                :class="settings.themeAuraColor === thm.color ? 'border-[#2ea44f]' : ''"
                @click="emit('update-settings', { themeAuraColor: thm.color })"
              >
                <div class="w-4 h-4 rounded-full shadow shrink-0" :style="{ backgroundColor: thm.color }" />
                <span class="text-xs text-slate-200 truncate">{{ thm.label }}</span>
              </button>
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2">
            <label class="text-xs font-bold text-white block">背景主题系统</label>
            <div class="grid grid-cols-3 gap-2">
              <button
                v-for="bg in backgroundStyles"
                :key="bg.id"
                class="p-2.5 rounded border text-left cursor-pointer transition-colors"
                :class="settings.backgroundStyle === bg.id
                  ? 'bg-[#1e2920] border-[#2ea44f] text-emerald-400 font-semibold'
                  : 'bg-[#121315] border-[#24262b] text-slate-300 hover:bg-[#1a1c1f]'"
                @click="emit('update-settings', { backgroundStyle: bg.id })"
              >
                <div class="font-semibold text-xs text-white">{{ bg.name }}</div>
                <div class="text-[10px] text-slate-400 mt-0.5">{{ bg.desc }}</div>
              </button>
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2">
            <div class="flex items-center justify-between text-xs">
              <span class="text-slate-300 font-bold">界面字体缩放</span>
              <span class="font-mono font-bold text-emerald-400 bg-[#141517] px-2 py-0.5 rounded border border-[#2c2f35]">
                {{ settings.uiFontScale }}%
              </span>
            </div>
            <input
              type="range"
              min="85"
              max="125"
              step="5"
              :value="settings.uiFontScale"
              class="w-full mc-slider cursor-pointer accent-[#2ea44f]"
              @input="emit('update-settings', { uiFontScale: Number(($event.target as HTMLInputElement).value) })"
            />
            <div class="flex justify-between text-[10px] text-slate-400 font-mono">
              <span>85%</span>
              <span>100%</span>
              <span>125%</span>
            </div>
          </div>

          <div class="mc-panel p-3.5 space-y-2">
            <div class="flex items-center justify-between">
              <div>
                <span class="text-xs font-semibold text-white block">硬件加速渲染 (GPU Acceleration)</span>
                <span class="text-[10px] text-slate-400">使用独立显卡渲染 UI 动效与平滑缩放</span>
              </div>
              <div
                class="mc-toggle"
                :class="settings.hardwareAcceleration ? 'active' : ''"
                @click="emit('update-settings', { hardwareAcceleration: !settings.hardwareAcceleration })"
              >
                <div class="mc-toggle-thumb" />
              </div>
            </div>

            <div class="flex items-center justify-between pt-2 border-t border-[#24262b]">
              <div>
                <span class="text-xs font-semibold text-white block">Discord 游戏状态实时同步 (RPC)</span>
                <span class="text-[10px] text-slate-400">在个人资料中实时显示当前 Minecraft 游玩时长与实例</span>
              </div>
              <div
                class="mc-toggle"
                :class="settings.enableDiscordRPC ? 'active' : ''"
                @click="emit('update-settings', { enableDiscordRPC: !settings.enableDiscordRPC })"
              >
                <div class="mc-toggle-thumb" />
              </div>
            </div>
          </div>
        </div>

        <!-- 6. 关于与诊断 -->
        <div v-else class="space-y-3.5 max-w-xl">
          <div
            class="relative rounded-xl py-6 px-6 overflow-hidden border transition-all duration-200 flex flex-col items-center justify-center"
            :class="settings.colorMode === 'light'
              ? 'bg-gradient-to-br from-[#e0f2fe] via-[#f0fdf4] to-[#f5f3ff] border-[#e2e8f0] shadow-sm'
              : 'bg-gradient-to-br from-[#4e749c] via-[#2f4d66] to-[#2a563f] border-white/15 shadow-xl'"
          >
            <div
              class="absolute -top-12 -left-12 w-48 h-48 rounded-full blur-3xl pointer-events-none"
              :class="settings.colorMode === 'light' ? 'bg-sky-200/50' : 'bg-sky-300/20'"
            />
            <div
              class="absolute -bottom-12 -right-12 w-48 h-48 rounded-full blur-3xl pointer-events-none"
              :class="settings.colorMode === 'light' ? 'bg-emerald-200/50' : 'bg-emerald-400/20'"
            />

            <div class="relative z-10 flex items-center justify-center gap-3.5">
              <img
                src="/IMG_20260827_125438_128x128.png"
                alt="Aura Launcher"
                class="w-10 h-10 object-contain drop-shadow-md"
                @error="($event.target as HTMLImageElement).src = '/IMG_20260827_125438_128x128.svg'"
              />

              <div
                class="flex items-baseline tracking-tight select-none"
                :class="settings.colorMode === 'light' ? 'text-slate-900' : 'text-white'"
              >
                <span class="text-3xl font-bold tracking-tight">Aura</span>
                <span
                  class="text-3xl font-black ml-2.5 tracking-tight"
                  :class="settings.colorMode === 'light' ? 'text-[#16a34a]' : 'text-white/95'"
                >2.4</span>
              </div>
            </div>

            <div
              class="relative z-10 text-[11px] font-mono tracking-widest mt-1.5"
              :class="settings.colorMode === 'light' ? 'text-slate-500' : 'text-white/70'"
            >
              Build 2026.08.28 • Next Channel
            </div>
          </div>

          <div class="mc-panel p-4 space-y-2.5">
            <span class="text-xs font-bold text-white block border-b border-[#24262b] pb-2">
              系统与运行环境
            </span>
            <div class="grid grid-cols-2 gap-2 text-xs font-mono">
              <div class="p-2 rounded bg-[#121315] border border-[#24262b] text-slate-300">
                <span class="text-slate-500 text-[10px] block">操作系统 / 架构</span>
                <span class="font-semibold text-slate-200">Windows 11 (x86_64)</span>
              </div>
              <div class="p-2 rounded bg-[#121315] border border-[#24262b] text-slate-300">
                <span class="text-slate-500 text-[10px] block">渲染框架 / 动画引擎</span>
                <span class="font-semibold text-slate-200">Vue 3 + Tauri 2 + Anime.js</span>
              </div>
              <div class="p-2 rounded bg-[#121315] border border-[#24262b] text-slate-300">
                <span class="text-slate-500 text-[10px] block">核心分支</span>
                <span class="font-semibold text-emerald-400">Next (aura.ui.v1)</span>
              </div>
              <div class="p-2 rounded bg-[#121315] border border-[#24262b] text-slate-300">
                <span class="text-slate-500 text-[10px] block">推荐运行环境</span>
                <span class="font-semibold text-slate-200">Java 21 / 17 (64-Bit)</span>
              </div>
            </div>
          </div>

          <div class="mc-panel p-3.5 flex items-center justify-between">
            <div class="flex items-center gap-2">
              <button
                class="flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-[#1f2125] hover:bg-[#272a2f] border border-[#2e3137] text-slate-200 text-xs font-semibold transition-colors cursor-pointer"
                @click="emit('show-toast', '已导出诊断日志至 aura-exported-logs.zip')"
              >
                <FileText class="w-3.5 h-3.5 text-sky-400" />
                <span>导出诊断日志 (ZIP)</span>
              </button>

              <button
                class="p-1.5 rounded-md bg-[#1f2125] hover:bg-[#272a2f] border border-[#2e3137] text-slate-300 hover:text-white transition-colors cursor-pointer"
                title="打开日志文件夹"
                @click="emit('show-toast', '已打开启动器日志所在目录')"
              >
                <FolderOpen class="w-4 h-4" />
              </button>

              <button
                class="p-1.5 rounded-md bg-[#1f2125] hover:bg-[#272a2f] border border-[#2e3137] text-slate-300 hover:text-white transition-colors cursor-pointer"
                title="从 HMCL CE 导入白名单设置"
                @click="emit('show-toast', '已从 HMCL CE 导入白名单设置（不含插件与安全状态）')"
              >
                <Folder class="w-4 h-4" />
              </button>
            </div>

            <BedrockButton
              variant="green"
              size="sm"
              @click="emit('show-toast', '正在联网检查 Aura Launcher 最新更新...')"
            >
              <RefreshCw class="w-3.5 h-3.5 mr-1" />
              <span>检查启动器更新</span>
            </BedrockButton>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
