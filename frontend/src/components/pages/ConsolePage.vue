<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { Terminal, Copy, Search, Check, Square, Activity } from 'lucide-vue-next';
import BedrockButton from '../BedrockButton.vue';
import { auraCoreInstanceLogs, auraCoreStopInstance } from '../../bridge';

interface ConsoleLine {
  time: string;
  level: 'INFO' | 'WARN' | 'ERROR';
  thread: string;
  text: string;
}

const props = defineProps<{
  instanceName: string;
  instanceId: string;
  engineActive: boolean;
}>();

const emit = defineEmits<{
  (event: 'show-toast', message: string): void;
}>();

const filterLevel = ref<'ALL' | 'INFO' | 'WARN' | 'ERROR'>('ALL');
const search = ref('');
const copied = ref(false);
const liveLines = ref<ConsoleLine[]>([]);
const gameRunning = ref(false);
const isStopping = ref(false);
let pollTimer: number | undefined;
let pollInFlight = false;

const levels: ('ALL' | 'INFO' | 'WARN' | 'ERROR')[] = ['ALL', 'INFO', 'WARN', 'ERROR'];

const mockLogs: ConsoleLine[] = [
  { time: '17:40:01', level: 'INFO', thread: 'main', text: `Aura Launcher v2.4.0 (HMCL Layout) initialized for ${props.instanceName}.` },
  { time: '17:40:02', level: 'INFO', thread: 'main', text: 'Checking Java 21 Temurin-21.0.3 64-Bit HotSpot Virtual Machine...' },
  { time: '17:40:03', level: 'INFO', thread: 'FabricLoader', text: 'Fabric Loader 0.16.9 successfully initialized 38 mods.' },
  { time: '17:40:04', level: 'INFO', thread: 'Sodium-Optimizer', text: 'Sodium rendering pipeline activated. OpenGL 4.6 Core ready.' },
  { time: '17:40:05', level: 'WARN', thread: 'ConfigLoader', text: 'Optional shader configuration found minor fallback flag.' },
  { time: '17:40:06', level: 'INFO', thread: 'RenderSystem', text: 'Display window created. Resolution: 1920x1080 Fullscreen.' },
  { time: '17:40:07', level: 'INFO', thread: 'MinecraftClient', text: 'Sound engine loaded. Joined singleplayer world.' },
];

function normalizeLevel(raw: string): ConsoleLine['level'] {
  const value = raw.toLowerCase();
  if (value.includes('err') || value.includes('fatal')) return 'ERROR';
  if (value.includes('warn')) return 'WARN';
  return 'INFO';
}

async function pollAuraCoreLogs() {
  if (pollInFlight) return;
  pollInFlight = true;
  try {
    const reply = await auraCoreInstanceLogs(props.instanceId);
    gameRunning.value = reply.running;
    liveLines.value = reply.logs.map((entry) => ({
      time: 'live',
      level: normalizeLevel(entry.level),
      thread: 'AuraCore',
      text: entry.line,
    }));
  } catch (error) {
    emit('show-toast', `读取 AuraCore 日志失败: ${String(error)}`);
  } finally {
    pollInFlight = false;
  }
}

function startPolling() {
  stopPolling();
  liveLines.value = [];
  void pollAuraCoreLogs();
  pollTimer = window.setInterval(() => void pollAuraCoreLogs(), 1500);
}

function stopPolling() {
  if (pollTimer !== undefined) {
    window.clearInterval(pollTimer);
    pollTimer = undefined;
  }
}

async function handleStopGame() {
  isStopping.value = true;
  try {
    await auraCoreStopInstance(props.instanceId);
    emit('show-toast', '已请求 AuraCore 终止游戏进程');
    await pollAuraCoreLogs();
  } catch (error) {
    emit('show-toast', `停止游戏失败: ${String(error)}`);
  } finally {
    isStopping.value = false;
  }
}

watch(
  () => [props.engineActive, props.instanceId] as const,
  ([active]) => {
    if (active) {
      startPolling();
    } else {
      stopPolling();
    }
  },
);

onMounted(() => {
  if (props.engineActive) startPolling();
});

onUnmounted(stopPolling);

const displayLogs = computed(() => (props.engineActive ? liveLines.value : mockLogs));

const filteredLogs = computed(() =>
  displayLogs.value.filter((l) => {
    const matchLevel = filterLevel.value === 'ALL' || l.level === filterLevel.value;
    const matchSearch =
      l.text.toLowerCase().includes(search.value.toLowerCase()) ||
      l.thread.toLowerCase().includes(search.value.toLowerCase());
    return matchLevel && matchSearch;
  }),
);

function handleCopy() {
  const text = displayLogs.value.map((l) => `[${l.time}] [${l.thread}/${l.level}]: ${l.text}`).join('\n');
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
          <span
            v-if="engineActive"
            class="flex items-center gap-1 px-2 py-0.5 rounded text-[10px] font-mono font-bold border"
            :class="gameRunning
              ? 'bg-[#0f2e1a] text-emerald-400 border-[#29683e]'
              : 'bg-[#241a10] text-amber-400 border-[#4a3417]'"
          >
            <Activity class="w-3 h-3" />
            {{ gameRunning ? 'AuraCore 运行中' : '进程未运行' }}
          </span>
        </h1>
        <p class="text-xs text-slate-400 mt-0.5">
          当前追踪实例: {{ instanceName }}
          <span v-if="engineActive" class="font-mono text-slate-500">({{ instanceId }})</span>
        </p>
      </div>

      <div class="flex items-center gap-2">
        <BedrockButton v-if="engineActive" variant="danger" size="sm" :disabled="isStopping || !gameRunning" @click="handleStopGame">
          <Square class="w-3.5 h-3.5 mr-1" />
          <span>{{ isStopping ? '停止中...' : '停止游戏' }}</span>
        </BedrockButton>
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
        <div v-if="engineActive && filteredLogs.length === 0" class="text-slate-500">
          正在等待 AuraCore 输出日志...
        </div>
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
