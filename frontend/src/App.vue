<script setup lang="ts">
import { nextTick, onMounted, ref, useTemplateRef } from 'vue';
import anime from 'animejs';
import { Info } from 'lucide-vue-next';
import TitleBar from './components/TitleBar.vue';
import Sidebar from './components/Sidebar.vue';
import AuraBackground from './components/AuraBackground.vue';
import HomePage from './components/pages/HomePage.vue';
import InstancesPage from './components/pages/InstancesPage.vue';
import SettingsPage from './components/pages/SettingsPage.vue';
import DownloadPage from './components/pages/DownloadPage.vue';
import ModsPage from './components/pages/ModsPage.vue';
import PluginsPage from './components/pages/PluginsPage.vue';
import MultiplayerPage from './components/pages/MultiplayerPage.vue';
import ConsolePage from './components/pages/ConsolePage.vue';
import PlaceholderPage from './components/pages/PlaceholderPage.vue';
import LaunchModal from './components/LaunchModal.vue';
import NewInstanceModal from './components/NewInstanceModal.vue';
import AccountModal from './components/AccountModal.vue';
import {
  INITIAL_INSTANCES,
  INITIAL_ACCOUNTS,
  MOCK_PLUGINS,
  MOCK_MODS,
  DEFAULT_SETTINGS,
} from './data/mockData';
import { bridgeRequest, parseSnapshot } from './bridge';
import type { PluginContribution } from './bridge';
import type {
  NavTab,
  MinecraftInstance,
  Account,
  LauncherPlugin,
  ModItem,
  LauncherSettings,
} from './types/launcher';

const activeTab = ref<NavTab>('home');
const isSidebarCollapsed = ref(false);
const instances = ref<MinecraftInstance[]>(INITIAL_INSTANCES);
const currentInstance = ref<MinecraftInstance>(INITIAL_INSTANCES[0]);
const accounts = ref<Account[]>(INITIAL_ACCOUNTS);
const plugins = ref<LauncherPlugin[]>(MOCK_PLUGINS);
const pluginContributions = ref<PluginContribution[]>([]);
const mods = ref<ModItem[]>(MOCK_MODS);
const settings = ref<LauncherSettings>({ ...DEFAULT_SETTINGS });

const isLaunching = ref(false);
const isLaunchModalOpen = ref(false);
const isNewInstanceModalOpen = ref(false);
const isAccountModalOpen = ref(false);
const toastMessage = ref<string | null>(null);
const mainViewRef = useTemplateRef<HTMLDivElement>('mainView');
let toastTimer: number | undefined;

function showToast(message: string) {
  toastMessage.value = message;
  window.clearTimeout(toastTimer);
  toastTimer = window.setTimeout(() => {
    toastMessage.value = null;
  }, 2800);
}

async function animatePageSwitch() {
  await nextTick();
  if (mainViewRef.value) {
    anime({
      targets: mainViewRef.value,
      opacity: [0.7, 1],
      translateY: [4, 0],
      duration: 160,
      easing: 'easeOutQuad',
    });
  }
}

function handleLaunchGame(target: MinecraftInstance = currentInstance.value) {
  currentInstance.value = target;
  if (isTauri) {
    void bridgeRequest('core.instance.launch', { id: target.id }).catch((error) => {
      showToast(`启动器启动失败: ${String(error)}`);
    });
  }
  isLaunching.value = true;
  isLaunchModalOpen.value = true;
  showToast(`正在启动 ${target.name}...`);
  window.setTimeout(() => {
    isLaunching.value = false;
  }, 3600);
}

function updateSettings(patch: Partial<LauncherSettings>) {
  settings.value = { ...settings.value, ...patch };
}

function toggleColorMode() {
  settings.value = {
    ...settings.value,
    colorMode: settings.value.colorMode === 'dark' ? 'light' : 'dark',
  };
}

function createInstance(instance: MinecraftInstance) {
  instances.value = [...instances.value, instance];
  currentInstance.value = instance;
  showToast(`已创建实例: ${instance.name}`);
}

function deleteInstance(id: string) {
  const target = instances.value.find((i) => i.id === id);
  instances.value = instances.value.filter((i) => i.id !== id);
  if (currentInstance.value.id === id) {
    currentInstance.value = instances.value[0] ?? currentInstance.value;
  }
  showToast(target ? `已删除实例: ${target.name}` : '实例已删除');
}

function duplicateInstance(instance: MinecraftInstance) {
  const copy: MinecraftInstance = {
    ...instance,
    id: `inst-${Date.now()}`,
    name: `${instance.name} (副本)`,
    lastPlayed: '从未',
    isFavorite: false,
  };
  instances.value = [...instances.value, copy];
  showToast(`已克隆实例: ${copy.name}`);
}

