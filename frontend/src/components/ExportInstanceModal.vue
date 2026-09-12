<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { X, Share2, ChevronRight, ChevronDown, Folder, FileText } from 'lucide-vue-next';
import BedrockButton from './BedrockButton.vue';
import type { MinecraftInstance } from '../types/launcher';
import {
    listInstanceExportFiles,
    type ExportFileEntry,
    type ExportFileListing,
} from '../bridge';

interface TreeNode {
  name: string;
  path: string;
  directory: boolean;
  checked: boolean;
  indeterminate: boolean;
  expanded: boolean;
  loading: boolean;
  truncated: boolean;
  childrenLoaded: boolean;
  children: TreeNode[];
  loadPromise: Promise<void> | null;
}

interface VisibleNode {
  node: TreeNode;
  depth: number;
}

const props = defineProps<{
  open: boolean;
  instance: MinecraftInstance | null;
}>();

const emit = defineEmits<{
  (event: 'close'): void;
  (event: 'export-instance', payload: {
    instance: MinecraftInstance;
    output: string;
    name: string;
    whitelist?: string[];
  }): void;
}>();

const output = ref('');
const name = ref('');
const mode = ref<'full' | 'custom'>('full');
const root = ref<TreeNode | null>(null);
const rootLoading = ref(false);
const selectionSupported = ref(false);
const selectionNotice = ref('');
const collecting = ref(false);
const collectionError = ref('');

/// Monotonic request generation: stale async continuations abort instead of mutating state.
let requestGeneration = 0;
/// Token sent to the launcher so stale replies can be identified after reopening.
let requestToken = '';

const visibleNodes = computed<VisibleNode[]>(() => {
  const flattened: VisibleNode[] = [];
  const walk = (nodes: TreeNode[], depth: number) => {
    for (const node of nodes) {
      flattened.push({ node, depth });
      if (node.directory && node.expanded && node.childrenLoaded) {
        walk(node.children, depth + 1);
      }
    }
  };
  if (root.value !== null) {
    walk(root.value.children, 0);
  }
  return flattened;
});

function isExportFileListing(value: ExportFileListing): boolean {
  if (typeof value.path !== 'string' || typeof value.truncated !== 'boolean') {
    return false;
  }
  return Array.isArray(value.entries);
}

function isExportFileEntry(value: ExportFileEntry): boolean {
  return typeof value.name === 'string'
      && value.name.length > 0
      && typeof value.path === 'string'
      && value.path.length > 0
      && typeof value.directory === 'boolean'
      && typeof value.suggested === 'boolean';
}

function makeNode(entry: ExportFileEntry, checked: boolean): TreeNode {
  return {
    name: entry.name,
    path: entry.path,
    directory: entry.directory,
    checked,
    indeterminate: false,
    expanded: false,
    loading: false,
    truncated: false,
    childrenLoaded: false,
    children: [],
    loadPromise: null,
  };
}

async function loadChildren(node: TreeNode, markChecked: boolean): Promise<void> {
  if (node.loadPromise !== null) {
    await node.loadPromise;
    return;
  }
  if (node.childrenLoaded || props.instance === null) {
    return;
  }
  const instanceId = props.instance.id;
  const token = requestToken;
  node.loading = true;
  const promise = (async () => {
    try {
      const listing = await listInstanceExportFiles(instanceId, node.path, token);
      if (!isExportFileListing(listing) || listing.token !== token) {
        return;
      }
      if (!(listing.entries ?? []).every(isExportFileEntry)) {
        throw new Error('malformed export listing');
      }
      node.truncated = listing.truncated === true;
      node.children = (listing.entries ?? [])
          .map((entry) => makeNode(entry, markChecked || entry.suggested));
      node.childrenLoaded = true;
    } finally {
      node.loading = false;
      node.loadPromise = null;
    }
  })();
  node.loadPromise = promise;
  await promise;
}

async function toggleExpanded(node: TreeNode): Promise<void> {
  if (!node.directory || node.loading) {
    return;
  }
  if (!node.childrenLoaded) {
    try {
      await loadChildren(node, false);
    } catch (error) {
      collectionError.value = `读取目录失败: ${String(error)}`;
      return;
    }
  }
  node.expanded = !node.expanded;
}

