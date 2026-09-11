<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { X, Package, FolderOpen } from 'lucide-vue-next';
import BedrockButton from './BedrockButton.vue';

const props = defineProps<{
  open: boolean;
}>();

const emit = defineEmits<{
  (event: 'close'): void;
  (event: 'import-instance', payload: { source: string; name: string; group: string }): void;
}>();

const source = ref('');
const name = ref('');
const group = ref('');
const isImporting = ref(false);

const derivedName = computed(() => {
  const raw = source.value.trim();
  if (raw.length === 0) return '';
  const withoutQuery = raw.split('?')[0].split('#')[0];
  const segments = withoutQuery.split(/[\\/]/).filter((segment) => segment.length > 0);
  const file = segments[segments.length - 1] ?? '';
  const withoutExtension = file.replace(/\.(zip|mrpack)$/i, '');
  return withoutExtension.length > 0 ? withoutExtension : '';
});

watch(
  () => props.open,
  (open) => {
    if (open) {
      source.value = '';
      name.value = '';
      group.value = '';
      isImporting.value = false;
    }
  },
);

watch(derivedName, (value) => {
  if (name.value.trim().length === 0 && value.length > 0) {
    name.value = value;
  }
});

function handleSubmit() {
  const trimmedSource = source.value.trim();
  const trimmedName = name.value.trim() || derivedName.value;
  if (trimmedSource.length === 0 || trimmedName.length === 0) return;
  isImporting.value = true;
  emit('import-instance', {
    source: trimmedSource,
    name: trimmedName,
    group: group.value.trim(),
  });
  isImporting.value = false;
  emit('close');
}
</script>

<template>
  <div
    v-if="open"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm select-none"
    @click.self="emit('close')"
  >
    <div class="relative w-full max-w-md bg-[#262729] border border-[#3b3c3e] rounded-lg shadow-2xl overflow-hidden flex flex-col">
      <div class="flex items-center justify-between px-4 py-3 bg-[#1e1f20] border-b border-[#353638]">
        <div class="flex items-center gap-2">
          <Package class="w-4 h-4 text-emerald-400" />
          <span class="font-bold text-sm text-white">导入实例 (Import Instance)</span>
        </div>
        <button class="text-slate-400 hover:text-white p-1 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4" />
        </button>
      </div>

      <form class="p-4 space-y-3" @submit.prevent="handleSubmit">
        <div class="space-y-1">
          <label class="text-xs font-bold text-white block">归档路径或 URL</label>
          <div class="flex items-center gap-2">
            <FolderOpen class="w-4 h-4 text-slate-500 shrink-0" />
            <input
              v-model="source"
              type="text"
              required
              placeholder="D:\Packs\my-instance.zip 或 https://example.com/pack.zip"
              class="flex-1 bg-[#1b1c1d] border border-[#3e3f41] rounded p-2 text-xs text-white font-mono focus:outline-none focus:border-emerald-500"
            />
          </div>
          <span class="text-[10px] text-slate-500 block">
            支持 MultiMC 格式（instance.cfg + mmc-pack.json + overrides/）；Modrinth / Curse / Technic 归档由后端自动识别
          </span>
        </div>

        <div class="space-y-1">
          <label class="text-xs font-bold text-white block">实例名称</label>
          <input
            v-model="name"
            type="text"
            required
            :placeholder="derivedName || '导入后的显示名称'"
            class="w-full bg-[#1b1c1d] border border-[#3e3f41] rounded p-2 text-xs text-white focus:outline-none focus:border-emerald-500"
          />
        </div>

        <div class="space-y-1">
          <label class="text-xs font-bold text-white block">分组 (可选)</label>
          <input
            v-model="group"
            type="text"
            placeholder="留空不分组"
            class="w-full bg-[#1b1c1d] border border-[#3e3f41] rounded p-2 text-xs text-white focus:outline-none focus:border-emerald-500"
          />
        </div>

        <div class="flex gap-2 pt-3 border-t border-[#353638]">
          <BedrockButton type="button" variant="grey" size="sm" class="flex-1" @click="emit('close')">
            取消
          </BedrockButton>
          <BedrockButton type="submit" variant="green" size="sm" class="flex-1" :disabled="isImporting">
            {{ isImporting ? '导入中...' : '开始导入' }}
          </BedrockButton>
        </div>
      </form>
    </div>
  </div>
</template>
