<template>
  <div
    class="app"
    @dragover.prevent="dragOver = true"
    @dragleave.prevent="dragOver = false"
    @drop.prevent="handleDrop"
    :class="{ 'drag-over': dragOver }"
  >
    <!-- 简洁头部 -->
    <div class="header" v-if="videoPath">
      <div class="file-info">
        <span class="file-name">{{ fileName }}</span>
        <span class="file-meta">{{ formatTime(duration) }}</span>
      </div>
      <div class="header-actions">
        <button class="btn-icon" @click="openFile" title="打开其他视频">📂</button>
        <button class="btn-icon btn-close" @click="closeVideo" title="关闭">✕</button>
      </div>
    </div>

    <!-- 视频区域 -->
    <div class="video-area" v-if="videoPath">
      <div class="video-wrapper">
        <video
          ref="videoPlayer"
          :src="videoUrl"
          :style="{ transform: 'rotate(' + rotation + 'deg)' }"
          @loadedmetadata="onVideoLoaded"
          @timeupdate="onTimeUpdate"
          @click="togglePlay"
        ></video>
      </div>

      <!-- 自定义播放控制栏 -->
      <div class="player-controls">
        <button class="btn-play" @click="togglePlay">{{ isPlaying ? '⏸' : '▶' }}</button>
        <div class="player-progress" @click="onPlayerProgressClick">
          <div class="player-progress-bar" :style="{ width: playheadPercent + '%' }"></div>
        </div>
        <span class="player-time">{{ formatTime(currentTime) }} / {{ formatTime(duration) }}</span>
      </div>

      <!-- 快速裁剪控件 - 悬浮在视频下方 -->
      <div class="quick-controls">
        <div class="time-display">
          <span>{{ formatTime(currentTime) }}</span>
          <span class="divider">/</span>
          <span class="muted">{{ formatTime(duration) }}</span>
        </div>

        <!-- 精简时间线 -->
        <div class="mini-timeline" @click="onTimelineClick">
          <div class="track">
            <div class="selection" :style="selectionStyle"></div>
            <div class="playhead" :style="{ left: playheadPercent + '%' }"></div>
          </div>
        </div>

        <!-- 快速标记按钮 -->
        <div class="mark-buttons">
          <button
            class="mark-btn"
            :class="{ active: startTime > 0 }"
            @click="setCurrentAsStart"
          >
            <span class="label">入点</span>
            <span class="time">{{ formatTime(startTime) }}</span>
          </button>

          <div class="selection-badge" v-if="hasSelection">
            <span>{{ formatTime(endTime - startTime) }}</span>
          </div>

          <button
            class="mark-btn"
            :class="{ active: endTime < duration }"
            @click="setCurrentAsEnd"
          >
            <span class="label">出点</span>
            <span class="time">{{ formatTime(endTime) }}</span>
          </button>
        </div>

        <!-- 旋转 -->
        <div class="rotate-row">
          <button
            class="rotate-btn"
            :class="{ active: rotation > 0 }"
            @click="rotateCW"
            title="顺时针旋转90度 (R)"
          >
            <span class="rotate-icon">↻</span>
            <span class="rotate-label">旋转</span>
          </button>
          <span class="rotate-angle">{{ rotation }}°</span>
          <button
            class="rotate-btn"
            :class="{ active: rotation > 0 }"
            @click="rotateCCW"
            title="逆时针旋转90度"
          >
            <span class="rotate-icon">↺</span>
            <span class="rotate-label">反转</span>
          </button>
        </div>

        <!-- 保存按钮 -->
        <div class="save-bar">
          <button class="btn-save" @click="saveVideo">保存裁剪</button>
          <button class="btn-save-as" @click="saveVideoAs">另存为...</button>
        </div>
      </div>
    </div>

    <!-- 空状态 / 拖放区域 -->
    <div class="drop-zone" v-else>
      <div class="drop-content">
        <div class="drop-icon">🎬</div>
        <p class="drop-title">拖拽视频到此处</p>
        <p class="drop-sub">或按空格预览已选中的视频</p>
        <button class="btn-browse" @click="openFile">选择视频</button>
        <p class="drop-hint">支持 MP4, MOV, AVI, MKV 等格式</p>
      </div>
    </div>

    <!-- 拖放高亮遮罩 -->
    <div class="drag-overlay" v-if="dragOver">
      <div class="drag-content">
        <div class="drag-icon">⬇️</div>
        <p>释放以打开视频</p>
      </div>
    </div>

    <!-- 处理进度 -->
    <div class="progress-overlay" v-if="isProcessing">
      <div class="progress-content">
        <div class="spinner"></div>
        <p>正在裁剪视频...</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { open } from '@tauri-apps/api/dialog'
