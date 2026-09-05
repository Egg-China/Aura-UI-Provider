<script setup lang="ts">
import { ref } from 'vue';
import { Users2, Plus, Server, Copy, Play, Trash2, Radio, Share2, KeyRound, Wifi } from 'lucide-vue-next';
import BedrockButton from '../BedrockButton.vue';

const emit = defineEmits<{
  (event: 'launch'): void;
  (event: 'show-toast', message: string): void;
}>();

const activeTab = ref<'terracotta' | 'servers'>('terracotta');

const terracottaMode = ref<'home' | 'host' | 'guest'>('home');
const roomCode = ref('AURA-8821-X9K2');
const inputGuestCode = ref('');
const isHostRunning = ref(false);
const isGuestConnected = ref(false);

interface ServerEntry {
  id: string;
  name: string;
  address: string;
  ping: string;
  players: string;
  version: string;
  motd: string;
}

const servers = ref<ServerEntry[]>([
  { id: '1', name: 'Hypixel 官方网络', address: 'mc.hypixel.net', ping: '18ms', players: '42,190 / 100,000', version: '1.8 - 1.21.4', motd: 'Hypixel Network [1.8-1.21.4] • The Best Minecraft Minigames' },
  { id: '2', name: 'Aura 官方生存社区服', address: 'play.auramc.cn', ping: '12ms', players: '128 / 500', version: '1.21.4 Fabric', motd: '✦ Aura 纯净生存二周目 • 苍白之园与试炼密室已开放 ✦' },
  { id: '3', name: '机械动力工业联机服', address: 'create.auramc.cn', ping: '26ms', players: '48 / 100', version: '1.20.1 Forge', motd: '【Create 0.5.1】自动化流水线 • 沉浸列车与蒸汽动力' },
]);

const showAddModal = ref(false);
const newServerName = ref('');
const newServerAddress = ref('');

function copyText(text: string) {
  void navigator.clipboard.writeText(text);
}

function handleStartHost() {
  isHostRunning.value = true;
  const code = `AURA-${Math.floor(1000 + Math.random() * 9000)}-${Math.random().toString(36).substring(2, 6).toUpperCase()}`;
  roomCode.value = code;
  void navigator.clipboard.writeText(code);
  emit('show-toast', `陶瓦联机房间已创建！邀请码 [${code}] 已复制到剪贴板。`);
}

function handleJoinGuest() {
  if (!inputGuestCode.value.trim()) return;
  isGuestConnected.value = true;
  emit('show-toast', `已成功加入陶瓦联机房间 [${inputGuestCode.value.trim()}]，请启动游戏进入联机大厅！`);
}

function handleAddServer() {
  if (!newServerAddress.value.trim()) return;

  servers.value = [
    ...servers.value,
    {
      id: Date.now().toString(),
      name: newServerName.value.trim() || newServerAddress.value.trim(),
      address: newServerAddress.value.trim(),
      ping: '22ms',
      players: '0 / 100',
      version: '1.21.4',
      motd: '自定义 Minecraft 服务器',
    },
  ];
  newServerName.value = '';
  newServerAddress.value = '';
  showAddModal.value = false;
  emit('show-toast', '服务器添加成功');
}

function handleDeleteServer(id: string) {
  servers.value = servers.value.filter((s) => s.id !== id);
  emit('show-toast', '已移除服务器');
}
</script>

