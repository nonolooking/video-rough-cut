# 部署指南

## 环境要求

### 开发环境
- Node.js 16+
- Rust 1.70+
- FFmpeg
- npm 或 yarn

### 目标平台
- Windows 10/11
- macOS 10.15+

## 安装 FFmpeg

### Windows
```powershell
# 方法1: 使用 Chocolatey
choco install ffmpeg

# 方法2: 手动安装
# 1. 下载: https://www.gyan.dev/ffmpeg/builds/
# 2. 解压到 C:\ffmpeg
# 3. 添加到 PATH:
#    setx PATH "%PATH%;C:\ffmpeg\bin"
```

### macOS
```bash
brew install ffmpeg
```

### Linux
```bash
sudo apt update
sudo apt install ffmpeg
```

## 开发运行

### Linux/macOS
```bash
chmod +x start.sh
./start.sh
```

### Windows
```bash
start.bat
```

或手动运行：
```bash
npm install
npm run tauri dev
```

## 构建生产版本

### 构建应用
```bash
npm run tauri build
```

### 构建产物位置
- **Windows**: `src-tauri/target/release/bundle/msi/视频粗剪工具_1.0.0_x64.msi`
- **macOS**: `src-tauri/target/release/bundle/dmg/视频粗剪工具_1.0.0_x64.dmg`

## 安装和文件关联

### Windows
1. 双击 `.msi` 安装文件
2. 安装完成后，右键点击视频文件
3. 选择"打开方式" → "选择其他应用"
4. 勾选"始终使用此应用打开 .mp4 文件"
5. 选择"视频粗剪工具"

### macOS
1. 打开 `.dmg` 文件
2. 将应用拖入 Applications 文件夹
3. 右键点击视频文件 → 显示简介
4. 在"打开方式"中选择"视频粗剪工具"
5. 点击"全部更改"

## 使用流程

### 方式一：通过应用打开
1. 启动应用
2. 点击"打开视频"按钮
3. 选择视频文件
4. 进行编辑和保存

### 方式二：通过文件管理器
1. 在文件管理器中找到视频文件
2. 右键点击 → 打开方式 → 视频粗剪工具
3. 应用自动加载视频文件
4. 进行编辑和保存

## 常见问题

### Q: 提示"无法执行 FFmpeg"
**A:** 确保 FFmpeg 已安装并添加到系统 PATH

### Q: 视频处理失败
**A:** 检查视频文件格式是否支持，尝试使用常见格式（MP4、AVI、MOV）

### Q: Windows 上应用无法启动
**A:** 
1. 检查是否安装了 Visual C++ Redistributable
2. 以管理员身份运行应用

### Q: macOS 上提示"无法验证开发者"
**A:** 
```bash
sudo xattr -d com.apple.quarantine /Applications/视频粗剪工具.app
```

## 性能优化建议

1. **使用 SSD**: 视频处理速度会更快
2. **足够内存**: 建议至少 8GB RAM
3. **关闭后台应用**: 释放系统资源
4. **使用快速编解码器**: FFmpeg 默认使用 libx264，可在代码中调整

## 自定义配置

### 修改视频质量
编辑 `src-tauri/src/main.rs`，修改 FFmpeg 参数：

```rust
.args(&[
    "-y",
    "-i", &input_path,
    "-ss", &format!("{}", start_time),
    "-t", &format!("{}", duration),
    "-c:v", "libx264",
    "-crf", "23",           // 添加质量控制（18-28，越小质量越好）
    "-preset", "medium",    // 添加编码速度设置
    "-c:a", "aac",
    "-b:a", "192k",         // 音频比特率
    &output_str
])
```

### 添加新功能
1. 前端：修改 `src/App.vue`
2. 后端：修改 `src-tauri/src/main.rs`
3. 配置：修改 `src-tauri/tauri.conf.json`

## 技术支持

如有问题，请提交 Issue：
https://github.com/your-username/video-rough-cut/issues