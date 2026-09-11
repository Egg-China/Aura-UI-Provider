<script setup lang="ts">
import { ref, watch } from 'vue';
import { X, Share2 } from 'lucide-vue-next';
import BedrockButton from './BedrockButton.vue';
import type { MinecraftInstance } from '../types/launcher';

const props = defineProps<{
  open: boolean;
  instance: MinecraftInstance | null;
}>();

const emit = defineEmits<{
  (event: 'close'): void;
  (event: 'export-instance', payload: { instance: MinecraftInstance; output: string; name: string }): void;
}>();

const output = ref('');
const name = ref('');
const isExporting = ref(false);

watch(
  () => [props.open, props.instance] as const,
  ([open, instance]) => {
    if (open && instance) {
      output.value = `${instance.name}.zip`;
      name.value = instance.name;
      isExporting.value = false;
    }
  },
  { immediate: true },
);

function handleSubmit() {
  if (!props.instance || output.value.trim().length === 0 || name.value.trim().length === 0) return;
  isExporting.value = true;
  emit('export-instance', {
    instance: props.instance,
    output: output.value.trim(),
    name: name.value.trim(),
  });
  isExporting.value = false;
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
          <Share2 class="w-4 h-4 text-emerald-400" />
          <span class="font-bold text-sm text-white">导出 MultiMC 整合包 (Export)</span>
        </div>
        <button class="text-slate-400 hover:text-white p-1 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4" />
        </button>
      </div>

      <form class="p-4 space-y-3" @submit.prevent="handleSubmit">
        <div class="space-y-1">
          <label class="text-xs font-bold text-white block">输出文件路径</label>
          <input
            v-model="output"
            type="text"
            required
            placeholder="D:\Packs\my-instance.zip"
            class="w-full bg-[#1b1c1d] border border-[#3e3f41] rounded p-2 text-xs text-white font-mono focus:outline-none focus:border-emerald-500"
          />
          <span class="text-[10px] text-slate-500 block">
            生成 instance.cfg + mmc-pack.json + overrides/ 的 MultiMC 格式压缩包，可在切换 AuraCore 后导入
          </span>
        </div>

        <div class="space-y-1">
          <label class="text-xs font-bold text-white block">包内实例名称</label>
          <input
            v-model="name"
            type="text"
            required
            class="w-full bg-[#1b1c1d] border border-[#3e3f41] rounded p-2 text-xs text-white focus:outline-none focus:border-emerald-500"
          />
        </div>

        <div class="flex gap-2 pt-3 border-t border-[#353638]">
          <BedrockButton type="button" variant="grey" size="sm" class="flex-1" @click="emit('close')">
            取消
          </BedrockButton>
          <BedrockButton type="submit" variant="green" size="sm" class="flex-1" :disabled="isExporting">
            {{ isExporting ? '导出中...' : '开始导出' }}
          </BedrockButton>
        </div>
      </form>
    </div>
  </div>
</template>
