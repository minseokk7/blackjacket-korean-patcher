use std::path::{Path, PathBuf};

/// Black Jacket 게임 설치 경로를 자동 탐지합니다.
pub fn detect_game_dir() -> Option<PathBuf> {
    // 1. 일반적인 드라이브 목록
    let drives = ["D", "C", "E", "F", "G"];
    
    // 2. 검사할 상대 경로 후보들
    let candidates = [
        r"xbox\Black Jacket\Content",
        r"XboxGames\Black Jacket\Content",
        r"XboxGames\BlackJacket\Content",
        r"xbox\BlackJacket\Content",
        r"Xbox\Black Jacket\Content",
        r"Xbox\BlackJacket\Content",
        r"Games\Black Jacket\Content",
        r"Games\BlackJacket\Content",
        r"SteamLibrary\steamapps\common\Black Jacket",
        r"Program Files (x86)\Steam\steamapps\common\Black Jacket",
        r"Steam\steamapps\common\Black Jacket",
    ];

    for drive in drives {
        for cand in candidates {
            let path = PathBuf::from(format!(r"{}:\{}", drive, cand));
            if is_valid_game_dir(&path) {
                return Some(path);
            }
        }
    }

    None
}

/// 경로가 유효한 게임 디렉터리(Content 또는 루트)인지 검증합니다.
pub fn is_valid_game_dir(path: &Path) -> bool {
    if !path.exists() {
        return false;
    }

    // 1. BlackJacket_Data 폴더가 있는지 확인
    let data_dir = path.join("BlackJacket_Data");
    if data_dir.exists() && data_dir.is_dir() {
        return true;
    }

    // 2. 실행 파일 존재 확인
    if path.join("BlackJacket.exe").exists() || path.join("gamelaunchhelper.exe").exists() {
        return true;
    }

    // 3. 만약 Content 하위 폴더에 있는 경우
    let content_dir = path.join("Content");
    if content_dir.exists() && content_dir.join("BlackJacket_Data").exists() {
        return true;
    }

    false
}

/// 사용자가 상위 폴더를 선택했을 경우 실제 패치 대상(Content 폴더)으로 정규화합니다.
pub fn resolve_content_dir(path: &Path) -> PathBuf {
    if path.join("Content").join("BlackJacket_Data").exists() {
        return path.join("Content");
    }
    path.to_path_buf()
}