import { convertFileSrc, appWindow } from '@tauri-apps/api/tauri'

const videoPath = ref<string>('')
const videoUrl = ref<string>('')
const fileName = ref<string>('')
const videoPlayer = ref<HTMLVideoElement | null>(null)
const currentTime = ref(0)
const duration = ref(0)
const startTime = ref(0)
const endTime = ref(0)
const isProcessing = ref(false)
const dragOver = ref(false)
const rotation = ref(0)
const isPlaying = ref(false)

const hasSelection = computed(() => startTime.value > 0 || endTime.value < duration.value)

const playheadPercent = computed(() => {
  if (duration.value === 0) return 0
  return (currentTime.value / duration.value) * 100
})

const selectionStyle = computed(() => {
  if (duration.value === 0) return {}
  const start = (startTime.value / duration.value) * 100
  const end = (endTime.value / duration.value) * 100
  return {
    left: start + '%',
    width: (end - start) + '%'
  }
})

function formatTime(seconds: number): string {
  if (isNaN(seconds) || seconds < 0) return '00:00'
  const hrs = Math.floor(seconds / 3600)
  const mins = Math.floor((seconds % 3600) / 60)
  const secs = Math.floor(seconds % 60)
  if (hrs > 0) return `${hrs}:${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}`
  return `${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}`
}

async function loadVideo(path: string) {
  videoPath.value = path
  fileName.value = path.split(/[\\/]/).pop() || '未知文件'
  try {
    videoUrl.value = convertFileSrc(path, 'assets')
  } catch {
    videoUrl.value = convertFileSrc(path)
  }
  startTime.value = 0
  endTime.value = 0
  currentTime.value = 0
  duration.value = 0
  setTimeout(() => {
    if (videoPlayer.value) {
      videoPlayer.value.load()
    }
  }, 100)
}

async function openFile() {
  try {
    const selected = await open({
      filters: [{
        name: 'Video',
        extensions: ['mp4', 'avi', 'mov', 'mkv', 'wmv', 'flv', 'webm', 'm4v']
      }]
    })
    if (selected && typeof selected === 'string') {
      await loadVideo(selected)
    }
  } catch (error) {
    console.error('Failed to open file:', error)
  }
}

function handleDrop(event: DragEvent) {
  dragOver.value = false
  const files = event.dataTransfer?.files
  if (files && files.length > 0) {
    const file = files[0]
    if (file.type.startsWith('video/')) {
      videoPath.value = file.name
      fileName.value = file.name
      videoUrl.value = URL.createObjectURL(file)
      startTime.value = 0
      endTime.value = 0
      currentTime.value = 0
    }
  }
}

function onVideoLoaded(event: Event) {
  const video = event.target as HTMLVideoElement
  duration.value = video.duration
  endTime.value = video.duration
}

function onTimeUpdate(event: Event) {
  const video = event.target as HTMLVideoElement
  currentTime.value = video.currentTime
}

function togglePlay() {
  if (videoPlayer.value) {
    if (videoPlayer.value.paused) {
      videoPlayer.value.play()
      isPlaying.value = true
    } else {
      videoPlayer.value.pause()
      isPlaying.value = false
    }
  }
}

function onPlayerProgressClick(event: MouseEvent) {
  if (!videoPlayer.value || duration.value === 0) return
  const progress = event.currentTarget as HTMLElement
  const rect = progress.getBoundingClientRect()
  videoPlayer.value.currentTime = ((event.clientX - rect.left) / rect.width) * duration.value
}

function onTimelineClick(event: MouseEvent) {
  if (!videoPlayer.value || duration.value === 0) return
  const timeline = event.currentTarget as HTMLElement
  const rect = timeline.getBoundingClientRect()
  const pos = (event.clientX - rect.left) / rect.width
  videoPlayer.value.currentTime = pos * duration.value
}

function setCurrentAsStart() {
  if (videoPlayer.value) {
    startTime.value = videoPlayer.value.currentTime
    if (startTime.value > endTime.value) {
      endTime.value = duration.value
    }
  }
}

function setCurrentAsEnd() {
  if (videoPlayer.value) {
    endTime.value = videoPlayer.value.currentTime
    if (endTime.value < startTime.value) {
      startTime.value = 0
    }
  }
}

