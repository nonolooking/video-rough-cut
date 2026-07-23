#!/bin/bash

echo "=========================================="
echo "    视频粗剪工具 - 开发模式启动"
echo "=========================================="
echo ""

# 检查 FFmpeg
if ! command -v ffmpeg &> /dev/null; then
    echo "❌ 错误: 未找到 FFmpeg"
    echo "请先安装 FFmpeg:"
    echo "  Windows: 下载并添加到 PATH"
    echo "  macOS: brew install ffmpeg"
    echo "  Linux: sudo apt install ffmpeg"
    exit 1
fi

echo "✅ FFmpeg 已安装: $(ffmpeg -version | head -n 1)"
echo ""

# 检查 Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ 错误: 未找到 Rust/Cargo"
    echo "请先安装 Rust: https://rustup.rs"
    exit 1
fi

echo "✅ Rust 已安装: $(rustc --version)"
echo ""

# 进入项目目录
cd "$(dirname "$0")"

# 检查 node_modules
if [ ! -d "node_modules" ]; then
    echo "📦 安装依赖..."
    npm install
fi

echo "🚀 启动开发服务器..."
echo ""
npm run tauri dev