@echo off
chcp 65001 >nul
echo ==========================================
echo    视频粗剪工具 - 开发模式启动
echo ==========================================
echo.

REM 检查 FFmpeg
where ffmpeg >nul 2>nul
if %errorlevel% neq 0 (
    echo ❌ 错误: 未找到 FFmpeg
    echo 请先安装 FFmpeg:
    echo   1. 下载: https://ffmpeg.org/download.html
    echo   2. 解压到任意目录（如 C:\ffmpeg）
    echo   3. 将 C:\ffmpeg\bin 添加到系统 PATH
    pause
    exit /b 1
)

echo ✅ FFmpeg 已安装
echo.

REM 检查 Rust
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo ❌ 错误: 未找到 Rust/Cargo
    echo 请先安装 Rust: https://rustup.rs
    pause
    exit /b 1
)

echo ✅ Rust 已安装
echo.

REM 进入项目目录
cd /d "%~dp0"

REM 检查 node_modules
if not exist "node_modules" (
    echo 📦 安装依赖...
    call npm install
)

echo 🚀 启动开发服务器...
echo.
call npm run tauri dev
pause