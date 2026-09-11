<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref, useTemplateRef } from 'vue';
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
import EditInstanceModal from './components/EditInstanceModal.vue';
import ImportInstanceModal from './components/ImportInstanceModal.vue';
import AccountModal from './components/AccountModal.vue';
import {
  INITIAL_INSTANCES,
  INITIAL_ACCOUNTS,
  MOCK_PLUGINS,
  MOCK_MODS,
  DEFAULT_SETTINGS,
} from './data/mockData';
import { bridgeRequest, parseSnapshot } from './bridge';
import {
  auraCoreAddOfflineAccount,
  auraCoreBeginMsaLogin,
  auraCoreCreateInstance,
  auraCoreDeleteInstance,
  auraCoreImportInstance,
  auraCoreListAccounts,
  auraCoreListInstances,
  auraCoreMigrate,
  auraCoreMsaInfo,
  auraCoreRemoveAccount,
  auraCoreRenameInstance,
  auraCoreSetInstanceGroup,
  auraCoreSetInstanceIcon,
  auraCoreSetDefaultAccount,
  auraCoreStatus,
  auraCoreTaskStatus,
  launchInstance,
} from './bridge';
import type {
  AuraCoreAccount,
  AuraCoreInstance,
  AuraCoreStatus,
  AuraCoreTaskStatus,
  PluginContribution,
} from './bridge';
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
const auraCoreEngineStatus = ref<AuraCoreStatus | null>(null);
const isMigratingAuraCore = ref(false);
const launchTaskStatusState = ref<AuraCoreTaskStatus | null>(null);
const msaLoginState = ref<{ active: boolean; verificationUrl?: string; userCode?: string; message: string } | null>(null);
let launchTaskTimer: number | undefined;
let msaLoginTimer: number | undefined;