function toggleFavorite(id: string) {
  instances.value = instances.value.map((i) =>
    i.id === id ? { ...i, isFavorite: !i.isFavorite } : i,
  );
  if (currentInstance.value.id === id) {
    currentInstance.value = instances.value.find((i) => i.id === id) ?? currentInstance.value;
  }
}

function openFolder(instance: MinecraftInstance) {
  showToast(`正在打开目录: ${instance.name}`);
}

function toggleMod(id: string) {
  mods.value = mods.value.map((m) => (m.id === id ? { ...m, enabled: !m.enabled } : m));
  const target = mods.value.find((m) => m.id === id);
  showToast(target ? (target.enabled ? `已激活模组: ${target.name}` : `已停用模组: ${target.name}`) : '模组状态已更新');
}

function installMod(id: string) {
  const target = mods.value.find((m) => m.id === id);
  mods.value = mods.value.map((m) => (m.id === id ? { ...m, installed: true, enabled: true } : m));
  showToast(target ? `已安装并激活模组: ${target.name}` : '模组已安装');
}

function togglePlugin(id: string) {
  plugins.value = plugins.value.map((p) => (p.id === id ? { ...p, enabled: !p.enabled } : p));
  const target = plugins.value.find((p) => p.id === id);
  showToast(target ? (target.enabled ? `已启用插件: ${target.name}` : `已禁用插件: ${target.name}`) : '插件状态已更新');
}

function installPlugin(id: string) {
  const target = plugins.value.find((p) => p.id === id);
  plugins.value = plugins.value.map((p) => (p.id === id ? { ...p, installed: true, enabled: true, status: 'Running' } : p));
  showToast(target ? `已安装插件: ${target.name}` : '插件已安装');
}

function uninstallPlugin(id: string) {
  const target = plugins.value.find((p) => p.id === id);
  plugins.value = plugins.value.map((p) =>
    p.id === id ? { ...p, installed: false, enabled: false, status: 'Disabled' } : p,
  );
  showToast(target ? `已卸载插件: ${target.name}` : '插件已卸载');
}

function openModsFolder() {
  showToast(`正在打开 mods 文件夹: ${currentInstance.value.name}`);
}

function selectAccount(account: Account) {
  accounts.value = accounts.value.map((a) => ({ ...a, isActive: a.id === account.id }));
  showToast(`已切换账户: ${account.username}`);
}

function addAccount(account: Account) {
  accounts.value = accounts.value.map((a) => ({ ...a, isActive: false }));
  accounts.value = [...accounts.value, account];
  showToast(`已添加账户: ${account.username}`);
}

function deleteAccount(id: string) {
  const remaining = accounts.value.filter((a) => a.id !== id);
  if (remaining.length > 0 && !remaining.some((a) => a.isActive)) {
    remaining[0] = { ...remaining[0], isActive: true };
  }
  accounts.value = remaining;
  showToast('账户已移除');
}

const pageTitles: Record<NavTab, string> = {
  home: '主页',
  instances: '实例列表',
  mods: '模组管理',
  download: '下载',
  plugins: '启动器插件',
  settings: '设置',
  multiplayer: '多人联机',
  console: '日志与控制台',
};

let isTauri = false;

const navTabs: NavTab[] = ['home', 'instances', 'mods', 'download', 'plugins', 'settings', 'multiplayer', 'console'];

async function waitForTauri(timeoutMilliseconds = 5000): Promise<boolean> {
  const deadline = Date.now() + timeoutMilliseconds;
  while (!('__TAURI_INTERNALS__' in (globalThis as Record<string, unknown>))) {
    if (Date.now() >= deadline) return false;
    await new Promise((resolve) => window.setTimeout(resolve, 50));
  }
  return true;
}

function hydrateInstances(raw: unknown): MinecraftInstance[] {
  if (!Array.isArray(raw)) return [];
  return raw
    .filter((item): item is Record<string, unknown> => typeof item === 'object' && item !== null)
    .map((item) => ({
      id: String(item.id ?? `inst-${Math.random().toString(36).slice(2)}`),
      name: String(item.name ?? '未命名实例'),
      version: String(item.version ?? '1.21.4'),
      loader: (['Vanilla', 'Fabric', 'Forge', 'NeoForge', 'Quilt'] as const).includes(item.loader as never)
        ? (item.loader as MinecraftInstance['loader'])
        : 'Vanilla',
      loaderVersion: item.loaderVersion === undefined ? undefined : String(item.loaderVersion),
      icon: String(item.icon ?? '⛏️'),
      lastPlayed: String(item.lastPlayed ?? '从未'),
      playTime: String(item.playTime ?? '0.0 小时'),
      modCount: Number(item.modCount ?? 0),
      bannerImage: item.bannerImage === undefined ? undefined : String(item.bannerImage),
      description: String(item.description ?? '由启动器同步的实例。'),
      isFavorite: Boolean(item.isFavorite ?? false),
      javaVersion: String(item.javaVersion ?? 'Java 21'),
      memoryMin: Number(item.memoryMin ?? 2),
      memoryMax: Number(item.memoryMax ?? 4),
    }));
}