<template>
  <div class="h-full flex flex-col select-none">
    <div class="border-b border-[#24262b] pb-2.5 mb-3 flex items-center justify-between shrink-0">
      <div>
        <h1 class="text-sm font-bold text-white flex items-center gap-2">
          <Users2 class="w-4 h-4 text-[#2ea44f]" />
          <span>多人联机与陶瓦大厅 (Multiplayer & Terracotta)</span>
        </h1>
        <p class="text-[11px] text-slate-400">
          集成 HMCL-CE 陶瓦 EasyTier P2P 免公网穿透联机与经典服务器列表
        </p>
      </div>

      <div class="flex items-center gap-1 bg-[#161719] border border-[#2c2f35] rounded-md p-0.5 text-xs">
        <button
          class="px-3 py-1 rounded text-xs font-semibold transition-colors cursor-pointer flex items-center gap-1.5"
          :class="activeTab === 'terracotta' ? 'bg-[#2ea44f] text-white shadow-sm' : 'text-slate-400 hover:text-white'"
          @click="activeTab = 'terracotta'"
        >
          <Radio class="w-3.5 h-3.5" />
          <span>陶瓦 P2P 联机</span>
        </button>

        <button
          class="px-3 py-1 rounded text-xs font-semibold transition-colors cursor-pointer flex items-center gap-1.5"
          :class="activeTab === 'servers' ? 'bg-[#2ea44f] text-white shadow-sm' : 'text-slate-400 hover:text-white'"
          @click="activeTab = 'servers'"
        >
          <Server class="w-3.5 h-3.5" />
          <span>公共服务器列表</span>
        </button>
      </div>
    </div>

    <div class="flex-1 overflow-y-auto pr-1">
      <div v-if="activeTab === 'terracotta'" class="space-y-4 max-w-2xl">
        <div class="mc-panel p-4 flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-lg bg-[#1a2e20] border border-[#2c6d44] flex items-center justify-center text-emerald-400">
              <Wifi class="w-5 h-5" />
            </div>
            <div>
              <div class="flex items-center gap-2">
                <span class="font-bold text-xs text-white">陶瓦联机核心 (Terracotta EasyTier Engine)</span>
                <span class="text-[10px] font-mono px-1.5 py-[2px] rounded bg-[#1e2920] text-emerald-400 border border-[#27683c]">
                  状态: 已就绪
                </span>
              </div>
              <p class="text-[11px] text-slate-400 mt-0.5">
                基于 P2P 直连打洞技术，房主与房客直接高速通信，无需第三方服务器中转。
              </p>
            </div>
          </div>
        </div>

        <div class="grid grid-cols-2 gap-3">
          <div
            class="p-4 rounded-lg mc-panel border cursor-pointer transition-all"
            :class="terracottaMode === 'host' ? 'border-[#2ea44f] bg-[#1a261e]' : 'hover:border-[#383b42]'"
            @click="terracottaMode = 'host'"
          >
            <div class="flex items-center justify-between mb-2">
              <span class="font-bold text-xs text-white flex items-center gap-1.5">
                <Share2 class="w-4 h-4 text-emerald-400" />
                <span>创建联机</span>
              </span>
              <span class="text-[10px] text-emerald-400 font-mono">创建联机</span>
            </div>
            <p class="text-[11px] text-slate-400 leading-relaxed">
              在单人世界中按下 ESC 并选择“对局域网开放”，一键生成专属房间邀请码分享给好友。
            </p>
          </div>

          <div
            class="p-4 rounded-lg mc-panel border cursor-pointer transition-all"
            :class="terracottaMode === 'guest' ? 'border-[#2ea44f] bg-[#1a261e]' : 'hover:border-[#383b42]'"
            @click="terracottaMode = 'guest'"
          >
            <div class="flex items-center justify-between mb-2">
              <span class="font-bold text-xs text-white flex items-center gap-1.5">
                <KeyRound class="w-4 h-4 text-blue-400" />
                <span>加入联机</span>
              </span>
              <span class="text-[10px] text-blue-400 font-mono">加入联机</span>
            </div>
            <p class="text-[11px] text-slate-400 leading-relaxed">
              输入房主提供的邀请码，启动游戏进入多人游戏列表即可直接连接房主世界。
            </p>
          </div>
        </div>

        <div v-if="terracottaMode === 'host'" class="mc-panel p-4 space-y-3">
          <div class="flex items-center justify-between border-b border-[#24262b] pb-2">
            <span class="text-xs font-bold text-white">房主控制台</span>
            <span class="text-[10px] text-slate-400 font-mono">网络状态: 极好 (NAT 类型: Cone NAT)</span>
          </div>

          <div v-if="!isHostRunning" class="text-center py-4 space-y-3">
            <p class="text-xs text-slate-300">
              请先在游戏单人世界中“对局域网开放”，然后点击下方按钮启动陶瓦房间。
            </p>
            <button
              class="px-6 py-2 rounded-md bg-[#2ea44f] hover:bg-[#34b558] text-white text-xs font-semibold shadow-sm transition-colors cursor-pointer"
              @click="handleStartHost"
            >
              启动陶瓦联机房间
            </button>
          </div>

          <div v-else class="space-y-3">
            <div class="p-3 bg-[#121315] border border-[#24262b] rounded-lg flex items-center justify-between">
              <div>
                <div class="text-[10px] text-slate-400 uppercase font-mono">房间邀请码</div>
                <div class="font-mono text-base font-bold text-emerald-400 tracking-wider">{{ roomCode }}</div>
              </div>
              <button
                class="flex items-center gap-1 px-3 py-1.5 rounded bg-[#1f2125] hover:bg-[#272a2f] border border-[#2e3137] text-xs text-white transition-colors cursor-pointer"
                @click="copyText(roomCode); emit('show-toast', '邀请码已复制到剪贴板！')"
              >
                <Copy class="w-3.5 h-3.5" />
                <span>复制邀请码</span>
              </button>
            </div>

            <div class="flex justify-between items-center text-xs text-slate-400 pt-1">
              <span>已连接房客: 0 / 8 人</span>
              <button
                class="text-rose-400 hover:underline cursor-pointer"
                @click="isHostRunning = false; emit('show-toast', '已关闭陶瓦联机房间')"
              >
                关闭房间
              </button>
            </div>
          </div>
        </div>

        <div v-if="terracottaMode === 'guest'" class="mc-panel p-4 space-y-3">
          <span class="text-xs font-bold text-white block border-b border-[#24262b] pb-2">房客加入房间</span>

          <form class="space-y-3" @submit.prevent="handleJoinGuest">
            <div>
              <label class="text-xs text-slate-300 block mb-1">请输入房主提供的邀请码</label>
              <input
                v-model="inputGuestCode"
                type="text"
                required
                placeholder="例如: AURA-8821-X9K2"
                class="w-full bg-[#121315] border border-[#24262b] rounded-md px-3 py-2 text-xs font-mono text-white focus:outline-none focus:border-[#2ea44f]"
              />
            </div>

            <div class="flex gap-2">
              <button
                type="submit"
                class="px-6 py-2 rounded-md bg-[#2563eb] hover:bg-[#3b82f6] text-white text-xs font-semibold transition-colors cursor-pointer"
              >
                加入房间
              </button>
              <button
                v-if="isGuestConnected"
                type="button"
                class="px-6 py-2 rounded-md bg-[#2ea44f] hover:bg-[#34b558] text-white text-xs font-semibold transition-colors cursor-pointer flex items-center gap-1.5"
                @click="emit('launch')"
              >
                <Play class="w-3.5 h-3.5 fill-white" />
                <span>启动游戏进入大厅</span>
              </button>
            </div>
          </form>
        </div>
      </div>

      <div v-else class="space-y-3">
        <div class="flex justify-between items-center">
          <span class="text-xs font-semibold text-slate-400">已保存的服务器 ({{ servers.length }})</span>
          <BedrockButton variant="green" size="sm" @click="showAddModal = true">
            <Plus class="w-3.5 h-3.5 mr-1" />
            <span>添加服务器</span>
          </BedrockButton>
        </div>

        <div class="space-y-2">
          <div v-for="srv in servers" :key="srv.id" class="mc-card p-3 rounded flex items-center justify-between group">
            <div class="flex items-center gap-3 min-w-0">
              <div class="w-9 h-9 rounded bg-[#161719] border border-[#2c2f35] flex items-center justify-center text-emerald-400 shrink-0 font-mono text-xs">
                <Server class="w-4 h-4" />
              </div>

              <div class="min-w-0 space-y-0.5">
                <div class="flex items-center gap-2">
                  <span class="font-bold text-xs text-white truncate">{{ srv.name }}</span>
                  <span class="text-[10px] px-1.5 py-[2px] rounded bg-[#161719] text-emerald-400 font-mono border border-[#2c2f35]">
                    {{ srv.ping }}
                  </span>
                </div>
                <div class="text-[11px] text-slate-400 font-mono">
                  {{ srv.address }} • <span class="text-slate-300">{{ srv.version }}</span>
                </div>
                <p class="text-[10px] text-slate-500 truncate">{{ srv.motd }}</p>
              </div>
            </div>

            <div class="flex items-center gap-3 shrink-0">
              <div class="text-right font-mono text-[11px] text-slate-400 hidden sm:block">{{ srv.players }}</div>

              <div class="flex items-center gap-1.5">
                <button
                  class="p-1.5 rounded bg-[#1f2125] hover:bg-[#272a2f] text-slate-300 hover:text-white border border-[#2e3137] transition-colors cursor-pointer"
                  title="复制 IP"
                  @click="copyText(srv.address); emit('show-toast', `已复制 IP: ${srv.address}`)"
                >
                  <Copy class="w-3.5 h-3.5" />
                </button>
                <button
                  class="p-1.5 rounded bg-[#1f2125] hover:bg-rose-900/40 text-slate-300 hover:text-rose-400 border border-[#2e3137] transition-colors cursor-pointer"
                  title="删除"
                  @click="handleDeleteServer(srv.id)"
                >
                  <Trash2 class="w-3.5 h-3.5" />
                </button>
                <BedrockButton variant="green" size="sm" @click="emit('launch')">
                  <Play class="w-3 h-3 fill-current mr-1" />
                  <span>连接</span>
                </BedrockButton>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div
      v-if="showAddModal"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm"
      @click.self="showAddModal = false"
    >
      <div class="w-full max-w-sm bg-[#191b1e] border border-[#2c2f35] rounded-lg p-4 space-y-3 shadow-xl">
        <h3 class="text-sm font-bold text-white">添加新 Minecraft 服务器</h3>
        <form class="space-y-3" @submit.prevent="handleAddServer">
          <div>
            <label class="text-xs text-slate-300 block mb-1">服务器名称</label>
            <input
              v-model="newServerName"
              type="text"
              placeholder="例如: 朋友的联机私服"
              class="w-full bg-[#121315] border border-[#24262b] rounded p-2 text-xs text-white focus:outline-none focus:border-[#2ea44f]"
            />
          </div>
          <div>
            <label class="text-xs text-slate-300 block mb-1">服务器 IP 地址</label>
            <input
              v-model="newServerAddress"
              type="text"
              required
              placeholder="例如: play.myserver.com:25565"
              class="w-full bg-[#121315] border border-[#24262b] rounded p-2 text-xs text-white focus:outline-none focus:border-[#2ea44f]"
            />
          </div>
          <div class="flex gap-2 pt-2">
            <BedrockButton type="button" variant="grey" size="sm" class="flex-1" @click="showAddModal = false">
              取消
            </BedrockButton>
            <BedrockButton type="submit" variant="green" size="sm" class="flex-1">
              保存服务器
            </BedrockButton>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