function setDescendants(node: TreeNode, checked: boolean): void {
  node.checked = checked;
  node.indeterminate = false;
  if (node.childrenLoaded) {
    for (const child of node.children) {
      setDescendants(child, checked);
    }
  }
}

function recomputeTreeStates(nodes: TreeNode[]): { checked: boolean; indeterminate: boolean } {
  let checkedCount = 0;
  let indeterminateCount = 0;
  for (const node of nodes) {
    if (node.childrenLoaded && node.children.length > 0) {
      const state = recomputeTreeStates(node.children);
      node.indeterminate = state.indeterminate;
      node.checked = state.checked;
    }
    if (node.checked) checkedCount += 1;
    if (node.indeterminate) indeterminateCount += 1;
  }
  const total = nodes.length;
  const fullyChecked = total > 0 && checkedCount === total && indeterminateCount === 0;
  return {
    checked: fullyChecked,
    indeterminate: (checkedCount > 0 || indeterminateCount > 0) && !fullyChecked,
  };
}

function syncTreeStates(): void {
  if (root.value !== null) {
    recomputeTreeStates(root.value.children);
  }
}

function toggleChecked(node: TreeNode): void {
  setDescendants(node, !(node.checked || node.indeterminate));
  syncTreeStates();
}

async function collectSelection(node: TreeNode, paths: string[], generation: number): Promise<void> {
  if (!node.checked && !node.indeterminate) {
    return;
  }
  if (node.directory && !node.childrenLoaded) {
    await loadChildren(node, true);
  }
  if (generation !== requestGeneration) {
    throw new Error('cancelled');
  }
  if (node.directory && node.checked) {
    // An expansion that raced this collection may have installed suggested-only defaults.
    setDescendants(node, true);
  }
  if (node.directory && node.checked && node.truncated) {
    throw new Error(`目录条目被截断: ${node.path}`);
  }
  paths.push(node.path);
  for (const child of node.children) {
    await collectSelection(child, paths, generation);
  }
}

async function handleSubmit(): Promise<void> {
  if (!props.instance || output.value.trim().length === 0 || name.value.trim().length === 0) {
    return;
  }
  if (mode.value === 'custom') {
    if (root.value === null || !selectionSupported.value) {
      collectionError.value = '当前环境不支持自定义选择';
      return;
    }
    const generation = requestGeneration;
    collecting.value = true;
    collectionError.value = '';
    try {
      const paths: string[] = [];
      for (const child of root.value.children) {
        await collectSelection(child, paths, generation);
      }
      if (generation !== requestGeneration) {
        return;
      }
      if (paths.length === 0) {
        collectionError.value = '请至少选择一个要导出的条目';
        return;
      }
      emit('export-instance', {
        instance: props.instance,
        output: output.value.trim(),
        name: name.value.trim(),
        whitelist: paths,
      });
    } catch (error) {
      if (generation === requestGeneration) {
        collectionError.value = `无法精确导出: ${String(error)}`;
      }
      return;
    } finally {
      if (generation === requestGeneration) {
        collecting.value = false;
      }
    }
  } else {
    emit('export-instance', {
      instance: props.instance,
      output: output.value.trim(),
      name: name.value.trim(),
    });
  }
  emit('close');
}

