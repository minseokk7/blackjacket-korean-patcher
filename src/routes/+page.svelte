<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  interface StatusResponse {
    game_dir: string;
    is_valid: boolean;
    has_backup: boolean;
    is_patched: boolean;
    is_auto_detected: boolean;
  }

  let status = $state<StatusResponse>({
    game_dir: "",
    is_valid: false,
    has_backup: false,
    is_patched: false,
    is_auto_detected: false,
  });

  let isLoading = $state(true);
  let isPatching = $state(false);
  let isRestoring = $state(false);
  let message = $state<string | null>(null);
  let messageType = $state<"success" | "error" | "info">("info");

  async function refreshStatus() {
    isLoading = true;
    try {
      status = await invoke<StatusResponse>("get_status");
    } catch (e: any) {
      showNotice(typeof e === "string" ? e : "상태 확인 중 오류가 발생했습니다.", "error");
    } finally {
      isLoading = false;
    }
  }

  async function handleSelectFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Black Jacket 게임 설치 폴더 (Content) 선택",
      });

      if (selected && typeof selected === "string") {
        isLoading = true;
        status = await invoke<StatusResponse>("set_game_dir", { path: selected });
        showNotice("게임 경로가 성공적으로 설정되었습니다.", "success");
      }
    } catch (e: any) {
      showNotice(typeof e === "string" ? e : "폴더 설정 중 오류가 발생했습니다.", "error");
    } finally {
      isLoading = false;
    }
  }

  async function handleApplyPatch() {
    if (!status.is_valid) {
      showNotice("유효한 Black Jacket 게임 설치 경로를 먼저 지정해주세요.", "error");
      return;
    }

    isPatching = true;
    try {
      const res = await invoke<string>("apply_korean_patch");
      showNotice(res, "success");
      await refreshStatus();
    } catch (e: any) {
      showNotice(typeof e === "string" ? e : "패치 적용 실패", "error");
    } finally {
      isPatching = false;
    }
  }

  async function handleRestoreOriginal() {
    if (!status.has_backup) {
      showNotice("복원할 수 있는 원본 백업 파일이 없습니다.", "error");
      return;
    }

    if (!confirm("정말 순정 원본 상태로 복원하시겠습니까?")) {
      return;
    }

    isRestoring = true;
    try {
      const res = await invoke<string>("restore_original");
      showNotice(res, "success");
      await refreshStatus();
    } catch (e: any) {
      showNotice(typeof e === "string" ? e : "복원 실패", "error");
    } finally {
      isRestoring = false;
    }
  }

  async function handleOpenFolder() {
    try {
      await invoke("open_game_folder");
    } catch (e: any) {
      showNotice(typeof e === "string" ? e : "폴더 열기 실패", "error");
    }
  }

  async function handleLaunchGame() {
    try {
      await invoke("launch_game");
      showNotice("Black Jacket 게임을 실행했습니다.", "info");
    } catch (e: any) {
      showNotice(typeof e === "string" ? e : "게임 실행 실패", "error");
    }
  }

  function showNotice(text: string, type: "success" | "error" | "info") {
    message = text;
    messageType = type;
  }

  onMount(() => {
    refreshStatus();
  });
</script>

