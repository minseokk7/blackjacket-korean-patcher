# 🎰 Black Jacket 올인원 한글 패치 프로그램 (GUI)

Xbox Game Pass 및 PC용 **Black Jacket**을 위한 단일 실행형(All-in-One Standalone) GUI 한글 패치 도구입니다.

![기술 스택](https://img.shields.io/badge/Tauri-v2-blue?logo=tauri)
![프론트엔드](https://img.shields.io/badge/Svelte-5_(Runes)-FF3E00?logo=svelte)
![백엔드](https://img.shields.io/badge/Rust-2021-DEA584?logo=rust)
![패키지 관리자](https://img.shields.io/badge/Bun-Fast-black?logo=bun)
![스타일](https://img.shields.io/badge/TailwindCSS-v4-38B2AC?logo=tailwindcss)

---

## ✨ 핵심 기능

1. **완전 독립 단일 실행 파일 (Standalone EXE)**
   - 116MB 상당의 한글 SDF 폰트와 번역 데이터 번들(압축 60MB)이 실행 파일 내부에 내장되어 있어 별도의 압축 해제 없이 파일 하나로 즉시 동작합니다.
2. **스마트 게임 경로 자동 감지**
   - Xbox Game Pass 설치 기본 경로(`D:\xbox\Black Jacket\Content`, `C:\XboxGames\...` 등) 및 Steam 경로를 자동으로 감지합니다.
   - 상위 폴더를 수동으로 선택하더라도 내부의 `Content` 폴더를 자동으로 탐색하여 정규화합니다.
3. **무손실 원본 백업 및 원클릭 복원**
   - 패치 적용 전 기존 원본 리소스를 `BlackJacket_Data_Backup_Original`에 자동으로 안전 보관합니다.
   - 언제든지 **[순정 원본 복원]** 버튼을 통해 초기 상태로 즉시 롤백할 수 있습니다.
4. **원클릭 게임 즉시 실행**
   - 패치 완료 후 프로그램을 닫지 않고 바로 게임을 구동할 수 있습니다.
5. **모던 카지노 다크 UI**
   - Svelte 5 Runes 및 Tailwind CSS 기반의 반응형 네온 골드/레드 카지노 테마 인터페이스를 제공합니다.

---

## 🚀 빠른 시작 (사용 방법)

1. [**Releases 최신 버전**](https://github.com/minseokk7/blackjacket-korean-patcher/releases/latest)에서 `BlackJacket_한글패치_GUI.exe` 파일을 다운로드합니다.
2. `BlackJacket_한글패치_GUI.exe`를 실행합니다.
3. 감지된 게임 경로를 확인한 후 **[Black Jacket 한글 패치 적용]** 버튼을 클릭합니다.
4. 패치 완료 후 **[게임 실행하기]** 버튼을 눌러 게임에 접속합니다.
5. 게임 내 설정에서 한글을 활성화합니다:
   - **`[Settings]` ➔ `[Display Language]` ➔ `'Korean'` 선택**

---

## 🛠️ 개발 및 빌드 환경

### 요구 사항
- [Bun](https://bun.sh/)
- [Rust](https://rustup.rs/) (Cargo)
- [Microsoft Visual Studio C++ 빌드 도구](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

### 빌드 명령어

```bash
# 의존성 설치
bun install

# 개발 모드 실행
bun run tauri dev

# 프로덕션 단일 바이너리 릴리즈 빌드
bun run tauri build
```

빌드 결과물은 `src-tauri/target/release/blackjacket-patcher.exe`에 생성됩니다.

---

## ⚖️ 라이선스

본 프로젝트는 개인 편의를 위한 비공식 한글 패치 지원 도구입니다. 게임 리소스의 저작권은 해당 개발사 및 배급사에 있습니다.