function hydrateAccounts(raw: unknown): Account[] {
  if (!Array.isArray(raw)) return [];
  return raw
    .filter((item): item is Record<string, unknown> => typeof item === 'object' && item !== null)
    .map((item) => ({
      id: String(item.id ?? `acc-${Math.random().toString(36).slice(2)}`),
      username: String(item.username ?? '玩家'),
      uuid: String(item.uuid ?? ''),
      type: (['microsoft', 'thirdparty', 'offline'] as const).includes(item.type as never)
        ? (item.type as Account['type'])
        : 'offline',
      skinUrl: String(item.skinUrl ?? 'https://minotar.net/helm/MHF_Steve/128.png'),
      isActive: Boolean(item.isActive ?? false),
      authServer: item.authServer === undefined ? undefined : String(item.authServer),
    }));
}

function hydrateSettings(raw: unknown): Partial<LauncherSettings> {
  if (typeof raw !== 'object' || raw === null) return {};
  const source = raw as Record<string, unknown>;
  const patch: Partial<LauncherSettings> = {};
  if (source.colorMode === 'dark' || source.colorMode === 'light') patch.colorMode = source.colorMode;
  if (typeof source.language === 'string') patch.language = source.language as LauncherSettings['language'];
  if (typeof source.themeAuraColor === 'string') patch.themeAuraColor = source.themeAuraColor;
  return patch;
}

async function hydrateFromLauncher() {
  const { invoke } = await import('@tauri-apps/api/core');
  const snapshot = parseSnapshot(await invoke<string>('get_snapshot'));
  if (!snapshot) return;

  const nextInstances = hydrateInstances(snapshot.instances);
  if (nextInstances.length > 0) {
    instances.value = nextInstances;
    currentInstance.value = nextInstances[0];
  }
  const nextAccounts = hydrateAccounts(snapshot.accounts);
  if (nextAccounts.length > 0) {
    accounts.value = nextAccounts.some((account) => account.isActive)
      ? nextAccounts
      : nextAccounts.map((account, index) => ({ ...account, isActive: index === 0 }));
  }
  const settingsPatch = hydrateSettings(snapshot.settings);
  if (Object.keys(settingsPatch).length > 0) {
    settings.value = { ...settings.value, ...settingsPatch };
  }
  if (Array.isArray(snapshot.pluginContributions)) {
    pluginContributions.value = snapshot.pluginContributions.filter(
      (contribution) => contribution && typeof contribution.id === 'string' && typeof contribution.label === 'string',
    );
  }
}

async function runPluginContribution(contribution: PluginContribution) {
  if (!isTauri) {
    showToast(`插件入口（本机预览）: ${contribution.label}`);
    return;
  }
  try {
    await bridgeRequest('core.plugin.action', { id: contribution.id });
    showToast(`已执行插件入口: ${contribution.label}`);
  } catch (error) {
    showToast(`插件入口失败: ${String(error)}`);
  }
}

onMounted(async () => {
  animatePageSwitch();
  if (!(await waitForTauri())) return;
  isTauri = true;

  try {
    const { invoke } = await import('@tauri-apps/api/core');
    await invoke('notify_ready');
    await hydrateFromLauncher();
    showToast('已连接 Aura 启动器并同步状态');

    const eventTimer = window.setInterval(async () => {
      try {
        const events = await invoke<Array<{ kind: string; payload: unknown }>>('drain_events');
        for (const event of events) {
          if (event.kind === 'navigate') {
            const target = typeof event.payload === 'object' && event.payload !== null
              ? (event.payload as Record<string, unknown>).tab
              : undefined;
            if (typeof target === 'string' && navTabs.includes(target as NavTab)) {
              activeTab.value = target as NavTab;
              showToast(`已导航: ${target}`);
            } else {
              showToast(`启动器请求导航: ${JSON.stringify(event.payload)}`);
            }
          } else if (event.kind === 'notify') {
            const message = typeof event.payload === 'object' && event.payload !== null
              ? (event.payload as Record<string, unknown>).message
              : undefined;
            showToast(typeof message === 'string' ? message : `启动器通知: ${JSON.stringify(event.payload)}`);
          }
        }
      } catch {
        window.clearInterval(eventTimer);
      }
    }, 600);
  } catch (error) {
    showToast(`协议桥初始化失败: ${String(error)}`);
  }
});
</script>

