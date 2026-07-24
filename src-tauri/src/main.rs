#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::Command;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::Manager;

static STARTUP_FILE: AtomicBool = AtomicBool::new(false);

#[tauri::command]
fn check_ffmpeg() -> Result<bool, String> {
    match Command::new("ffmpeg").arg("-version").output() {
        Ok(output) if output.status.success() => Ok(true),
        _ => Ok(false),
    }
}

#[tauri::command]
async fn cut_video(
    input_path: String,
    start_time: f64,
    end_time: f64,
    save_as_new: bool,
    rotation: i32,
    _app_handle: tauri::AppHandle
) -> Result<String, String> {
    println!("[cut_video] input: {}, start: {}, end: {}, save_as_new: {}, rotation: {}",
        input_path, start_time, end_time, save_as_new, rotation);

    // 先检查 FFmpeg 是否可用
    match Command::new("ffmpeg").arg("-version").output() {
        Ok(output) if output.status.success() => {},
        _ => return Err("未检测到 FFmpeg。请先安装 FFmpeg 并添加到系统环境变量 PATH 中，然后重启本软件。\n下载地址: https://ffmpeg.org/download.html".to_string()),
    }

    let input = PathBuf::from(&input_path);
    
    if !input.exists() {
        return Err("输入文件不存在".to_string());
    }
    
    let output_path = if save_as_new {
        let parent = input.parent().unwrap_or(&std::path::Path::new("."));
        let stem = input.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("video");
        let ext = input.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("mp4");
        
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        parent.join(format!("{}_{}_edited.{}", stem, timestamp, ext))
    } else {
        let parent = input.parent().unwrap_or(&std::path::Path::new("."));
        let stem = input.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("video");
        let ext = input.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("mp4");
        parent.join(format!("{}_edited.{}", stem, ext))
    };
    
    let output_str = output_path.to_string_lossy().to_string();
    
    let duration = end_time - start_time;
    
    // 根据旋转角度构建FFmpeg滤镜参数
    // 0°=不旋转, 90°=顺时针, 180°=翻转, 270°=逆时针
    let rotate_filter = match rotation % 360 {
        90 => Some("transpose=1"),      // 顺时针90度
        180 => Some("hflip,vflip"),     // 180度
        270 => Some("transpose=2"),     // 逆时针90度(=顺时针270度)
        _ => None,                       // 0度，不旋转
    };
    
    let mut args: Vec<String> = vec![
        "-y".into(),
        "-i".into(), input_path.clone(),
        "-ss".into(), format!("{}", start_time),
        "-t".into(), format!("{}", duration),
    ];
    
    // 如果有旋转，添加视频滤镜
    if let Some(filter) = rotate_filter {
        args.push("-vf".into());
        args.push(filter.into());
    }
    
    args.extend_from_slice(&[
        "-c:v".into(), "libx264".into(),
        "-c:a".into(), "aac".into(),
        "-strict".into(), "experimental".into(),
        output_str.clone(),
    ]);
    
    println!("[cut_video] ffmpeg args: {:?}", args);

    let ffmpeg_status = Command::new("ffmpeg")
        .args(&args)
        .status();

    println!("[cut_video] ffmpeg result: {:?}", ffmpeg_status);

    match ffmpeg_status {
        Ok(status) if status.success() => {
            println!("[cut_video] success, output: {}", output_str);
            Ok(output_str)
        },
        Ok(status) => {
            println!("[cut_video] ffmpeg exited with code: {:?}", status.code());
            Err(format!("FFmpeg 处理失败 (退出码: {:?})。请检查视频文件是否损坏，或尝试其他格式。", status.code()))
        },
        Err(e) => {
            println!("[cut_video] ffmpeg execution error: {:?}", e);
            Err("无法执行 FFmpeg，请确保已安装 FFmpeg 并添加到系统环境变量 PATH 中，然后重启本软件。\n下载地址: https://ffmpeg.org/download.html".to_string())
        }
    }
}

#[tauri::command]
fn get_startup_file() -> Option<String> {
    if STARTUP_FILE.load(Ordering::SeqCst) {
        std::env::args().nth(1)
    } else {
        None
    }
}

#[tauri::command]
fn get_temp_path(file_name: String) -> String {
    let mut path = std::env::temp_dir();
    path.push(file_name);
    path.to_string_lossy().to_string()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        STARTUP_FILE.store(true, Ordering::SeqCst);
    }
    
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![cut_video, get_startup_file, get_temp_path, check_ffmpeg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}