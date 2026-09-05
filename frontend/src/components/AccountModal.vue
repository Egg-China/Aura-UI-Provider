<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue';
import anime from 'animejs';
import {
  X,
  User,
  Check,
  Trash2,
  ExternalLink,
  Globe,
  ShieldCheck,
  ChevronLeft,
  ChevronRight,
} from 'lucide-vue-next';
import BedrockButton from './BedrockButton.vue';
import type { Account } from '../types/launcher';

type AccountModalMode = 'view' | 'add-ms' | 'add-thirdparty' | 'add-offline';

interface CardTheme {
  bgGradient: string;
  borderColor: string;
  activeBadge: string;
  subtext: string;
}

const props = defineProps<{
  open: boolean;
  accounts: Account[];
  currentAccount: Account;
}>();

const emit = defineEmits<{
  (event: 'close'): void;
  (event: 'select-account', account: Account): void;
  (event: 'add-account', account: Account): void;
  (event: 'delete-account', id: string): void;
}>();

const mode = ref<AccountModalMode>('view');
const currentIndex = ref(0);
const offlineName = ref('');
const thirdpartyName = ref('');
const thirdpartyServer = ref('https://littleskin.cn/api/yggdrasil');
const isAuthorizing = ref(false);
const activeCardRef = ref<HTMLDivElement | null>(null);
let wheelLocked = false;

watch(
  () => props.open,
  (open) => {
    if (open) {
      mode.value = 'view';
      const activeIdx = props.accounts.findIndex((a) => a.id === props.currentAccount.id);
      if (activeIdx !== -1) currentIndex.value = activeIdx;
    }
  },
);

watch(currentIndex, async () => {
  await nextTick();
  if (activeCardRef.value) {
    anime({
      targets: activeCardRef.value,
      translateY: [-14, 0],
      scale: [0.96, 1],
      opacity: [0.65, 1],
      duration: 250,
      easing: 'easeOutCubic',
    });
  }
});

const currentViewingAccount = computed(() => props.accounts[currentIndex.value] || props.accounts[0]);

function handlePrev() {
  currentIndex.value = currentIndex.value > 0 ? currentIndex.value - 1 : props.accounts.length - 1;
}

function handleNext() {
  currentIndex.value = currentIndex.value < props.accounts.length - 1 ? currentIndex.value + 1 : 0;
}

function handleWheel(event: WheelEvent) {
  if (wheelLocked) return;
  if (Math.abs(event.deltaY) > 18 || Math.abs(event.deltaX) > 18) {
    wheelLocked = true;
    if (event.deltaY > 0 || event.deltaX > 0) {
      handleNext();
    } else {
      handlePrev();
    }
    window.setTimeout(() => {
      wheelLocked = false;
    }, 260);
  }
}

function handleAddOffline() {
  if (!offlineName.value.trim()) return;

  const newAcc: Account = {
    id: `acc-${Date.now()}`,
    username: offlineName.value.trim(),
    uuid: 'offline-' + Math.random().toString(36).substring(2, 10),
    type: 'offline',
    skinUrl: `https://minotar.net/helm/${encodeURIComponent(offlineName.value.trim())}/128.png`,
    isActive: true,
  };

  emit('add-account', newAcc);
  offlineName.value = '';
  mode.value = 'view';
  currentIndex.value = 0;
}

function handleAddThirdParty() {
  if (!thirdpartyName.value.trim()) return;

  const newAcc: Account = {
    id: `acc-${Date.now()}`,
    username: thirdpartyName.value.trim(),
    uuid: 'tp-' + Math.random().toString(36).substring(2, 12),
    type: 'thirdparty',
    skinUrl: `https://minotar.net/helm/${encodeURIComponent(thirdpartyName.value.trim())}/128.png`,
    authServer: thirdpartyServer.value.replace('https://', '').split('/')[0] || 'littleskin.cn',
    isActive: true,
  };

  emit('add-account', newAcc);
  thirdpartyName.value = '';
  mode.value = 'view';
  currentIndex.value = 0;
}

