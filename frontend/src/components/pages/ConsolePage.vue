<script setup lang="ts">
import { computed, ref } from 'vue';
import { Terminal, Copy, Search, Check } from 'lucide-vue-next';
import BedrockButton from '../BedrockButton.vue';

const props = defineProps<{
  instanceName: string;
}>();

const emit = defineEmits<{
  (event: 'show-toast', message: string): void;
}>();

const filterLevel = ref<'ALL' | 'INFO' | 'WARN' | 'ERROR'>('ALL');
const search = ref('');
const copied = ref(false);

const levels: ('ALL' | 'INFO' | 'WARN' | 'ERROR')[] = ['ALL', 'INFO', 'WARN', 'ERROR'];

const mockLogs = [
  { time: '17:40:01', level: 'INFO', thread: 'main', text: `Aura Launcher v2.4.0 (HMCL Layout) initialized for ${props.instanceName}.` },
  { time: '17:40:02', level: 'INFO', thread: 'main', text: 'Checking Java 21 Temurin-21.0.3 64-Bit HotSpot Virtual Machine...' },
  { time: '17:40:03', level: 'INFO', thread: 'FabricLoader', text: 'Fabric Loader 0.16.9 successfully initialized 38 mods.' },
  { time: '17:40:04', level: 'INFO', thread: 'Sodium-Optimizer', text: 'Sodium rendering pipeline activated. OpenGL 4.6 Core ready.' },
  { time: '17:40:05', level: 'WARN', thread: 'ConfigLoader', text: 'Optional shader configuration found minor fallback flag.' },
  { time: '17:40:06', level: 'INFO', thread: 'RenderSystem', text: 'Display window created. Resolution: 1920x1080 Fullscreen.' },
  { time: '17:40:07', level: 'INFO', thread: 'MinecraftClient', text: 'Sound engine loaded. Joined singleplayer world.' },
];

const filteredLogs = computed(() =>
  mockLogs.filter((l) => {
    const matchLevel = filterLevel.value === 'ALL' || l.level === filterLevel.value;
    const matchSearch =
      l.text.toLowerCase().includes(search.value.toLowerCase()) ||
      l.thread.toLowerCase().includes(search.value.toLowerCase());
    return matchLevel && matchSearch;
  }),
);

function handleCopy() {
  const text = mockLogs.map((l) => `[${l.time}] [${l.thread}/${l.level}]: ${l.text}`).join('\n');
  void navigator.clipboard.writeText(text);
  copied.value = true;
  window.setTimeout(() => {
    copied.value = false;
  }, 2000);
  emit('show-toast', '控制台日志已复制到剪贴板');
}
</script>

<template>
  <div class="space-y-4 select-none pb-4">
    <div class="flex items-center justify-between border-b border-[#313233] pb-3">
      <div>
        <h1 class="text-xl font-bold text-white flex items-center gap-2">
          <Terminal class="w-5 h-5 text-emerald-400" />
          <span>实时游戏日志与控制台 (Game Logs)</span>
        </h1>
        <p class="text-xs text-slate-400 mt-0.5">当前追踪实例: {{ instanceName }}</p>
      </div>

      <div class="flex items-center gap-2">
        <BedrockButton variant="grey" size="sm" @click="handleCopy">
          <Check v-if="copied" class="w-3.5 h-3.5 mr-1 text-emerald-400" />
          <Copy v-else class="w-3.5 h-3.5 mr-1" />
          <span>{{ copied ? '已复制' : '复制日志' }}</span>
        </BedrockButton>
      </div>
    </div>

    <div class="mc-panel rounded-lg overflow-hidden flex flex-col h-[calc(100vh-14rem)]">
      <div class="flex items-center justify-between px-3 py-2 bg-[#1e1f20] border-b border-[#353638] text-xs">
        <div class="flex items-center gap-1">
          <button
            v-for="lvl in levels"
            :key="lvl"
            class="px-2.5 py-0.5 rounded text-xs font-mono font-bold cursor-pointer transition-colors"
            :class="filterLevel === lvl
              ? 'bg-[#3c8527] text-white'
              : 'bg-[#18191a] text-slate-400 hover:text-white'"
            @click="filterLevel = lvl"
          >
            {{ lvl }}
          </button>
        </div>

        <div class="relative">
          <Search class="w-3 h-3 text-slate-400 absolute left-2 top-1/2 -translate-y-1/2" />
          <input
            v-model="search"
            type="text"
            placeholder="搜索日志..."
            class="bg-[#141516] border border-[#3e3f41] rounded px-2 pl-6 py-0.5 text-xs text-white focus:outline-none"
          />
        </div>
      </div>

      <div class="flex-1 p-4 bg-[#141516] overflow-y-auto font-mono text-xs space-y-1 select-text">
        <div v-for="(l, i) in filteredLogs" :key="i" class="flex items-start gap-2 leading-relaxed">
          <span class="text-slate-500 shrink-0">[{{ l.time }}]</span>
          <span
            class="shrink-0 font-bold"
            :class="l.level === 'INFO' ? 'text-emerald-400' : l.level === 'WARN' ? 'text-amber-400' : 'text-rose-400'"
          >
            [{{ l.thread }}/{{ l.level }}]:
          </span>
          <span :class="l.level === 'WARN' ? 'text-amber-200' : l.level === 'ERROR' ? 'text-rose-300' : 'text-slate-300'">
            {{ l.text }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