watch(
  () => [props.open, props.instance] as const,
  ([open, instance]) => {
    requestGeneration += 1;
    requestToken = `export-${requestGeneration}`;
    if (!open || instance === null) {
      return;
    }
    output.value = `${instance.name}.zip`;
    name.value = instance.name;
    mode.value = 'full';
    root.value = null;
    rootLoading.value = true;
    selectionSupported.value = false;
    selectionNotice.value = '';
    collectionError.value = '';
    collecting.value = false;
    const generation = requestGeneration;
    const token = requestToken;
    listInstanceExportFiles(instance.id, '', token)
        .then((listing) => {
          if (generation !== requestGeneration || listing.token !== token) {
            return;
          }
          if (!isExportFileListing(listing)
              || !(listing.entries ?? []).every(isExportFileEntry)) {
            selectionNotice.value = '导出文件列表格式异常，已保持全量导出';
            return;
          }
          const rootNode: TreeNode = {
            name: '',
            path: '',
            directory: true,
            checked: false,
            indeterminate: false,
            expanded: true,
            loading: false,
            truncated: listing.truncated === true,
            childrenLoaded: true,
            children: (listing.entries ?? [])
                .map((entry) => makeNode(entry, entry.suggested)),
            loadPromise: null,
          };
          root.value = rootNode;
          if (rootNode.truncated) {
            selectionNotice.value = '实例根目录条目过多，自定义选择不可用';
            return;
          }
          selectionSupported.value = true;
        })
        .catch(() => {
          if (generation === requestGeneration) {
            selectionNotice.value = '当前启动器不支持自定义选择，已保持全量导出';
          }
        })
        .finally(() => {
          if (generation === requestGeneration) {
            rootLoading.value = false;
          }
        });
  },
  { immediate: true },
);
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

        <div class="space-y-1">
          <label class="text-xs font-bold text-white block">导出范围</label>
          <div class="grid grid-cols-2 gap-2">
            <button
              type="button"
              class="rounded border px-2 py-1.5 text-xs transition-colors"
              :class="mode === 'full' ? 'border-emerald-500 bg-emerald-500/10 text-emerald-300' : 'border-[#3e3f41] bg-[#1b1c1d] text-slate-300 hover:border-[#4e4f51]'"
              @click="mode = 'full'"
            >
              全量导出（推荐迁移）
            </button>
            <button
              type="button"
              :disabled="!selectionSupported"
              class="rounded border px-2 py-1.5 text-xs transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
              :class="mode === 'custom' ? 'border-emerald-500 bg-emerald-500/10 text-emerald-300' : 'border-[#3e3f41] bg-[#1b1c1d] text-slate-300 hover:border-[#4e4f51]'"
              @click="mode = 'custom'"
            >
              自定义选择
            </button>
          </div>
          <span v-if="rootLoading" class="text-[10px] text-slate-500 block">正在读取实例文件树...</span>
          <span v-else-if="selectionNotice" class="text-[10px] text-amber-400 block">{{ selectionNotice }}</span>
          <span v-else-if="mode === 'custom'" class="text-[10px] text-slate-500 block">
            勾选目录会在导出前自动展开；取消勾选可在分享时排除存档等私人文件
          </span>
        </div>

        <div
          v-if="mode === 'custom' && selectionSupported && root"
          class="max-h-56 overflow-auto rounded border border-[#3e3f41] bg-[#1b1c1d] p-1 space-y-0.5"
        >
          <div
            v-for="item in visibleNodes"
            :key="item.node.path"
            class="flex items-center gap-1.5 rounded px-1 py-0.5 hover:bg-[#232527]"
            :style="{ marginLeft: `${item.depth * 14}px` }"
          >
            <button
              v-if="item.node.directory"
              type="button"
              class="text-slate-500 hover:text-white p-0.5"
              @click="toggleExpanded(item.node)"
            >
              <ChevronDown v-if="item.node.expanded" class="w-3 h-3" />
              <ChevronRight v-else class="w-3 h-3" />
            </button>
            <span v-else class="w-4"></span>
            <input
              type="checkbox"
              class="accent-emerald-500 w-3 h-3"
              :checked="item.node.checked"
              :indeterminate.prop="item.node.indeterminate"
              @change="toggleChecked(item.node)"
            />
            <Folder v-if="item.node.directory" class="w-3.5 h-3.5 text-sky-400 shrink-0" />
            <FileText v-else class="w-3.5 h-3.5 text-slate-400 shrink-0" />
            <span class="text-[11px] text-slate-200 truncate" :title="item.node.path">{{ item.node.name }}</span>
            <span v-if="item.node.loading" class="text-[10px] text-slate-500">加载中...</span>
            <span v-else-if="item.node.truncated" class="text-[10px] text-amber-400 shrink-0" title="条目超过上限，该目录无法整体勾选导出">截断</span>
          </div>
        </div>

        <p v-if="collectionError" class="text-[11px] text-red-400">{{ collectionError }}</p>

        <div class="flex gap-2 pt-3 border-t border-[#353638]">
          <BedrockButton type="button" variant="grey" size="sm" class="flex-1" @click="emit('close')">
            取消
          </BedrockButton>
          <BedrockButton type="submit" variant="green" size="sm" class="flex-1" :disabled="collecting">
            {{ collecting ? '正在收集选择...' : '开始导出' }}
          </BedrockButton>
        </div>
      </form>
    </div>
  </div>
</template>