function handleMicrosoftLogin() {
  isAuthorizing.value = true;
  window.setTimeout(() => {
    const newAcc: Account = {
      id: `acc-${Date.now()}`,
      username: 'Steve_' + Math.floor(Math.random() * 899 + 100),
      uuid: 'ms-' + Math.random().toString(36).substring(2, 12),
      type: 'microsoft',
      skinUrl: 'https://minotar.net/helm/MHF_Steve/128.png',
      isActive: true,
    };
    emit('add-account', newAcc);
    isAuthorizing.value = false;
    mode.value = 'view';
    currentIndex.value = 0;
  }, 1200);
}

function handleDeleteViewing() {
  emit('delete-account', currentViewingAccount.value.id);
  if (currentIndex.value >= props.accounts.length - 1) {
    currentIndex.value = Math.max(0, props.accounts.length - 2);
  }
}

function getCardTheme(account: Account, isCurrent: boolean): CardTheme {
  switch (account.type) {
    case 'microsoft':
      return {
        bgGradient: 'bg-gradient-to-br from-[#1b4e2f] via-[#22633c] to-[#143d24]',
        borderColor: isCurrent ? 'border-[#38b760] ring-4 ring-[#38b760]/30' : 'border-[#2c7546]',
        activeBadge: 'bg-[#0f2e1a] text-[#4ade80] border-[#29683e]',
        subtext: 'Microsoft',
      };
    case 'thirdparty':
      return {
        bgGradient: 'bg-gradient-to-br from-[#164070] via-[#1c5391] to-[#123359]',
        borderColor: isCurrent ? 'border-[#3b82f6] ring-4 ring-[#3b82f6]/30' : 'border-[#265e9c]',
        activeBadge: 'bg-[#0e243d] text-[#60a5fa] border-[#244c77]',
        subtext: account.authServer || 'littleskin.cn',
      };
    default:
      return {
        bgGradient: 'bg-gradient-to-br from-[#282a30] via-[#32363d] to-[#202227]',
        borderColor: isCurrent ? 'border-[#9ca3af] ring-4 ring-[#9ca3af]/30' : 'border-[#434752]',
        activeBadge: 'bg-[#151619] text-[#d1d5db] border-[#373a42]',
        subtext: 'Offline',
      };
  }
}

const isCardActive = computed(() => currentViewingAccount.value?.id === props.currentAccount.id);
const currentTheme = computed(() => getCardTheme(currentViewingAccount.value || props.accounts[0], isCardActive.value));

const nextAccount = computed(() =>
  props.accounts.length > 1 ? props.accounts[(currentIndex.value + 1) % props.accounts.length] : null,
);
const thirdAccount = computed(() =>
  props.accounts.length > 2 ? props.accounts[(currentIndex.value + 2) % props.accounts.length] : null,
);
const nextTheme = computed(() => (nextAccount.value ? getCardTheme(nextAccount.value, false) : null));
const thirdTheme = computed(() => (thirdAccount.value ? getCardTheme(thirdAccount.value, false) : null));
</script>