<main class="h-full w-full p-6 flex flex-col justify-between select-none">
  <!-- 상단 헤더 영역 -->
  <header class="flex items-center justify-between border-b border-slate-800/80 pb-4">
    <div class="flex items-center gap-3">
      <div class="w-11 h-11 rounded-xl bg-gradient-to-br from-amber-500 via-rose-600 to-purple-700 p-[1px] shadow-lg shadow-rose-950/40">
        <div class="w-full h-full bg-slate-950/90 rounded-[11px] flex items-center justify-center font-black text-amber-400 text-xl tracking-tighter">
          BJ
        </div>
      </div>
      <div>
        <div class="flex items-center gap-2">
          <h1 class="text-xl font-extrabold tracking-wide text-transparent bg-clip-text bg-gradient-to-r from-amber-200 via-rose-200 to-purple-200">
            BLACK JACKET
          </h1>
          <span class="px-2 py-0.5 text-[11px] font-semibold bg-rose-950/80 text-rose-300 border border-rose-800/60 rounded-full">
            v1.0
          </span>
        </div>
        <p class="text-xs text-slate-400 font-medium">완전 한국어 로컬라이징 공식 패치 프로그램</p>
      </div>
    </div>

    <!-- 호환 플랫폼 배지 -->
    <div class="flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-slate-900/80 border border-slate-800 text-[11px] text-slate-300 font-medium">
      <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
      Xbox Game Pass & PC 지원
    </div>
  </header>

  <!-- 본문 대시보드 카드 영역 -->
  <div class="my-4 flex flex-col gap-3.5 flex-1 overflow-y-auto pr-1">
    <!-- 1. 게임 설치 경로 카드 -->
    <div class="p-4 rounded-xl bg-slate-900/60 border border-slate-800/80 shadow-md backdrop-blur-sm">
      <div class="flex items-center justify-between mb-2">
        <span class="text-xs font-semibold text-slate-300 flex items-center gap-1.5">
          <svg class="w-4 h-4 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"></path>
          </svg>
          게임 설치 경로 (Content 폴더)
        </span>
        {#if status.is_auto_detected}
          <span class="text-[10px] text-emerald-400 bg-emerald-950/50 border border-emerald-800/50 px-2 py-0.5 rounded-full font-medium">
            자동 감지 완료
          </span>
        {/if}
      </div>

      <div class="flex items-center gap-2">
        <input
          type="text"
          readonly
          value={status.game_dir || "게임 경로를 검색하는 중..."}
          class="flex-1 px-3 py-2 text-xs bg-slate-950/80 border border-slate-800 rounded-lg text-slate-200 font-mono focus:outline-none select-all"
        />
        <button
          onclick={handleSelectFolder}
          class="px-3 py-2 bg-slate-800 hover:bg-slate-700 active:bg-slate-900 text-xs font-medium text-slate-200 rounded-lg transition-colors border border-slate-700/60 whitespace-nowrap cursor-pointer"
        >
          찾아보기...
        </button>
        <button
          onclick={handleOpenFolder}
          disabled={!status.is_valid}
          class="px-3 py-2 bg-slate-800/50 hover:bg-slate-700/60 disabled:opacity-40 text-xs font-medium text-slate-300 rounded-lg transition-colors border border-slate-700/40 whitespace-nowrap cursor-pointer"
          title="윈도우 탐색기에서 열기"
        >
          열기
        </button>
      </div>

      <!-- 경로 검증 상태 -->
      <div class="mt-2.5 flex items-center gap-2 text-xs">
        {#if status.is_valid}
          <div class="flex items-center gap-1.5 text-emerald-400 font-medium">
            <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M5 13l4 4L19 7"></path>
            </svg>
            올바른 Black Jacket 게임 파일이 확인되었습니다.
          </div>
        {:else}
          <div class="flex items-center gap-1.5 text-rose-400 font-medium">
            <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M6 18L18 6M6 6l12 12"></path>
            </svg>
            게임을 찾을 수 없습니다. [찾아보기]를 눌러 게임 Content 폴더를 선택해주세요.
          </div>
        {/if}
      </div>
    </div>

    <!-- 2. 패치 및 백업 상태 2열 그리드 -->
    <div class="grid grid-cols-2 gap-3.5">
      <!-- 패치 상태 카드 -->
      <div class="p-3.5 rounded-xl bg-slate-900/60 border border-slate-800/80 shadow-md">
        <span class="text-xs text-slate-400 font-medium block mb-1.5">현재 패치 상태</span>
        <div class="flex items-center gap-2">
          {#if status.is_patched}
            <span class="w-2.5 h-2.5 rounded-full bg-emerald-400 shadow-sm shadow-emerald-400/50"></span>
            <span class="text-sm font-bold text-emerald-300">한글 패치 적용됨</span>
          {:else}
            <span class="w-2.5 h-2.5 rounded-full bg-amber-400 shadow-sm shadow-amber-400/50"></span>
            <span class="text-sm font-bold text-amber-300">미적용 (순정 상태)</span>
          {/if}
        </div>
      </div>

      <!-- 원본 백업 카드 -->
      <div class="p-3.5 rounded-xl bg-slate-900/60 border border-slate-800/80 shadow-md">
        <span class="text-xs text-slate-400 font-medium block mb-1.5">순정 원본 백업</span>
        <div class="flex items-center gap-2">
          {#if status.has_backup}
            <span class="w-2.5 h-2.5 rounded-full bg-cyan-400 shadow-sm shadow-cyan-400/50"></span>
            <span class="text-sm font-bold text-cyan-300">백업 보존됨 (롤백 가능)</span>
          {:else}
            <span class="w-2.5 h-2.5 rounded-full bg-slate-500"></span>
            <span class="text-sm font-bold text-slate-400">백업 없음 (패치 시 자동 생성)</span>
          {/if}
        </div>
      </div>
    </div>

    <!-- 3. 패치 특징 & 언어 설정 가이드 -->
    <div class="p-4 rounded-xl bg-gradient-to-br from-slate-900/80 to-slate-950 border border-slate-800/60 text-xs text-slate-300 leading-relaxed space-y-2">
      <div class="flex items-center gap-1.5 font-bold text-amber-300">
        <svg class="w-4 h-4 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
        </svg>
        한글 패치 적용 후 게임 내 언어 설정 방법
      </div>
      <p class="text-slate-300 pl-5">
        게임 실행 후 <span class="text-white font-semibold">[Settings (설정)]</span> ➔
        <span class="text-white font-semibold">[Display Language (표시 언어)]</span> 옵션에서
        <span class="text-amber-300 font-bold">'Korean'</span>을 선택하시면 모든 카지노 카드/퍽/대사/UI가 즉시 100% 한글로 출력됩니다!
      </p>
      <div class="pt-2 border-t border-slate-800/80 grid grid-cols-2 gap-2 text-[11px] text-slate-400 pl-5">
        <div>• 80개 테이블 2,200여 개 전체 텍스트 번역</div>
        <div>• 맑은 고딕 기반 동적 SDF 고해상도 폰트</div>
        <div>• 원본 영어와 한글 자유로운 토글 지원</div>
        <div>• 원클릭 자동 백업 및 무손실 롤백 지원</div>
      </div>
    </div>

    <!-- 알림 메시지 영역 -->
    {#if message}
      <div
        class="p-3 rounded-lg text-xs font-medium border flex items-center justify-between {
          messageType === 'success'
            ? 'bg-emerald-950/60 text-emerald-200 border-emerald-800/60'
            : messageType === 'error'
              ? 'bg-rose-950/60 text-rose-200 border-rose-800/60'
              : 'bg-cyan-950/60 text-cyan-200 border-cyan-800/60'
        }"
      >
        <span>{message}</span>
        <button onclick={() => (message = null)} class="text-slate-400 hover:text-white ml-2 cursor-pointer">✕</button>
      </div>
    {/if}
  </div>

  <!-- 하단 액션 버튼 영역 -->
  <footer class="pt-3 border-t border-slate-800/80 flex items-center gap-3">
    <!-- 원본 복원 버튼 -->
    <button
      onclick={handleRestoreOriginal}
      disabled={!status.has_backup || isRestoring || isPatching}
      class="px-4 py-3 rounded-xl bg-slate-900 hover:bg-slate-800 disabled:opacity-40 text-xs font-semibold text-rose-300 border border-rose-900/50 transition-all flex items-center gap-1.5 cursor-pointer whitespace-nowrap"
    >
      {#if isRestoring}
        <span class="w-3.5 h-3.5 border-2 border-rose-400 border-t-transparent rounded-full animate-spin"></span>
        복원 중...
      {:else}
        <svg class="w-4 h-4 text-rose-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
        </svg>
        순정 원본 복원
      {/if}
    </button>

    <!-- 한글 패치 적용 버튼 (메인) -->
    <button
      onclick={handleApplyPatch}
      disabled={!status.is_valid || isPatching || isRestoring}
      class="flex-1 py-3 px-5 rounded-xl bg-gradient-to-r from-amber-500 via-rose-600 to-purple-600 hover:from-amber-400 hover:via-rose-500 hover:to-purple-500 active:scale-[0.98] disabled:opacity-40 disabled:hover:scale-100 text-sm font-extrabold text-white shadow-lg shadow-rose-950/60 transition-all flex items-center justify-center gap-2 cursor-pointer"
    >
      {#if isPatching}
        <span class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></span>
        한글 패치 설치 중...
      {:else}
        <svg class="w-5 h-5 text-amber-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"></path>
        </svg>
        {status.is_patched ? "한글 패치 다시 적용하기" : "Black Jacket 한글 패치 적용"}
      {/if}
    </button>

    <!-- 게임 실행 버튼 -->
    <button
      onclick={handleLaunchGame}
      disabled={!status.is_valid}
      class="px-4 py-3 rounded-xl bg-emerald-600/80 hover:bg-emerald-500 text-xs font-semibold text-white shadow-md shadow-emerald-950/40 transition-all flex items-center gap-1.5 cursor-pointer whitespace-nowrap disabled:opacity-40"
      title="Black Jacket 게임 즉시 실행"
    >
      <svg class="w-4 h-4 text-emerald-200" fill="currentColor" viewBox="0 0 24 24">
        <path d="M8 5v14l11-7z"/>
      </svg>
      게임 실행
    </button>
  </footer>
</main>
