# 快速开始

## 一键启动（推荐）

### Windows
双击运行 `start.bat`

### Linux/macOS
```bash
chmod +x start.sh
./start.sh
```

## 手动启动

### 1. 安装依赖
```bash
npm install
```

### 2. 开发模式运行
```bash
npm run tauri dev
```

### 3. 构建生产版本
```bash
npm run tauri build
```

## 系统要求检查

运行启动脚本会自动检查以下工具：
- ✅ Node.js
- ✅ Rust/Cargo
- ✅ FFmpeg

如果缺少任何工具，脚本会提示安装方法。

## 首次使用

### 1. 打开视频
点击"打开视频"按钮，选择一个视频文件

### 2. 设置裁剪范围
- 播放视频到开始位置，点击"设为当前位置"（开始时间）
- 播放到结束位置，点击"设为当前位置"（结束时间）

### 3. 保存视频
点击"保存视频"或"另存为..."按钮

## 文件结构

```
video-rough-cut/
├── start.bat          # Windows 启动脚本
├── start.sh           # Linux/macOS 启动脚本
├── package.json       # 项目配置
├── README.md          # 项目说明
├── DEPLOYMENT.md      # 部署指南
├── USERGUIDE.md       # 用户指南
├── QUICKSTART.md      # 本文件
├── src/               # 前端源码
│   ├── App.vue        # 主应用组件
│   └── main.ts        # 入口文件
└── src-tauri/         # 后端源码
    ├── src/main.rs    # Rust 主程序
    └── tauri.conf.json # Tauri 配置
```

## 常用命令

| 命令 | 说明 |
|------|------|
| `npm run dev` | 启动前端开发服务器 |
| `npm run build` | 构建前端 |
| `npm run tauri dev` | 启动完整应用（开发模式）|
| `npm run tauri build` | 构建生产版本 |

## 下一步

- 查看 [用户指南](USERGUIDE.md) 了解详细使用方法
- 查看 [部署指南](DEPLOYMENT.md) 了解如何打包和分发
- 查看 [README.md](README.md) 了解项目详情

## 遇到问题？

### FFmpeg 未找到
- Windows: 下载并添加到 PATH
- macOS: `brew install ffmpeg`
- Linux: `sudo apt install ffmpeg`

### Rust 未安装
访问 https://rustup.rs 安装 Rust

### 其他问题
查看 [DEPLOYMENT.md](DEPLOYMENT.md) 中的常见问题解答