<template>
  <div
    v-if="open"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm select-none"
    @click.self="emit('close')"
  >
    <div class="relative w-full max-w-lg bg-[#191b1e] border border-[#2c2f35] rounded-xl shadow-2xl overflow-hidden flex flex-col">
      <div class="flex items-center justify-between px-5 py-3.5 bg-[#141517] border-b border-[#24262b]">
        <div class="flex items-center gap-2">
          <User class="w-4 h-4 text-[#2ea44f]" />
          <span class="font-bold text-sm text-white">账户管理 (Account Management)</span>
        </div>
        <button class="text-slate-400 hover:text-white p-1 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4" />
        </button>
      </div>

      <div class="p-6 space-y-5" @wheel="handleWheel">
        <template v-if="mode === 'view' && currentViewingAccount">
          <!-- 层叠卡组与左右导航 -->
          <div class="relative flex items-center justify-center gap-2 pt-1 pb-3">
            <button
              class="w-9 h-9 rounded-full bg-[#24272c] hover:bg-[#2f333a] active:scale-95 text-slate-300 hover:text-white flex items-center justify-center border border-[#373b43] transition-all cursor-pointer shadow-md shrink-0 z-30"
              title="上一个账户 (支持滚轮)"
              @click="handlePrev"
            >
              <ChevronLeft class="w-5 h-5" />
            </button>

            <div class="relative w-[385px] h-[225px] flex items-center justify-center">
              <div
                v-if="thirdAccount && thirdTheme"
                class="absolute inset-0 rounded-2xl border transition-all duration-300 pointer-events-none"
                :class="[thirdTheme.bgGradient, thirdTheme.borderColor]"
                :style="{ transform: 'translateY(14px) scale(0.88)', opacity: 0.25, zIndex: 1 }"
              />

              <div
                v-if="nextAccount && nextTheme"
                class="absolute inset-0 rounded-2xl border transition-all duration-300 pointer-events-none"
                :class="[nextTheme.bgGradient, nextTheme.borderColor]"
                :style="{ transform: 'translateY(8px) scale(0.94)', opacity: 0.45, zIndex: 2 }"
              />

              <div
                ref="activeCardRef"
                class="absolute inset-0 rounded-2xl p-5 border transition-all duration-200 cursor-pointer shadow-2xl flex flex-col justify-between overflow-hidden z-10"
                :class="[currentTheme.bgGradient, currentTheme.borderColor]"
                @click="emit('select-account', currentViewingAccount)"
              >
                <div class="flex items-start justify-between relative z-10">
                  <div class="flex items-center gap-3">
                    <div class="w-11 h-11 rounded-lg bg-black/40 border border-white/15 p-0.5 flex items-center justify-center shrink-0 shadow-inner">
                      <img
                        :src="currentViewingAccount.skinUrl"
                        :alt="currentViewingAccount.username"
                        class="w-9 h-9 rounded-sm pixelated"
                        @error="($event.target as HTMLImageElement).style.display = 'none'"
                      />
                    </div>

                    <div>
                      <h3 class="font-black text-lg text-white tracking-wide drop-shadow-sm leading-tight">
                        {{ currentViewingAccount.username }}
                      </h3>
                      <span class="text-[11px] font-medium text-white/70 tracking-wide">
                        {{ currentTheme.subtext }}
                      </span>
                    </div>
                  </div>

                  <div class="flex items-center gap-2">
                    <span
                      v-if="isCardActive"
                      class="text-[11px] font-bold px-2.5 py-0.5 rounded-full border flex items-center gap-1 shadow-sm"
                      :class="currentTheme.activeBadge"
                    >
                      <Check class="w-3.5 h-3.5" /> 当前使用
                    </span>
                    <button
                      v-else
                      class="p-1.5 rounded-md bg-black/30 hover:bg-rose-900/70 text-white/60 hover:text-rose-200 border border-white/15 transition-colors cursor-pointer"
                      title="移除此账户"
                      @click.stop="handleDeleteViewing"
                    >
                      <Trash2 class="w-4 h-4" />
                    </button>
                  </div>
                </div>

                <div class="relative z-10 text-center my-auto py-1">
                  <div class="font-mono text-[11px] sm:text-xs tracking-wider text-white/95 font-semibold drop-shadow select-text">
                    {{ currentViewingAccount.uuid }}
                  </div>
                </div>

                <div class="flex items-center justify-between relative z-10 h-7">
                  <div />

                  <div class="flex items-center justify-end">
                    <div
                      v-if="currentViewingAccount.type === 'microsoft'"
                      class="grid grid-cols-2 gap-0.5 w-5 h-5 shadow-sm"
                      title="Microsoft Account"
                    >
                      <div class="bg-[#f25022] rounded-[1px]" />
                      <div class="bg-[#7fba00] rounded-[1px]" />
                      <div class="bg-[#00a4ef] rounded-[1px]" />
                      <div class="bg-[#ffb900] rounded-[1px]" />
                    </div>

                    <div
                      v-else-if="currentViewingAccount.type === 'offline'"
                      class="flex items-center gap-1.5"
                      title="Aura Launcher Offline Account"
                    >
                      <img
                        src="/IMG_20260827_125438_128x128.png"
                        alt="Aura Launcher Logo"
                        class="w-5.5 h-5.5 object-contain drop-shadow"
                        @error="($event.target as HTMLImageElement).src = '/IMG_20260827_125438_128x128.svg'"
                      />
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <button
              class="w-9 h-9 rounded-full bg-[#24272c] hover:bg-[#2f333a] active:scale-95 text-slate-300 hover:text-white flex items-center justify-center border border-[#373b43] transition-all cursor-pointer shadow-md shrink-0 z-30"
              title="下一个账户 (支持滚轮)"
              @click="handleNext"
            >
              <ChevronRight class="w-5 h-5" />
            </button>
          </div>

          <!-- 分页指示与确认按钮 -->
          <div class="flex items-center justify-between pt-1">
            <div class="flex items-center gap-1.5">
              <button
                v-for="(_, idx) in accounts"
                :key="idx"
                class="h-2 rounded-full transition-all cursor-pointer"
                :class="idx === currentIndex ? 'w-6 bg-[#2ea44f]' : 'w-2 bg-[#33363e] hover:bg-[#454954]'"
                @click="currentIndex = idx"
              />
              <span class="text-[11px] text-slate-400 font-mono ml-2">
                {{ currentIndex + 1 }} / {{ accounts.length }}
              </span>
            </div>

            <button
              class="px-4 py-1.5 rounded-md font-semibold text-xs transition-colors cursor-pointer"
              :class="isCardActive
                ? 'bg-[#2ea44f] text-white'
                : 'bg-[#282a2e] hover:bg-[#33363d] text-slate-200 border border-[#3b3e46]'"
              @click="emit('select-account', currentViewingAccount)"
            >
              {{ isCardActive ? '✓ 当前已选中' : '切换为此账户' }}
            </button>
          </div>

          <!-- 底部三个标准添加账户按钮 -->
          <div class="grid grid-cols-3 gap-2 pt-4 border-t border-[#26282e]">
            <button
              class="flex items-center justify-center gap-1.5 py-2.5 px-2 rounded-md bg-[#2ea44f] hover:bg-[#34b558] active:bg-[#279044] text-white text-xs font-semibold shadow-sm transition-colors cursor-pointer"
              @click="mode = 'add-ms'"
            >
              <ShieldCheck class="w-3.5 h-3.5" />
              <span>微软登录</span>
            </button>

            <button
              class="flex items-center justify-center gap-1.5 py-2.5 px-2 rounded-md bg-[#2563eb] hover:bg-[#3b82f6] active:bg-[#1d4ed8] text-white text-xs font-semibold shadow-sm transition-colors cursor-pointer"
              @click="mode = 'add-thirdparty'"
            >
              <Globe class="w-3.5 h-3.5" />
              <span>第三方登录</span>
            </button>

            <button
              class="flex items-center justify-center gap-1.5 py-2.5 px-2 rounded-md bg-[#2a2d33] hover:bg-[#33363d] active:bg-[#222428] text-slate-200 border border-[#3b3e46] text-xs font-semibold shadow-sm transition-colors cursor-pointer"
              @click="mode = 'add-offline'"
            >
              <User class="w-3.5 h-3.5" />
              <span>离线账户</span>
            </button>
          </div>
        </template>

        <!-- 模式: 添加微软账户 -->
        <div v-else-if="mode === 'add-ms'" class="space-y-4 text-center py-3">
          <div class="w-12 h-12 mx-auto rounded-full bg-[#2ea44f]/20 border border-[#2ea44f]/40 flex items-center justify-center text-[#34b558]">
            <ShieldCheck class="w-6 h-6" />
          </div>
          <div class="space-y-1">
            <div class="text-sm font-bold text-white">添加微软账户</div>
            <p class="text-xs text-slate-400 max-w-sm mx-auto">
              使用微软官方 OAuth2 登录协议，安全同步 Minecraft 皮肤与角色数据。
            </p>
          </div>
          <div class="pt-2 space-y-2 max-w-xs mx-auto">
            <button
              :disabled="isAuthorizing"
              class="w-full py-2.5 rounded-md bg-[#2ea44f] hover:bg-[#34b558] text-white font-semibold text-xs transition-colors cursor-pointer flex items-center justify-center gap-1.5 disabled:opacity-70"
              @click="handleMicrosoftLogin"
            >
              <template v-if="isAuthorizing">
                <div class="w-3.5 h-3.5 border-2 border-white border-t-transparent rounded-full animate-spin" />
                <span>正在获取微软授权...</span>
              </template>
              <template v-else>
                <span>前往浏览器授权登录</span>
                <ExternalLink class="w-3.5 h-3.5" />
              </template>
            </button>
            <BedrockButton variant="subtle" size="sm" class="w-full" @click="mode = 'view'">
              返回账户列表
            </BedrockButton>
          </div>
        </div>

        <!-- 模式: 添加第三方认证账户 -->
        <form v-else-if="mode === 'add-thirdparty'" class="space-y-3 py-1" @submit.prevent="handleAddThirdParty">
          <div class="text-xs font-bold text-blue-400 flex items-center gap-1.5 mb-1">
            <Globe class="w-4 h-4" />
            <span>添加第三方认证账户</span>
          </div>
          <div>
            <label class="text-xs text-slate-300 block mb-1">角色昵称</label>
            <input
              v-model="thirdpartyName"
              type="text"
              required
              placeholder="例如: LittleSkin_Player"
              class="w-full bg-[#16171a] border border-[#2e3137] rounded-md p-2 text-xs text-white focus:outline-none focus:border-blue-500"
            />
          </div>
          <div>
            <label class="text-xs text-slate-300 block mb-1">Yggdrasil 认证服务器 URL</label>
            <input
              v-model="thirdpartyServer"
              type="text"
              class="w-full bg-[#16171a] border border-[#2e3137] rounded-md p-2 text-xs text-white font-mono focus:outline-none focus:border-blue-500"
              placeholder="https://littleskin.cn/api/yggdrasil"
            />
            <span class="text-[10px] text-slate-500 mt-1 block">支持 LittleSkin、Blessing Skin 等主流皮肤站。</span>
          </div>
          <div class="flex gap-2 pt-2">
            <BedrockButton type="button" variant="grey" size="sm" class="flex-1" @click="mode = 'view'">
              取消
            </BedrockButton>
            <button
              type="submit"
              :disabled="!thirdpartyName.trim()"
              class="flex-1 py-2 rounded-md bg-[#2563eb] hover:bg-[#3b82f6] text-white text-xs font-semibold transition-colors disabled:opacity-50 cursor-pointer"
            >
              确认添加
            </button>
          </div>
        </form>

        <!-- 模式: 添加离线账户 -->
        <form v-else-if="mode === 'add-offline'" class="space-y-3 py-1" @submit.prevent="handleAddOffline">
          <div class="text-xs font-bold text-slate-300 flex items-center gap-1.5 mb-1">
            <User class="w-4 h-4 text-slate-400" />
            <span>添加离线账户</span>
          </div>
          <div>
            <label class="text-xs text-slate-300 block mb-1">玩家昵称</label>
            <input
              v-model="offlineName"
              type="text"
              required
              autofocus
              placeholder="例如: Steve_Hunter"
              class="w-full bg-[#16171a] border border-[#2e3137] rounded-md p-2 text-xs text-white focus:outline-none focus:border-[#2ea44f]"
            />
          </div>
          <div class="flex gap-2 pt-2">
            <BedrockButton type="button" variant="grey" size="sm" class="flex-1" @click="mode = 'view'">
              取消
            </BedrockButton>
            <button
              type="submit"
              :disabled="!offlineName.trim()"
              class="flex-1 py-2 rounded-md bg-[#2ea44f] hover:bg-[#34b558] text-white text-xs font-semibold transition-colors disabled:opacity-50 cursor-pointer"
            >
              确认添加
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
