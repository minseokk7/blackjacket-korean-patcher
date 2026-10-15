mod detector;
mod patcher;

use patcher::PatchManager;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use tauri::State;

#[derive(Default)]
pub struct AppState {
    pub game_dir: Mutex<Option<PathBuf>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StatusResponse {
    pub game_dir: String,
    pub is_valid: bool,
    pub has_backup: bool,
    pub is_patched: bool,
    pub is_auto_detected: bool,
}

fn get_effective_game_dir(state: &AppState) -> (PathBuf, bool) {
    let mut lock = state.game_dir.lock().unwrap();
    if let Some(ref dir) = *lock {
        return (dir.clone(), false);
    }
    if let Some(detected) = detector::detect_game_dir() {
        *lock = Some(detected.clone());
        return (detected, true);
    }
    let fallback = PathBuf::from(r"D:\xbox\Black Jacket\Content");
    *lock = Some(fallback.clone());
    (fallback, false)
}

#[tauri::command]
fn get_status(state: State<AppState>) -> StatusResponse {
    let (game_dir, is_auto) = get_effective_game_dir(&state);
    let resolved = detector::resolve_content_dir(&game_dir);
    let is_valid = detector::is_valid_game_dir(&resolved);

    let pm = PatchManager::new(resolved);
    let has_backup = pm.has_backup();
    let is_patched = pm.is_patched();

    StatusResponse {
        game_dir: game_dir.to_string_lossy().to_string(),
        is_valid,
        has_backup,
        is_patched,
        is_auto_detected: is_auto,
    }
}

#[tauri::command]
fn set_game_dir(state: State<AppState>, path: String) -> Result<StatusResponse, String> {
    let p = PathBuf::from(&path);
    if !p.exists() {
        return Err("존재하지 않는 폴더 경로입니다.".into());
    }
    let resolved = detector::resolve_content_dir(&p);
    if !detector::is_valid_game_dir(&resolved) {
        return Err("Black Jacket 게임 실행 파일 또는 데이터 폴더를 찾을 수 없습니다.".into());
    }
    *state.game_dir.lock().unwrap() = Some(resolved);
    Ok(get_status(state))
}

#[tauri::command]
fn apply_korean_patch(state: State<AppState>) -> Result<String, String> {
    let (game_dir, _) = get_effective_game_dir(&state);
    let resolved = detector::resolve_content_dir(&game_dir);

    if !detector::is_valid_game_dir(&resolved) {
        return Err("유효한 Black Jacket 게임 폴더가 아닙니다. 경로를 다시 확인해주세요.".into());
    }

    let pm = PatchManager::new(resolved);
    let count = pm.apply_patch()?;

    Ok(format!(
        "Black Jacket 완전 한국어 패치 v1.0 적용이 성공적으로 완료되었습니다! (총 {}개 파일 설치)",
        count
    ))
}

#[tauri::command]
fn restore_original(state: State<AppState>) -> Result<String, String> {
    let (game_dir, _) = get_effective_game_dir(&state);
    let resolved = detector::resolve_content_dir(&game_dir);

    let pm = PatchManager::new(resolved);
    pm.restore_original()?;

    Ok("게임 순정 원본 상태로 복원이 완료되었습니다.".into())
}

#[tauri::command]
fn open_game_folder(state: State<AppState>) -> Result<(), String> {
    let (game_dir, _) = get_effective_game_dir(&state);
    let resolved = detector::resolve_content_dir(&game_dir);
    Command::new("explorer")
        .arg(&resolved)
        .spawn()
        .map_err(|e| format!("폴더 열기 실패: {}", e))?;
    Ok(())
}

#[tauri::command]
fn launch_game(state: State<AppState>) -> Result<(), String> {
    let (game_dir, _) = get_effective_game_dir(&state);
    let resolved = detector::resolve_content_dir(&game_dir);

    let launcher = if resolved.join("gamelaunchhelper.exe").exists() {
        resolved.join("gamelaunchhelper.exe")
    } else if resolved.join("BlackJacket.exe").exists() {
        resolved.join("BlackJacket.exe")
    } else {
        return Err("게임 실행 파일(gamelaunchhelper.exe / BlackJacket.exe)을 찾을 수 없습니다.".into());
    };

    Command::new(&launcher)
        .current_dir(&resolved)
        .spawn()
        .map_err(|e| format!("게임 실행 실패: {}", e))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_status,
            set_game_dir,
            apply_korean_patch,
            restore_original,
            open_game_folder,
            launch_game
        ])
        .run(tauri::generate_context!())
        .expect("Tauri 앱 실행 중 오류 발생");
}