<template>
  <div
    class="h-screen w-screen flex flex-col select-none overflow-hidden transition-colors"
    :class="settings.colorMode === 'light' ? 'theme-light bg-[#f1f3f5]' : 'bg-[#141518]'"
  >
    <TitleBar
      :is-sidebar-collapsed="isSidebarCollapsed"
      :color-mode="settings.colorMode"
      @toggle-sidebar="isSidebarCollapsed = !isSidebarCollapsed"
      @toggle-color-mode="toggleColorMode"
      @notify="showToast"
    />

    <div class="flex flex-1 min-h-0">
      <Sidebar
        :active-tab="activeTab"
        :is-collapsed="isSidebarCollapsed"
        :plugin-count="plugins.filter((p) => p.enabled).length"
        :current-account="accounts.find((a) => a.isActive) ?? accounts[0]"
        :contributions="pluginContributions"
        @plugin-contribution="runPluginContribution"
        @update:active-tab="activeTab = $event"
        @collapse="isSidebarCollapsed = !isSidebarCollapsed"
        @open-accounts="isAccountModalOpen = true"
      />

      <main class="relative flex-1 min-w-0 overflow-hidden">
        <AuraBackground />

        <div ref="mainView" class="relative h-full w-full">
          <HomePage
            v-if="activeTab === 'home'"
            :current-instance="currentInstance"
            :instances="instances"
            :is-launching="isLaunching"
            @select-instance="currentInstance = $event"
            @launch="handleLaunchGame()"
          />
          <InstancesPage
            v-else-if="activeTab === 'instances'"
            :instances="instances"
            :current-instance="currentInstance"
            @select-instance="currentInstance = $event"
            @delete-instance="deleteInstance"
            @duplicate-instance="duplicateInstance"
            @toggle-favorite="toggleFavorite"
            @open-new-instance="isNewInstanceModalOpen = true"
            @open-folder="openFolder"
            @launch-instance="handleLaunchGame"
            @navigate="activeTab = $event"
            @show-toast="showToast"
          />
          <DownloadPage
            v-else-if="activeTab === 'download'"
            @create-instance="createInstance"
            @show-toast="showToast"
          />
          <ModsPage
            v-else-if="activeTab === 'mods'"
            :current-instance="currentInstance"
            :mods="mods"
            @toggle-mod="toggleMod"
            @install-mod="installMod"
            @open-mods-folder="openModsFolder"
          />
          <PluginsPage
            v-else-if="activeTab === 'plugins'"
            :plugins="plugins"
            @toggle-plugin="togglePlugin"
            @install-plugin="installPlugin"
            @uninstall-plugin="uninstallPlugin"
            @show-toast="showToast"
          />
          <MultiplayerPage
            v-else-if="activeTab === 'multiplayer'"
            @launch="handleLaunchGame()"
            @show-toast="showToast"
          />
          <ConsolePage
            v-else-if="activeTab === 'console'"
            :instance-name="currentInstance.name"
            @show-toast="showToast"
          />
          <SettingsPage
            v-else-if="activeTab === 'settings'"
            :settings="settings"
            @update-settings="updateSettings"
            @show-toast="showToast"
          />
          <PlaceholderPage
            v-else
            :title="pageTitles[activeTab]"
            :active-tab="activeTab"
            @navigate="activeTab = $event"
          />
        </div>
      </main>
    </div>

    <LaunchModal
      :open="isLaunchModalOpen"
      :instance="currentInstance"
      :account="accounts.find((a) => a.isActive) ?? accounts[0]"
      @close="isLaunchModalOpen = false"
    />

    <NewInstanceModal
      :open="isNewInstanceModalOpen"
      @close="isNewInstanceModalOpen = false"
      @create-instance="createInstance"
    />

    <AccountModal
      :open="isAccountModalOpen"
      :accounts="accounts"
      :current-account="accounts.find((a) => a.isActive) ?? accounts[0]"
      @close="isAccountModalOpen = false"
      @select-account="selectAccount"
      @add-account="addAccount"
      @delete-account="deleteAccount"
    />

    <transition
      enter-active-class="transition-all duration-200"
      leave-active-class="transition-all duration-200"
    >
      <div
        v-if="toastMessage"
        class="fixed left-1/2 -translate-x-1/2 bottom-8 z-[9999] flex items-center gap-2 px-3.5 py-2 rounded-md bg-[#1e2023] border border-[#2d2f34] text-xs text-slate-200 shadow-xl"
      >
        <Info class="w-3.5 h-3.5 text-emerald-400 shrink-0" />
        <span>{{ toastMessage }}</span>
      </div>
    </transition>
  </div>
</template>
