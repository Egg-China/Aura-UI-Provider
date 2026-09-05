# Aura 双 UI 架构与 Modern UI 交付计划

## 1. 总体架构

Aura Launcher 保留 Java 与 C/C++/Rust 双 UI 架构：

```
Aura Launcher (JVM, GPL 谱系)
├── 老 UI: HMCL UI (JavaFX, 内建, 永远可用作恢复界面)
├── 新 UI: Modern UI (Tauri 2 + Vue 3, 独立 .npl UI-Provider 插件)
│          └── 默认主题 = AuraUI 移植 (React 原型 -> Vue 3 重写, Apache-2.0)
└── 预留: Qt 前端接口 (仅协议适配层占位, 不随产品分发)
```

- JavaFX 内建不可卸载，是兜底与救援 UI（`--ui=javafx` 已支持持久恢复）。
- 新 UI 通过 `aura.ui.v1`（ABI 1）进程协议与 JVM 通信，崩溃自动回退 JavaFX。
- Qt 只保留协议层接口：`UiFrontendDescriptor` 已是纯数据契约，任何能实现
  `aura.ui.v1` 帧协议 + `--stdio` 进程模型的 native 技术都可接入。

## 2. 插件 UI 注册（新老 UI 统一贡献面）

- 老 UI（JavaFX）：`PluginUIRegistry` 已支持权限校验后的 sidebar 项/页注册；
  本阶段扩展按钮级注册（页面内动作按钮）并保持代际撤销。
- 新 UI（Vue）：插件贡献通过协议下发——Launcher 在 `ui.snapshot.replace`
  中携带 `pluginContributions`（sidebar 项 / 页面路由 / 动作按钮声明），
  Vue 侧动态渲染并在触发时回调 `core.plugin.action`。
- 同一插件可同时向两个 UI 注册；注销时两端同步移除。

## 3. AuraUI -> Vue 3 移植（frontend/）

React 19 + TS + Tailwind 4 原型按页重写为 Vue 3 `<script setup lang="ts">`：

| 波次 | 内容 | 状态 |
|---|---|---|
| W2 | 脚手架 + 设计系统 CSS + App 壳 + TitleBar/Sidebar/背景 + HomePage | 进行中 |
| W3 | InstancesPage + NewInstanceModal + LaunchModal | 待办 |
| W4 | SettingsPage 全量完善（见 §4）+ General/Appearance/About | 待办 |
| W5 | DownloadPage + Java 管理页 + ModsPage | 待办 |
| W6 | PluginsPage（真实 PluginManager 状态）+ Multiplayer + Console + Account 流程 | 待办 |
| W7 | Tauri 2 壳（无边框窗口、自定义标题栏、协议线程 -> Vue IPC） | 待办 |
| W8 | 插件贡献渲染 + core.* 命令面（snapshot/settings/instances/accounts） | 待办 |

## 4. 设置项完善清单（对照 HMCL 全量）

现有原型约 30 项；补齐至 HMCL 对等：

- 全局游戏：版本隔离策略、内存百分比模式、环境变量、服务器地址预填、
  游戏语言、文件校验策略（关/快速/全量）、跳过完整性检查、模组加载器默认、
  native GLFW/OpenAL、LWJGL 修复、崩溃报告上传
- Java：独立管理页（扫描/自动下载 JDK/供应商与架构过滤）、强制版本、
  服务端/客户端 JVM 偏好
- 下载：镜像源顺序与自定义源、BMCLAPI 凭据、代理认证（用户名/密码）、
  下载前确认、自动重试策略
- 常规：语言全量（zh_CN/zh_TW/lzh/en/ja/ru/uk/es/ar）、更新通道+跳过版本、
  **UI 前端选择**（selectedUiFrontend，重启生效）、旧版数据导入、
  日志导出/打开日志目录、临时目录检查、四月愚人节开关
- 个性化：HMCL 主题包系统、背景（内置/自定义/网络/视频）、背景模糊与透明度、
  字体家族/字号、动画与减少动态、缩放系数
- 账户：微软 OAuth、authlib-injector 服务器管理、离线档案皮肤
- 关于：版本/开源许可/更新检查/诊断导出

## 5. Release 分发策略

- 官方 Release 仅分发**支持完善**的平台：windows-x64、linux-x64、
  macos-x64、macos-arm64（Rust/Tauri 一线支持 + CI 全量测试）。
- 缺乏支持或停止维护的平台（arm32、riscv64、loongarch64、freebsd、
  HarmonyOS 等）：不提供官方构建，README 提供 `cargo build --release`
  + 打包脚本自助编译指引；不接受这些平台的构建缺陷报告。
- 每 Release 附 SHA-256 清单；`.npl` 与 Launcher 版本用 `launcherVersion`
  约束对齐。

## 6. 许可

- 本仓库（协议壳 + Vue 重写）：Apache-2.0（见 LICENSE、NOTICE）。
- 重写不得复制 HMCL GPL 源码；视觉设计沿用自有 AuraUI 原型。
