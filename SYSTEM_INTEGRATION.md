# 系统级空格预览集成方案

## 目标体验

在文件管理器中选中视频 → 按空格 → 弹出预览窗口 → 直接编辑裁剪 → 保存

## 方案对比

### 方案一：macOS Quick Look 扩展（推荐）

**效果**：Finder中选中视频 → 按空格 → 出现预览浮窗 → 底部有编辑按钮

**实现方式**：
```
需要开发独立的 macOS Quick Look Extension
├── VideoRoughCutQLExtension/
│   ├── Info.plist          # 声明支持的视频类型
│   ├── PreviewViewController.swift  # 预览界面
│   └── ThumbnailProvider.swift      # 缩略图
```

**限制**：
- 必须用 Swift/Objective-C 开发
- 无法直接用 Tauri/Vue 技术栈
- 需要 Xcode 和 Apple Developer 账号

**参考项目**：
- https://github.com/anthonygelibert/QLColorCode
- https://github.com/sindresorhus/quick-look-plugins

---

### 方案二：Windows 预览处理程序

**效果**：资源管理器选中视频 → 按空格或右侧预览窗格 → 显示编辑界面

**实现方式**：
```
需要开发 COM 组件 Preview Handler
├── VideoRoughCutPreview/
│   ├── PreviewHandler.cs    # COM 接口实现
│   ├── PreviewControl.cs    # WPF/WinForms 界面
│   └── Registration.reg     # 注册表配置
```

**限制**：
- 必须用 C#/.NET 或 C++ 开发
- COM 组件注册复杂
- 无法直接用现有 Web 技术栈

**参考文档**：
- https://docs.microsoft.com/windows/win32/shell/preview-handlers

---

### 方案三：文件关联快速打开（当前实现）

**效果**：选中视频 → 右键"用视频粗剪打开" → 应用窗口直接加载视频

**已实现功能**：
- ✅ 命令行参数接收文件路径
- ✅ 文件关联配置（tauri.conf.json）
- ✅ 拖放文件打开
- ✅ 启动时自动加载视频

**优化方向**：
1. 窗口更小巧（类似预览浮窗）
2. 无边框窗口 + 自定义标题栏
3. ESC 键快速关闭

---

### 方案四：全局快捷键工具 + 应用（折中方案）

**效果**：选中视频 → 自定义快捷键（如 Ctrl+Shift+E）→ 弹出编辑窗口

**实现方式**：
```
1. 开发一个小型系统托盘工具
2. 监听全局快捷键
3. 获取当前选中的文件路径
4. 调用视频粗剪应用打开
```

**macOS 实现**：
```swift
// 使用 AppleScript 获取 Finder 选中文件
tell application "Finder"
    set selectedFiles to selection
    return POSIX path of (selectedFiles as alias)
end tell
```

**Windows 实现**：
```powershell
# 获取资源管理器选中文件
$shell = New-Object -ComObject Shell.Application
$folder = $shell.Namespace((Get-Process explorer | Select-Object -First 1).MainWindowHandle)
```

---

## 当前应用已支持的快速打开方式

### 1. 右键菜单打开（推荐）

**Windows**：
1. 右键点击视频文件
2. 选择"打开方式" → "视频粗剪工具"
3. 勾选"始终使用此应用"

**macOS**：
1. 右键点击视频文件 → "显示简介"
2. "打开方式"选择"视频粗剪工具"
3. 点击"全部更改"

### 2. 拖放打开
- 直接把视频文件拖到应用窗口
- 窗口高亮显示拖放区域
- 松开后立即加载

### 3. 命令行打开
```bash
# 直接传入视频路径
video-rough-cut "/path/to/video.mp4"
```

---

## 窗口优化为预览风格

### 修改 tauri.conf.json

```json
{
  "windows": [
    {
      "width": 720,
      "height": 520,
      "minWidth": 480,
      "minHeight": 360,
      "title": "视频粗剪",
      "decorations": false,      // 无边框
      "transparent": true,       // 透明背景
      "alwaysOnTop": true,       // 置顶
      "skipTaskbar": false,
      "center": true
    }
  ]
}
```

### 添加自定义标题栏（App.vue）

```vue
<template>
  <div class="app" data-tauri-drag-region>
    <!-- 自定义标题栏 -->
    <div class="titlebar" data-tauri-drag-region>
      <span class="title">视频粗剪</span>
      <div class="window-controls">
        <button @click="minimize">−</button>
        <button @click="close">×</button>
      </div>
    </div>
    ...
  </div>
</template>
```

### 添加 ESC 关闭

```typescript
import { appWindow } from '@tauri-apps/api/window'

document.addEventListener('keydown', (e) => {
  if (e.key === 'Escape') {
    appWindow.close()
  }
})
```

---

## 推荐实现路线

### Phase 1: 当前已实现 ✅
- 文件关联 + 命令行参数
- 拖放打开
- 简洁预览界面

### Phase 2: 窗口优化
- 无边框窗口
- 自定义标题栏
- ESC 关闭
- 窗口尺寸更像预览浮窗

### Phase 3: 系统级集成（可选）
- macOS: 开发 Quick Look Extension（Swift）
- Windows: 开发 Preview Handler（C#）
- 或开发全局快捷键辅助工具

---

## 使用建议

### 最快捷的使用方式

**方式1：设置默认打开程序**
1. 右键视频 → 打开方式 → 选择"视频粗剪工具"
2. 勾选"始终使用此应用"
3. 以后双击视频直接打开编辑

**方式2：发送到菜单**
1. Windows: 右键视频 → 发送到 → 视频粗剪工具
2. 需要创建快捷方式到 SendTo 文件夹

**方式3：快捷启动**
1. 固定应用到任务栏/Dock
2. 拖拽视频到图标上打开

---

## 技术限制说明

| 功能 | 能否实现 | 技术方案 | 难度 |
|------|---------|---------|------|
| 空格预览（macOS） | ✅ | Quick Look Extension | 高 |
| 空格预览（Windows） | ✅ | Preview Handler | 高 |
| 右键快速打开 | ✅ | 文件关联 | 低 |
| 拖放打开 | ✅ | 已实现 | 低 |
| 双击打开 | ✅ | 默认程序 | 低 |
| 全局快捷键 | ⚠️ | 需额外工具 | 中 |

---

## 结论

**当前应用已实现的方案**：
- ✅ 右键打开（最接近空格预览的替代方案）
- ✅ 拖放打开
- ✅ 命令行参数打开
- ✅ 简洁预览式界面

**要实现真正的"按空格预览编辑"**：
需要为 macOS 和 Windows 分别开发原生系统扩展，这超出了 Tauri 的能力范围，需要额外的原生开发工作。

**推荐用户操作**：
将"视频粗剪工具"设为视频文件的默认打开程序，这样双击视频文件就能直接打开编辑，体验最接近空格预览。