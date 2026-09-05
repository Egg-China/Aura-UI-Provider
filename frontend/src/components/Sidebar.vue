<script setup lang="ts">
import {
  House,
  FolderOpen,
  Blocks,
  Download,
  Puzzle,
  Settings,
  Users2,
  Terminal,
} from 'lucide-vue-next';
import type { Account, NavTab } from '../types/launcher';

defineProps<{
  activeTab: NavTab;
  isCollapsed: boolean;
  pluginCount: number;
  currentAccount: Account;
}>();

const emit = defineEmits<{
  (event: 'update:activeTab', tab: NavTab): void;
  (event: 'collapse'): void;
  (event: 'open-accounts'): void;
}>();

const gameTabs: { id: NavTab; label: string; icon: unknown }[] = [
  { id: 'home', label: '主页', icon: House },
  { id: 'instances', label: '实例列表', icon: FolderOpen },
  { id: 'mods', label: '模组管理', icon: Blocks },
];

const contentTabs: { id: NavTab; label: string; icon: unknown }[] = [
  { id: 'download', label: '下载', icon: Download },
];

const generalTabs: { id: NavTab; label: string; icon: unknown }[] = [
  { id: 'plugins', label: '启动器插件', icon: Puzzle },
  { id: 'settings', label: '设置', icon: Settings },
  { id: 'multiplayer', label: '多人联机', icon: Users2 },
  { id: 'console', label: '日志与控制台', icon: Terminal },
];
</script>

<template>
  <aside
    class="h-full flex flex-col border-r border-[#1e2024] bg-[#17181c] transition-all duration-200 select-none z-40"
    :class="isCollapsed ? 'w-[52px]' : 'w-[200px]'"
  >
    <div class="flex-1 overflow-y-auto py-2 px-1.5">
      <div v-for="section in [
        { title: '游戏', tabs: gameTabs },
        { title: '内容', tabs: contentTabs },
        { title: '通用', tabs: generalTabs },
      ]" :key="section.title" class="mb-2">
        <div v-if="!isCollapsed" class="flex items-center gap-2 mb-1.5 px-1.5">
          <span class="text-[11px] font-semibold text-slate-400 uppercase tracking-wider">{{ section.title }}</span>
          <div class="flex-1 h-[1px] bg-[#222428]" />
        </div>
        <div v-else class="w-full h-[1px] bg-[#222428] my-1.5" />

        <div class="space-y-1">
          <button
            v-for="tab in section.tabs"
            :key="tab.id"
            class="w-full flex items-center rounded-md font-medium text-xs transition-colors cursor-pointer text-left"
            :class="[
              isCollapsed ? 'justify-center h-9' : 'gap-2.5 px-2.5 py-2',
              activeTab === tab.id
                ? 'bg-[#1e2126] text-white border-l-2 border-l-[#2ea44f]'
                : 'text-slate-300 hover:bg-[#1a1c20] hover:text-white',
            ]"
            :title="tab.label"
            @click="emit('update:activeTab', tab.id)"
          >
            <component :is="tab.icon" class="w-3.5 h-3.5 text-slate-400 shrink-0" />
            <span v-if="!isCollapsed">{{ tab.label }}</span>
          </button>
        </div>
      </div>
    </div>

    <div v-if="!isCollapsed" class="pt-2 border-t border-[#1e2024] px-1.5 pb-1.5">
      <button
        class="w-full flex items-center gap-2.5 px-2 py-2 rounded-md hover:bg-[#1a1c20] transition-colors cursor-pointer text-left"
        title="账户管理"
        @click="emit('open-accounts')"
      >
        <img
          :src="currentAccount.skinUrl"
          :alt="currentAccount.username"
          class="w-7 h-7 rounded pixelated bg-black/30 border border-white/10 shrink-0"
          @error="($event.target as HTMLImageElement).style.display = 'none'"
        />
        <div class="min-w-0 flex-1">
          <div class="text-[11px] font-semibold text-slate-200 truncate">{{ currentAccount.username }}</div>
          <div class="text-[9px] text-slate-500">账户管理</div>
        </div>
      </button>
      <div class="text-[10px] text-slate-500 flex justify-between px-1 pt-1.5">
        <span>Aura Modern UI</span>
        <span>W3</span>
      </div>
    </div>
    <div v-else class="pt-2 border-t border-[#1e2024] pb-1.5 flex flex-col items-center gap-1.5">
      <button
        class="w-8 h-8 rounded-md overflow-hidden hover:ring-1 hover:ring-[#2ea44f] transition-all cursor-pointer shrink-0"
        title="账户管理"
        @click="emit('open-accounts')"
      >
        <img
          :src="currentAccount.skinUrl"
          :alt="currentAccount.username"
          class="w-full h-full pixelated bg-black/30"
          @error="($event.target as HTMLImageElement).style.display = 'none'"
        />
      </button>
      <span class="text-[10px] text-slate-500 font-mono">W3</span>
    </div>
  </aside>
</template>
