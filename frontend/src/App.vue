<script setup lang="ts">
import { nextTick, onMounted, ref, useTemplateRef } from 'vue';
import anime from 'animejs';
import { Info } from 'lucide-vue-next';
import TitleBar from './components/TitleBar.vue';
import Sidebar from './components/Sidebar.vue';
import AuraBackground from './components/AuraBackground.vue';
import HomePage from './components/pages/HomePage.vue';
import PlaceholderPage from './components/pages/PlaceholderPage.vue';
import LaunchModal from './components/LaunchModal.vue';
import {
  INITIAL_INSTANCES,
  INITIAL_ACCOUNTS,
  MOCK_PLUGINS,
  DEFAULT_SETTINGS,
} from './data/mockData';
import type {
  NavTab,
  MinecraftInstance,
  Account,
  LauncherPlugin,
  LauncherSettings,
} from './types/launcher';

const activeTab = ref<NavTab>('home');
const isSidebarCollapsed = ref(false);
const instances = ref<MinecraftInstance[]>(INITIAL_INSTANCES);
const currentInstance = ref<MinecraftInstance>(INITIAL_INSTANCES[0]);
const accounts = ref<Account[]>(INITIAL_ACCOUNTS);
const plugins = ref<LauncherPlugin[]>(MOCK_PLUGINS);
const settings = ref<LauncherSettings>({ ...DEFAULT_SETTINGS });

const isLaunching = ref(false);
const isLaunchModalOpen = ref(false);
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
  isLaunching.value = true;
  isLaunchModalOpen.value = true;
  showToast(`正在启动 ${target.name}...`);
  window.setTimeout(() => {
    isLaunching.value = false;
  }, 3600);
}

function toggleColorMode() {
  settings.value = {
    ...settings.value,
    colorMode: settings.value.colorMode === 'dark' ? 'light' : 'dark',
  };
}

const pageTitles: Record<NavTab, string> = {
  home: '主页',
  instances: '实例列表',
  download: '下载',
  plugins: '启动器插件',
  settings: '设置',
  multiplayer: '多人联机',
  console: '日志与控制台',
};

onMounted(animatePageSwitch);
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
        @update:active-tab="activeTab = $event"
        @collapse="isSidebarCollapsed = !isSidebarCollapsed"
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