function closeVideo() {
  videoPath.value = ''
  videoUrl.value = ''
  fileName.value = ''
  startTime.value = 0
  endTime.value = 0
  duration.value = 0
  rotation.value = 0
}

function rotateCW() {
  rotation.value = (rotation.value + 90) % 360
}

function rotateCCW() {
  rotation.value = (rotation.value - 90 + 360) % 360
}

async function saveVideo() {
  await processVideo(false)
}

async function saveVideoAs() {
  await processVideo(true)
}

async function processVideo(saveAsNew: boolean) {
  if (!videoPath.value) return
  isProcessing.value = true
  try {
    const result = await invoke('cut_video', {
      inputPath: videoPath.value,
      startTime: startTime.value,
      endTime: endTime.value,
      saveAsNew: saveAsNew,
      rotation: rotation.value
    })
    if (result) {
      alert(`视频已保存到: ${result}`)
    }
  } catch (error) {
    alert(`处理失败: ${error}`)
  } finally {
    isProcessing.value = false
  }
}

onMounted(() => {
  // 尝试获取启动文件
  invoke<string>('get_startup_file').then(file => {
    if (file) {
      loadVideo(file)
    }
  })

  // 键盘快捷键
  document.addEventListener('keydown', (e) => {
    if (!videoPath.value) return
    if (e.key === 'r' || e.key === 'R') {
      rotateCW()
    } else if (e.key === 'Escape') {
      closeVideo()
    }
  })

  // 监听拖放
  document.addEventListener('dragover', (e) => e.preventDefault())
})
</script>

<style scoped>
.app {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: #1a1a2e;
  color: #eee;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  overflow: hidden;
}

/* ===== 头部 ===== */
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  background: rgba(0,0,0,0.3);
  border-bottom: 1px solid rgba(255,255,255,0.05);
  flex-shrink: 0;
}

.file-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.file-name {
  font-weight: 500;
  font-size: 0.95rem;
}

