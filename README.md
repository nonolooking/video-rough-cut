# 视频粗剪工具

一个跨平台的视频粗剪工具，支持 Windows 和 macOS，可以直接在文件管理器中通过右键菜单或文件关联快速打开视频进行编辑。

## 功能特性

- 🎬 视频预览和播放控制
- ✂️ 简单的时间线裁剪功能
- 💾 支持保存或另存为新视频
- 🖱️ 文件管理器集成（右键菜单）
- 🎨 现代化用户界面
- ⚡ 轻量级应用，启动快速

## 技术栈

- **前端**: Vue 3 + TypeScript
- **后端**: Tauri (Rust)
- **视频处理**: FFmpeg

## 安装要求

### 系统要求
- Windows 10/11 或 macOS 10.15+
- FFmpeg（必须）

### 安装 FFmpeg

#### Windows
1. 下载 FFmpeg: https://ffmpeg.org/download.html
2. 解压到任意目录（例如 `C:\ffmpeg`）
3. 将 `C:\ffmpeg\bin` 添加到系统环境变量 PATH

#### macOS
```bash
brew install ffmpeg
```

## 开发指南

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run tauri dev
```

### 构建应用

```bash
npm run tauri build
```

构建产物位于 `src-tauri/target/release/bundle/` 目录。

## 使用说明

### 打开视频
1. 点击"打开视频"按钮选择视频文件
2. 或者直接双击视频文件（需要先设置文件关联）

### 编辑视频
1. 使用播放器预览视频
2. 点击时间线跳转到指定位置
3. 设置开始时间和结束时间
4. 点击"设为当前位置"按钮快速标记

### 保存视频
- **保存视频**: 在原文件同目录保存为 `原文件名_edited.mp4`
- **另存为**: 保存为新文件，文件名包含时间戳

## 文件关联设置

### Windows
安装应用后，右键点击视频文件 → 打开方式 → 选择其他应用 → 选择视频粗剪工具

### macOS
右键点击视频文件 → 显示简介 → 打开方式 → 选择视频粗剪工具 → 点击"全部更改"

## 支持的视频格式

- MP4
- AVI
- MOV
- MKV
- WMV
- FLV
- WebM

## 项目结构

```
video-rough-cut/
├── src/              # Vue 前端代码
│   ├── App.vue       # 主应用组件
│   └── main.ts       # 入口文件
├── src-tauri/        # Tauri 后端代码
│   ├── src/
│   │   └── main.rs   # Rust 主程序
│   ├── Cargo.toml    # Rust 依赖
│   └── tauri.conf.json # Tauri 配置
├── package.json
└── README.md
```

## 许可证

MIT License