const isLaunching = ref(false);
const isLaunchModalOpen = ref(false);
const isNewInstanceModalOpen = ref(false);
const isEditInstanceModalOpen = ref(false);
const editingInstance = ref<MinecraftInstance | null>(null);
const isImportModalOpen = ref(false);
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
  if (isTauri.value) {
    void launchInstance(target.id)
      .then((taskId) => {
        if (taskId !== null) startLaunchTaskPolling(taskId);
      })
      .catch((error) => {
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

function startLaunchTaskPolling(taskId: string) {
  stopLaunchTaskPolling();
  launchTaskStatusState.value = null;
  launchTaskTimer = window.setInterval(async () => {
    try {
      const status = await auraCoreTaskStatus(taskId);
      launchTaskStatusState.value = status;
      if (status.state !== 'running') {
        stopLaunchTaskPolling();
        if (status.state === 'succeeded') {
          showToast('AuraCore 启动任务完成，游戏进程运行中');
        } else {
          showToast(`AuraCore 启动任务${status.state === 'aborted' ? '已中止' : '失败'}: ${status.error ?? '未知错误'}`);
        }
        window.setTimeout(() => {
          launchTaskStatusState.value = null;
        }, 4200);
      }
    } catch (error) {
      stopLaunchTaskPolling();
      showToast(`AuraCore 任务查询失败: ${String(error)}`);
    }
  }, 1200);
}

function stopLaunchTaskPolling() {
  if (launchTaskTimer !== undefined) {
    window.clearInterval(launchTaskTimer);
    launchTaskTimer = undefined;
  }
}

function updateSettings(patch: Partial<LauncherSettings>) {
  settings.value = { ...settings.value, ...patch };
  if (!isTauri.value) return;
  if (patch.selectedUiFrontend !== undefined) {
    void bridgeRequest('core.settings.set', { key: 'uiFrontend', value: patch.selectedUiFrontend })
      .then(() => showToast('界面切换已保存，重启启动器后生效'))
      .catch((error) => showToast(`界面切换保存失败: ${String(error)}`));
  }
  if (patch.coreEngine !== undefined) {
    void bridgeRequest('core.settings.set', { key: 'coreEngine', value: patch.coreEngine })
      .then(() => {
        showToast(
          patch.coreEngine === 'auracore'
            ? '已切换为 AuraCore 原生核心，重启启动器后生效'
            : '已切换为 HMCL Java 核心，重启启动器后生效',
        );
        void refreshAuraCoreStatus();
        if (patch.coreEngine === 'auracore') {
          void refreshAuraCoreData();
          void handleMigrateAuraCore();
        }
      })
      .catch((error) => showToast(`启动器核心切换失败: ${String(error)}`));
  }
}

function toggleColorMode() {
  settings.value = {
    ...settings.value,
    colorMode: settings.value.colorMode === 'dark' ? 'light' : 'dark',
  };
}

function createInstance(instance: MinecraftInstance) {
  if (auraCoreActive.value) {
    showToast(`正在通过 AuraCore 创建实例: ${instance.name}...`);
    void auraCoreCreateInstance(instance.name, instance.version)
      .then((taskId) => void trackAuraCoreTask(taskId, `实例创建完成: ${instance.name}`))
      .catch((error) => showToast(`AuraCore 实例创建失败: ${String(error)}`));
    return;
  }
  instances.value = [...instances.value, instance];
  currentInstance.value = instance;
  showToast(`已创建实例: ${instance.name}`);
}

function deleteInstance(id: string) {
  const target = instances.value.find((i) => i.id === id);
  if (auraCoreActive.value && target) {
    void auraCoreDeleteInstance(id)
      .then(async () => {
        showToast(`已删除 AuraCore 实例: ${target.name}`);
        await refreshAuraCoreData();
      })
      .catch((error) => showToast(`AuraCore 实例删除失败: ${String(error)}`));
    return;
  }
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

function importInstance(payload: { source: string; name: string; group: string }) {
  if (!auraCoreActive.value) {
    showToast('实例导入需要切换到 AuraCore 原生核心');
    return;
  }
  showToast(`正在通过 AuraCore 导入实例: ${payload.name}...`);
  void auraCoreImportInstance(payload.source, payload.name, payload.group.length > 0 ? payload.group : undefined)
    .then((taskId) => void trackAuraCoreTask(taskId, `实例导入完成: ${payload.name}`))
    .catch((error) => showToast(`AuraCore 实例导入失败: ${String(error)}`));
}

function openEditModal(instance: MinecraftInstance) {
  editingInstance.value = instance;
  isEditInstanceModalOpen.value = true;
}

function saveInstanceEdit(payload: { instance: MinecraftInstance; name: string; group: string; icon: string }) {
  const { instance, name, group, icon } = payload;
  if (auraCoreActive.value) {
    const operations: Promise<void>[] = [];
    if (name !== instance.name) operations.push(auraCoreRenameInstance(instance.id, name));
    if ((instance.group ?? '') !== group) operations.push(auraCoreSetInstanceGroup(instance.id, group));
    if (icon !== instance.icon) operations.push(auraCoreSetInstanceIcon(instance.id, icon));
    if (operations.length === 0) return;
    void Promise.all(operations)
      .then(async () => {
        showToast(`已保存 AuraCore 实例设置: ${name}`);
        await refreshAuraCoreData();
      })
      .catch((error) => showToast(`保存 AuraCore 实例失败: ${String(error)}`));
    return;
  }
  instances.value = instances.value.map((entry) =>
    entry.id === instance.id ? { ...entry, name, group: group.length > 0 ? group : undefined, icon } : entry,
  );
  if (currentInstance.value.id === instance.id) {
    currentInstance.value = instances.value.find((entry) => entry.id === instance.id) ?? currentInstance.value;
  }
  showToast(`已保存实例设置: ${name}`);
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
  if (auraCoreActive.value) {
    void auraCoreSetDefaultAccount(account.username)
      .then(() => {
        accounts.value = accounts.value.map((a) => ({ ...a, isActive: a.id === account.id }));
        showToast(`已切换 AuraCore 默认账户: ${account.username}`);
      })
      .catch((error) => showToast(`切换 AuraCore 账户失败: ${String(error)}`));
    return;
  }
  accounts.value = accounts.value.map((a) => ({ ...a, isActive: a.id === account.id }));
  showToast(`已切换账户: ${account.username}`);
}

function addAccount(account: Account) {
  if (auraCoreActive.value) {
    if (account.type !== 'offline') {
      showToast('AuraCore 原生核心暂仅支持离线与微软账户');
      return;
    }
    void auraCoreAddOfflineAccount(account.username)
      .then(async () => {
        showToast(`已添加 AuraCore 离线账户: ${account.username}`);
        await refreshAuraCoreData();
      })
      .catch((error) => showToast(`添加 AuraCore 账户失败: ${String(error)}`));
    return;
  }
  accounts.value = accounts.value.map((a) => ({ ...a, isActive: false }));
  accounts.value = [...accounts.value, account];
  showToast(`已添加账户: ${account.username}`);
}

function deleteAccount(id: string) {
  const target = accounts.value.find((a) => a.id === id);
  if (auraCoreActive.value && target) {
    void auraCoreRemoveAccount(target.username)
      .then(async () => {
        showToast(`已移除 AuraCore 账户: ${target.username}`);
        await refreshAuraCoreData();
      })
      .catch((error) => showToast(`移除 AuraCore 账户失败: ${String(error)}`));
    return;
  }
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

const isTauri = ref(false);

const navTabs: NavTab[] = ['home', 'instances', 'mods', 'download', 'plugins', 'settings', 'multiplayer', 'console'];

const auraCoreActive = computed(() => isTauri.value && settings.value.coreEngine === 'auracore');

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

function hydrateAuraCoreInstances(raw: AuraCoreInstance[]): MinecraftInstance[] {
  return raw
    .filter((item) => typeof item?.id === 'string' && item.id.length > 0)
    .map((item) => ({
      id: item.id,
      name: typeof item.name === 'string' && item.name.length > 0 ? item.name : item.id,
      version: item.gameVersion ?? '未知版本',
      group: item.group,
      loader: (['Vanilla', 'Fabric', 'Forge', 'NeoForge', 'Quilt'] as const).includes(item.loader as never)
        ? (item.loader as MinecraftInstance['loader'])
        : 'Vanilla',
      loaderVersion: item.loaderVersion === undefined || item.loaderVersion.length === 0 ? undefined : item.loaderVersion,
      icon: typeof item.icon === 'string' && item.icon.length > 0 ? item.icon : '⛏️',
      lastPlayed: item.lastLaunch && item.lastLaunch > 0 ? new Date(item.lastLaunch).toLocaleString() : '从未',
      playTime: '—',
      modCount: Number.isFinite(item.modCount) ? Number(item.modCount) : 0,
      description: item.group ? `AuraCore 分组: ${item.group}` : '由 AuraCore 原生核心管理的实例。',
      isFavorite: false,
      javaVersion: '自动选择',
      memoryMin: 2,
      memoryMax: 4,
    }));
}

function auraCoreAccountType(type: string): Account['type'] {
  const value = type.toLowerCase();
  if (value.includes('msa') || value.includes('microsoft')) return 'microsoft';
  if (value.includes('offline')) return 'offline';
  return 'thirdparty';
}

function hydrateAuraCoreAccounts(raw: AuraCoreAccount[]): Account[] {
  const mapped = raw.map((item) => ({
    id: item.internalId || item.profileName,
    username: item.profileName,
    uuid: item.internalId || item.profileName,
    type: auraCoreAccountType(item.type),
    skinUrl: `https://minotar.net/helm/${encodeURIComponent(item.profileName)}/128.png`,
    isActive: item.isDefault === true,
  }));
  if (mapped.length > 0 && !mapped.some((account) => account.isActive)) {
    mapped[0] = { ...mapped[0], isActive: true };
  }
  return mapped;
}

async function refreshAuraCoreStatus() {
  try {
    auraCoreEngineStatus.value = await auraCoreStatus();
  } catch {
    auraCoreEngineStatus.value = null;
  }
}

async function refreshAuraCoreData() {
  if (!auraCoreActive.value) return;
  try {
    const backendInstances = hydrateAuraCoreInstances(await auraCoreListInstances());
    instances.value = backendInstances;
    if (backendInstances.length > 0 && !backendInstances.some((entry) => entry.id === currentInstance.value.id)) {
      currentInstance.value = backendInstances[0];
    }
  } catch (error) {
    showToast(`读取 AuraCore 实例失败: ${String(error)}`);
  }
  try {
    accounts.value = hydrateAuraCoreAccounts(await auraCoreListAccounts());
  } catch (error) {
    showToast(`读取 AuraCore 账户失败: ${String(error)}`);
  }
}

async function trackAuraCoreTask(taskId: string, doneMessage: string) {
  for (let attempt = 0; attempt < 600; attempt++) {
    try {
      const status = await auraCoreTaskStatus(taskId);
      if (status.state === 'succeeded') {
        showToast(doneMessage);
        await refreshAuraCoreData();
        return;
      }
      if (status.state === 'failed' || status.state === 'aborted') {
        showToast(`AuraCore 任务失败: ${status.error ?? status.state}`);
        return;
      }
    } catch (error) {
      showToast(`AuraCore 任务查询失败: ${String(error)}`);
      return;
    }
    await new Promise((resolve) => window.setTimeout(resolve, 1000));
  }
}

function hydrateSettings(raw: unknown): Partial<LauncherSettings> {
  if (typeof raw !== 'object' || raw === null) return {};
  const source = raw as Record<string, unknown>;
  const patch: Partial<LauncherSettings> = {};
  if (source.colorMode === 'dark' || source.colorMode === 'light') patch.colorMode = source.colorMode;
  if (typeof source.language === 'string') patch.language = source.language as LauncherSettings['language'];
  if (typeof source.themeAuraColor === 'string') patch.themeAuraColor = source.themeAuraColor;
  if (source.uiFrontend === 'javafx' || source.uiFrontend === 'dev.aura.modern-ui') {
    patch.selectedUiFrontend = source.uiFrontend;
  }
  if (source.coreEngine === 'hmcl' || source.coreEngine === 'auracore') {
    patch.coreEngine = source.coreEngine;
  }
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
  if (settings.value.coreEngine === 'auracore') {
    await refreshAuraCoreData();
  }
}

async function runPluginContribution(contribution: PluginContribution) {
  if (!isTauri.value) {
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

function stopMsaLoginPolling() {
  if (msaLoginTimer !== undefined) {
    window.clearInterval(msaLoginTimer);
    msaLoginTimer = undefined;
  }
}

function startMicrosoftDeviceLogin() {
  if (!auraCoreActive.value) return;
  stopMsaLoginPolling();
  msaLoginState.value = { active: true, message: '正在向微软申请设备码...' };
  void auraCoreBeginMsaLogin()
    .then((taskId) => {
      msaLoginTimer = window.setInterval(async () => {
        try {
          const info = await auraCoreMsaInfo(taskId);
          if (info.codeIssued && info.userCode && !msaLoginState.value?.userCode) {
            msaLoginState.value = {
              active: true,
              verificationUrl: info.verificationUrl,
              userCode: info.userCode,
              message: '等待浏览器完成授权...',
            };
          }
          const status = await auraCoreTaskStatus(taskId);
          if (status.state === 'succeeded') {
            stopMsaLoginPolling();
            msaLoginState.value = null;
            showToast('微软账户授权成功');
            void refreshAuraCoreData();
          } else if (status.state === 'failed' || status.state === 'aborted') {
            stopMsaLoginPolling();
            msaLoginState.value = { active: false, message: `授权失败: ${status.error ?? status.state}` };
            showToast(`微软授权失败: ${status.error ?? status.state}`);
          }
        } catch (error) {
          stopMsaLoginPolling();
          msaLoginState.value = { active: false, message: `授权查询失败: ${String(error)}` };
        }
      }, 1500);
    })
    .catch((error) => {
      msaLoginState.value = { active: false, message: `无法开始微软登录: ${String(error)}` };
      showToast(`微软登录启动失败: ${String(error)}`);
    });
}

function handleMigrateAuraCore() {
  if (!isTauri.value) return;
  isMigratingAuraCore.value = true;
  void auraCoreMigrate()
    .then((outcomes) => {
      const copied = Object.values(outcomes).filter((outcome) => outcome === 'copied').length;
      const failed = Object.values(outcomes).length - copied;
      showToast(
        failed === 0
          ? `AuraCore 设置迁移完成: ${copied} 项已复制`
          : `AuraCore 迁移完成: ${copied} 项成功 / ${failed} 项失败`,
      );
    })
    .catch((error) => showToast(`AuraCore 设置迁移失败: ${String(error)}`))
    .finally(() => {
      isMigratingAuraCore.value = false;
    });
}

onUnmounted(() => {
  stopLaunchTaskPolling();
  stopMsaLoginPolling();
});

onMounted(async () => {
  animatePageSwitch();
  if (!(await waitForTauri())) return;
  isTauri.value = true;

  try {
    const { invoke } = await import('@tauri-apps/api/core');
    await invoke('notify_ready');
    await hydrateFromLauncher();
    void refreshAuraCoreStatus();
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
            :aura-core-active="auraCoreActive"
            @select-instance="currentInstance = $event"
            @delete-instance="deleteInstance"
            @edit-instance="openEditModal"
            @duplicate-instance="duplicateInstance"
            @toggle-favorite="toggleFavorite"
            @open-new-instance="isNewInstanceModalOpen = true"
            @open-import="isImportModalOpen = true"
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
            :instance-id="currentInstance.id"
            :engine-active="auraCoreActive"
            @show-toast="showToast"
          />
          <SettingsPage
            v-else-if="activeTab === 'settings'"
            :settings="settings"
            :aura-core-status="auraCoreEngineStatus"
            :is-migrating-aura-core="isMigratingAuraCore"
            @update-settings="updateSettings"
            @migrate-auracore="handleMigrateAuraCore"
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
      :task-status="launchTaskStatusState"
      @close="isLaunchModalOpen = false"
    />

    <NewInstanceModal
      :open="isNewInstanceModalOpen"
      :aura-core-active="auraCoreActive"
      @close="isNewInstanceModalOpen = false"
      @create-instance="createInstance"
    />

    <EditInstanceModal
      :open="isEditInstanceModalOpen"
      :instance="editingInstance"
      :aura-core-active="auraCoreActive"
      @close="isEditInstanceModalOpen = false"
      @save-instance="saveInstanceEdit"
    />

    <ImportInstanceModal
      :open="isImportModalOpen"
      @close="isImportModalOpen = false"
      @import-instance="importInstance"
    />

    <AccountModal
      :open="isAccountModalOpen"
      :accounts="accounts"
      :current-account="accounts.find((a) => a.isActive) ?? accounts[0]"
      :aura-core-active="auraCoreActive"
      :msa-state="msaLoginState"
      @close="isAccountModalOpen = false"
      @select-account="selectAccount"
      @add-account="addAccount"
      @delete-account="deleteAccount"
      @add-microsoft="startMicrosoftDeviceLogin"
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
