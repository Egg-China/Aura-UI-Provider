export type NavTab = 'home' | 'instances' | 'download' | 'plugins' | 'settings' | 'multiplayer' | 'console';

export type ModLoader = 'Vanilla' | 'Fabric' | 'Forge' | 'NeoForge' | 'Quilt';

export interface MinecraftInstance {
  id: string;
  name: string;
  version: string;
  loader: ModLoader;
  loaderVersion?: string;
  icon: string;
  lastPlayed: string;
  playTime: string;
  modCount: number;
  bannerImage?: string;
  description: string;
  isFavorite?: boolean;
  javaVersion: string;
  memoryMin: number;
  memoryMax: number;
}

export interface Account {
  id: string;
  username: string;
  uuid: string;
  type: 'microsoft' | 'thirdparty' | 'offline';
  skinUrl: string;
  isActive: boolean;
  authServer?: string;
}

export interface LauncherPlugin {
  id: string;
  name: string;
  version: string;
  author: string;
  category: 'system' | 'theme' | 'multiplayer' | 'tool';
  description: string;
  icon: string;
  enabled: boolean;
  installed: boolean;
  sourceUrl?: string;
  status: 'Running' | 'Active' | 'Disabled';
  downloads?: string;
}

export interface ModItem {
  id: string;
  name: string;
  summary: string;
  author: string;
  downloads: string;
  category: string;
  iconUrl: string;
  installed: boolean;
  enabled: boolean;
  version: string;
  loaders: ModLoader[];
}

export interface NewsItem {
  id: string;
  title: string;
  date: string;
  tag: string;
  tagColor: string;
  image: string;
  summary: string;
  readTime: string;
}

export interface LauncherSettings {
  // 1. 全局游戏设置 (Global Game Settings)
  autoMemory: boolean;
  maxMemoryGB: number;
  minMemoryGB: number;
  gcPreset: 'G1GC' | 'ZGC' | 'Shenandoah' | 'ParallelGC' | 'Custom';
  javaAutoDetect: boolean;
  javaPath: string;
  jvmArgs: string;
  gameResolution: { width: number; height: number };
  fullscreen: boolean;
  launcherActionAfterLaunch: 'hide' | 'keep' | 'close' | 'console';
  autoCrashReport: boolean;
  gameDir: string;

  // 2. 下载与镜像源设置 (Download & Mirror Settings)
  downloadSource: 'auto' | 'official' | 'bmclapi';
  defaultAddonSource: 'curseforge' | 'modrinth';
  autoDownloadThreads: boolean;
  downloadThreads: number;
  commonCacheDir: string;

  // 3. 启动器设置 (Launcher General Settings)
  updateChannel: 'stable' | 'dev';
  acceptPreview: boolean;
  autoCheckUpdate: boolean;
  language: 'zh_CN' | 'zh_TW' | 'en_US' | 'ja_JP' | 'ru_RU' | 'de_DE' | 'fr_FR' | 'es_ES';
  proxyType: 'none' | 'system' | 'http' | 'socks5';
  proxyHost: string;
  proxyPort: number;
  debugLog: boolean;

  // 4. 个性化与外观 (Personalization & Appearance)
  colorMode: 'dark' | 'light';
  themeAuraColor: string;
  hardwareAcceleration: boolean;
  enableDiscordRPC: boolean;

  // 5. HMCL 对等扩展（W4 补全，W8 接 core.settings 协议）
  versionIsolation: 'global' | 'isolated';
  fileVerification: boolean;
  gameEnvVars: string;
  proxyUsername: string;
  proxyPassword: string;
  selectedUiFrontend: 'javafx' | 'modern-ui';
  backgroundStyle: 'particles' | 'gradient' | 'plain';
  uiFontScale: number;
  aprilFools: boolean;
}