.file-meta {
  font-size: 0.8rem;
  color: #888;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.btn-icon {
  width: 32px;
  height: 32px;
  border-radius: 6px;
  border: none;
  background: rgba(255,255,255,0.08);
  color: #ccc;
  cursor: pointer;
  font-size: 1rem;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.btn-icon:hover {
  background: rgba(255,255,255,0.15);
  color: #fff;
}

.btn-close:hover {
  background: rgba(255,80,80,0.3);
}

/* ===== 视频区域 ===== */
.video-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.video-wrapper {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  background: #000;
}

.video-wrapper video {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  cursor: pointer;
}

/* 自定义播放控制栏 */
.player-controls {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 16px;
  background: rgba(0,0,0,0.6);
  border-top: 1px solid rgba(255,255,255,0.05);
  flex-shrink: 0;
}

.btn-play {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  border: none;
  background: rgba(255,255,255,0.12);
  color: #fff;
  cursor: pointer;
  font-size: 1rem;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  flex-shrink: 0;
}

.btn-play:hover {
  background: rgba(255,255,255,0.2);
}

.player-progress {
  flex: 1;
  height: 6px;
  background: rgba(255,255,255,0.1);
  border-radius: 3px;
  cursor: pointer;
  position: relative;
}

.player-progress-bar {
  height: 100%;
  background: #4CAF50;
  border-radius: 3px;
  transition: width 0.1s;
}

.player-time {
  font-size: 0.8rem;
  color: #aaa;
  font-family: 'SF Mono', monospace;
  white-space: nowrap;
  flex-shrink: 0;
}

/* ===== 快速控件 ===== */
.quick-controls {
  padding: 12px 16px;
  background: rgba(0,0,0,0.4);
  border-top: 1px solid rgba(255,255,255,0.05);
  display: flex;
  flex-direction: column;
  gap: 10px;
  flex-shrink: 0;
}

.time-display {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.85rem;
  font-family: 'SF Mono', monospace;
  color: #aaa;
}

.time-display .divider {
  color: #555;
}

.time-display .muted {
  color: #666;
}

/* ===== 精简时间线 ===== */
.mini-timeline {
  height: 28px;
  cursor: pointer;
  position: relative;
  display: flex;
  align-items: center;
}

.track {
  width: 100%;
  height: 6px;
  background: rgba(255,255,255,0.1);
  border-radius: 3px;
  position: relative;
}

.selection {
  position: absolute;
  top: 0;
  bottom: 0;
  background: linear-gradient(90deg, #4CAF50, #81C784);
  border-radius: 3px;
  opacity: 0.7;
}

.playhead {
  position: absolute;
  top: -4px;
  width: 3px;
  height: 14px;
  background: #fff;
  border-radius: 2px;
  transform: translateX(-50%);
}

/* ===== 标记按钮 ===== */
.mark-buttons {
  display: flex;
  align-items: center;
  gap: 8px;
}

.mark-btn {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
  padding: 8px 12px;
  border-radius: 8px;
  border: 1px solid rgba(255,255,255,0.1);
  background: rgba(255,255,255,0.05);
  color: #aaa;
  cursor: pointer;
  transition: all 0.2s;
}

.mark-btn:hover {
  background: rgba(255,255,255,0.1);
}

.mark-btn.active {
  border-color: #4CAF50;
  background: rgba(76,175,80,0.15);
  color: #81C784;
}

.mark-btn .label {
  font-size: 0.7rem;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.mark-btn .time {
  font-size: 0.85rem;
  font-family: 'SF Mono', monospace;
  font-weight: 500;
}

.selection-badge {
  padding: 4px 12px;
  background: rgba(76,175,80,0.2);
  border-radius: 12px;
  font-size: 0.8rem;
  color: #81C784;
  font-family: 'SF Mono', monospace;
  white-space: nowrap;
}

/* ===== 旋转 ===== */
.rotate-row{display:flex;align-items:center;justify-content:center;gap:16px;padding:2px 0}
.rotate-btn{display:flex;align-items:center;gap:6px;padding:6px 14px;border-radius:8px;border:1px solid rgba(255,255,255,0.1);background:rgba(255,255,255,0.05);color:#aaa;cursor:pointer;transition:all .2s;font-size:.85rem}
.rotate-btn:hover{background:rgba(255,255,255,0.1);color:#fff}
.rotate-btn.active{border-color:#2196F3;background:rgba(33,150,243,0.15);color:#64B5F6}
.rotate-icon{font-size:1.1rem;line-height:1}
.rotate-angle{font-family:'SF Mono',monospace;font-size:.95rem;color:#64B5F6;min-width:32px;text-align:center}

/* ===== 保存栏 ===== */
.save-bar {
  display: flex;
  gap: 8px;
}

.btn-save, .btn-save-as {
  flex: 1;
  padding: 10px;
  border-radius: 8px;
  border: none;
  font-size: 0.9rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-save {
  background: #4CAF50;
  color: white;
}

.btn-save:hover {
  background: #45a049;
}

.btn-save-as {
  background: rgba(255,255,255,0.08);
  color: #ccc;
}

.btn-save-as:hover {
  background: rgba(255,255,255,0.15);
  color: #fff;
}

/* ===== 拖放区域 ===== */
.drop-zone {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.drop-content {
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.drop-icon {
  font-size: 4rem;
  opacity: 0.6;
}

.drop-title {
  font-size: 1.3rem;
  font-weight: 500;
  color: #ddd;
}

.drop-sub {
  font-size: 0.9rem;
  color: #777;
}

.btn-browse {
  padding: 10px 28px;
  border-radius: 8px;
  border: 1px solid rgba(255,255,255,0.2);
  background: rgba(255,255,255,0.08);
  color: #ddd;
  cursor: pointer;
  font-size: 0.95rem;
  transition: all 0.2s;
  margin-top: 8px;
}

.btn-browse:hover {
  background: rgba(255,255,255,0.15);
  border-color: rgba(255,255,255,0.3);
}

.drop-hint {
  font-size: 0.75rem;
  color: #555;
  margin-top: 8px;
}

/* ===== 拖放遮罩 ===== */
.drag-overlay {
  position: fixed;
  inset: 0;
  background: rgba(76,175,80,0.15);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
  border: 3px dashed #4CAF50;
}

.drag-content {
  text-align: center;
}

.drag-icon {
  font-size: 3rem;
  margin-bottom: 8px;
}

.drag-content p {
  font-size: 1.1rem;
  color: #81C784;
}

/* ===== 进度遮罩 ===== */
.progress-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0,0,0,0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 200;
}

.progress-content {
  text-align: center;
}

.spinner {
  width: 48px;
  height: 48px;
  border: 3px solid rgba(255,255,255,0.1);
  border-top-color: #4CAF50;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
  margin: 0 auto 16px;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.progress-content p {
  color: #aaa;
}
</style>