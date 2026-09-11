<script setup lang="ts">
import { ref, watch } from 'vue';
import { X, Pencil } from 'lucide-vue-next';
import BedrockButton from './BedrockButton.vue';
import type { MinecraftInstance } from '../types/launcher';

const props = defineProps<{
  open: boolean;
  instance: MinecraftInstance | null;
  auraCoreActive?: boolean;
}>();

const emit = defineEmits<{
  (event: 'close'): void;
  (event: 'save-instance', payload: { instance: MinecraftInstance; name: string; group: string; icon: string }): void;
}>();

const name = ref('');
const group = ref('');
const icon = ref('⛏️');
const isSaving = ref(false);

watch(
  () => [props.open, props.instance] as const,
  ([open, instance]) => {
    if (open && instance) {
      name.value = instance.name;
      group.value = instance.group ?? '';
      icon.value = instance.icon;
      isSaving.value = false;
    }
  },
  { immediate: true },
);

function handleSubmit() {
  if (!props.instance || !name.value.trim()) return;
  isSaving.value = true;
  emit('save-instance', {
    instance: props.instance,
    name: name.value.trim(),
    group: group.value.trim(),
    icon: icon.value.trim() || '⛏️',
  });
  isSaving.value = false;
  emit('close');
}
</script>

<template>
  <div
    v-if="open && instance"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm select-none"
    @click.self="emit('close')"
  >
    <div class="relative w-full max-w-md bg-[#262729] border border-[#3b3c3e] rounded-lg shadow-2xl overflow-hidden flex flex-col">
      <div class="flex items-center justify-between px-4 py-3 bg-[#1e1f20] border-b border-[#353638]">
        <div class="flex items-center gap-2">
          <Pencil class="w-4 h-4 text-emerald-400" />
          <span class="font-bold text-sm text-white">编辑实例 (Edit Instance)</span>
        </div>
        <button class="text-slate-400 hover:text-white p-1 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4" />
        </button>
      </div>

      <form class="p-4 space-y-3" @submit.prevent="handleSubmit">
        <div class="space-y-1">
          <label class="text-xs font-bold text-white block">实例名称</label>
          <input
            v-model="name"
            type="text"
            required
            class="w-full bg-[#1b1c1d] border border-[#3e3f41] rounded p-2 text-xs text-white focus:outline-none focus:border-emerald-500"
          />
        </div>

        <div class="space-y-1">
          <label class="text-xs font-bold text-white block">分组 (Group)</label>
          <input
            v-model="group"
            type="text"
            placeholder="留空清除分组"
            class="w-full bg-[#1b1c1d] border border-[#3e3f41] rounded p-2 text-xs text-white focus:outline-none focus:border-emerald-500"
          />
          <span v-if="auraCoreActive" class="text-[10px] text-slate-500 block font-mono">
            写入 AuraCore 实例分组；非 AuraCore 模式仅保存在当前界面会话
          </span>
        </div>

        <div class="space-y-1">
          <label class="text-xs font-bold text-white block">图标</label>
          <div class="flex items-center gap-2">
            <span class="w-9 h-9 rounded bg-[#1b1c1d] border border-[#3e3f41] flex items-center justify-center text-lg">
              {{ icon }}
            </span>
            <input
              v-model="icon"
              type="text"
              class="flex-1 bg-[#1b1c1d] border border-[#3e3f41] rounded p-2 text-xs text-white focus:outline-none focus:border-emerald-500"
            />
          </div>
          <div class="flex items-center gap-1.5 pt-1">
            <button
              v-for="emoji in ['⛏️', '🗡️', '🌿', '🔥', '💎', '🐉']"
              :key="emoji"
              type="button"
              class="w-7 h-7 rounded bg-[#1e1f20] border border-[#38393b] hover:border-emerald-500 text-sm cursor-pointer transition-colors"
              @click="icon = emoji"
            >
              {{ emoji }}
            </button>
          </div>
        </div>

        <div class="flex gap-2 pt-3 border-t border-[#353638]">
          <BedrockButton type="button" variant="grey" size="sm" class="flex-1" @click="emit('close')">
            取消
          </BedrockButton>
          <BedrockButton type="submit" variant="green" size="sm" class="flex-1" :disabled="isSaving">
            {{ isSaving ? '保存中...' : '保存修改' }}
          </BedrockButton>
        </div>
      </form>
    </div>
  </div>
</template>
