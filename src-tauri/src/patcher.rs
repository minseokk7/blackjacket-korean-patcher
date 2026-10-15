use std::fs::{self, File};
use std::io::{self, Cursor};
use std::path::PathBuf;
use zip::ZipArchive;

// 60MB 압축 패치 데이터를 바이너리에 직접 내장하여 단일 완전 독립 실행 파일로 동작
const EMBEDDED_PATCH_ZIP: &[u8] = include_bytes!("../data/patch_data.zip");

pub struct PatchManager {
    content_dir: PathBuf,
}

impl PatchManager {
    pub fn new(content_dir: PathBuf) -> Self {
        Self { content_dir }
    }

    /// 백업 디렉터리 경로 반환
    pub fn backup_dir(&self) -> PathBuf {
        self.content_dir.join("BlackJacket_Data_Backup_Original")
    }

    /// 백업이 존재하는지 확인
    pub fn has_backup(&self) -> bool {
        let bdir = self.backup_dir();
        bdir.exists() && bdir.join("Unity.ResourceManager.dll").exists()
    }

    /// 패치가 적용되어 있는지 확인
    pub fn is_patched(&self) -> bool {
        let de_bundle = self.content_dir
            .join("BlackJacket_Data")
            .join("StreamingAssets")
            .join("aa")
            .join("StandaloneWindows64")
            .join("localization-string-tables-german(de)_assets_all.bundle");
        
        // 독일어 대체 번들이 있거나 ResourceManager dll 크기가 141,312 바이트인 경우
        if de_bundle.exists() {
            return true;
        }

        let dll_path = self.content_dir.join("BlackJacket_Data").join("Managed").join("Unity.ResourceManager.dll");
        if let Ok(meta) = fs::metadata(&dll_path) {
            if meta.len() == 141312 {
                return true;
            }
        }

        false
    }

    /// 원본 파일 백업 생성
    pub fn create_backup_if_needed(&self) -> Result<(), String> {
        let bdir = self.backup_dir();
        if self.has_backup() {
            return Ok(());
        }

        fs::create_dir_all(&bdir).map_err(|e| format!("백업 폴더 생성 실패: {}", e))?;

        let files_to_backup = [
            ("Managed/Unity.ResourceManager.dll", "Unity.ResourceManager.dll"),
            (
                "StreamingAssets/aa/StandaloneWindows64/defaultlocalgroup_assets_all_765c0a93fa74c52920d904c7be63e708.bundle",
                "defaultlocalgroup_assets_all_765c0a93fa74c52920d904c7be63e708.bundle"
            ),
            (
                "StreamingAssets/aa/StandaloneWindows64/localization-string-tables-english(en)_assets_all_c34affd770f0a3f2a1624341ea1b4aeb.bundle",
                "localization-string-tables-english(en)_assets_all_c34affd770f0a3f2a1624341ea1b4aeb.bundle"
            ),
        ];

        let data_dir = self.content_dir.join("BlackJacket_Data");
        for (rel_src, dst_name) in files_to_backup {
            let src = data_dir.join(rel_src);
            if src.exists() {
                let dst = bdir.join(dst_name);
                fs::copy(&src, &dst).map_err(|e| format!("파일 백업 실패 ({}): {}", rel_src, e))?;
            }
        }

        Ok(())
    }

    /// 패치 적용 (외부 폴더 우선 확인 후 내장 zip에서 추출)
    pub fn apply_patch(&self) -> Result<usize, String> {
        self.create_backup_if_needed()?;

        let mut extracted_count = 0;
        let reader = Cursor::new(EMBEDDED_PATCH_ZIP);
        let mut archive = ZipArchive::new(reader).map_err(|e| format!("내장 압축 데이터 해제 실패: {}", e))?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| format!("압축 엔트리 읽기 실패: {}", e))?;
            let name = file.name().to_string();

            // BlackJacket_Data 하위 파일만 추출
            if !name.starts_with("BlackJacket_Data/") && !name.starts_with("BlackJacket_Data\\") {
                continue;
            }

            let outpath = self.content_dir.join(&name);
            if file.is_dir() {
                fs::create_dir_all(&outpath).map_err(|e| format!("디렉터리 생성 실패: {}", e))?;
            } else {
                if let Some(parent) = outpath.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent).map_err(|e| format!("디렉터리 생성 실패: {}", e))?;
                    }
                }
                let mut outfile = File::create(&outpath).map_err(|e| format!("파일 생성 실패 ({}): {}", name, e))?;
                io::copy(&mut file, &mut outfile).map_err(|e| format!("파일 쓰기 실패 ({}): {}", name, e))?;
                extracted_count += 1;
            }
        }

        Ok(extracted_count)
    }

    /// 원본으로 복원 (롤백)
    pub fn restore_original(&self) -> Result<(), String> {
        if !self.has_backup() {
            return Err("복원할 원본 백업 파일이 존재하지 않습니다.".into());
        }

        let bdir = self.backup_dir();
        let data_dir = self.content_dir.join("BlackJacket_Data");

        let files_to_restore = [
            ("Unity.ResourceManager.dll", "Managed/Unity.ResourceManager.dll"),
            (
                "defaultlocalgroup_assets_all_765c0a93fa74c52920d904c7be63e708.bundle",
                "StreamingAssets/aa/StandaloneWindows64/defaultlocalgroup_assets_all_765c0a93fa74c52920d904c7be63e708.bundle"
            ),
            (
                "localization-string-tables-english(en)_assets_all_c34affd770f0a3f2a1624341ea1b4aeb.bundle",
                "StreamingAssets/aa/StandaloneWindows64/localization-string-tables-english(en)_assets_all_c34affd770f0a3f2a1624341ea1b4aeb.bundle"
            ),
        ];

        for (src_name, rel_dst) in files_to_restore {
            let src = bdir.join(src_name);
            if src.exists() {
                let dst = data_dir.join(rel_dst);
                if let Some(parent) = dst.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                fs::copy(&src, &dst).map_err(|e| format!("파일 복원 실패 ({}): {}", rel_dst, e))?;
            }
        }

        // 추가된 독일어 번들 제거
        let de_bundle = data_dir
            .join("StreamingAssets")
            .join("aa")
            .join("StandaloneWindows64")
            .join("localization-string-tables-german(de)_assets_all.bundle");
        if de_bundle.exists() {
            let _ = fs::remove_file(de_bundle);
        }

        Ok(())
    }
}
