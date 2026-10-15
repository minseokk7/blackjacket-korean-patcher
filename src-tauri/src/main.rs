// Windows 릴리즈 빌드 시 콘솔 창 숨김 처리
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    blackjacket_patcher_lib::run()
}
