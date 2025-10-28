use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    // 当 resources 目录变化时，重新运行 build 脚本
    println!("cargo:rerun-if-changed=resources");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let resources_dir = manifest_dir.join("resources");

    // 若项目中不存在 resources 目录，静默退出（不影响构建）
    if !resources_dir.exists() {
        println!("cargo:warning=resources directory not found: {}", resources_dir.display());
        return;
    }

    // 计算 target/<profile> 目录（从 OUT_DIR 向上找到 target 目录）
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

    let profile_dir = find_profile_dir(&out_dir, &profile)
        .unwrap_or_else(|| manifest_dir.join("target").join(&profile));
    let dest_dir = profile_dir.join("resources");

    // 清理旧的 resources
    if dest_dir.exists() {
        if let Err(e) = fs::remove_dir_all(&dest_dir) {
            println!(
                "cargo:warning=failed to remove old resources at {}: {}",
                dest_dir.display(),
                e
            );
        }
    }

    // 复制新的 resources
    if let Err(e) = copy_dir_recursive(&resources_dir, &dest_dir) {
        println!(
            "cargo:warning=failed to copy resources {} -> {}: {}",
            resources_dir.display(),
            dest_dir.display(),
            e
        );
    } else {
        println!(
            "cargo:warning=resources copied: {} -> {}",
            resources_dir.display(),
            dest_dir.display()
        );
    }
}

fn find_profile_dir(out_dir: &Path, profile: &str) -> Option<PathBuf> {
    // 在 OUT_DIR 的祖先路径中寻找名为 <profile> 的目录，然后取其父目录作为构建产物根
    for ancestor in out_dir.ancestors() {
        if ancestor.file_name().and_then(|s| s.to_str()) == Some(profile) {
            return Some(ancestor.to_path_buf());
        }
    }
    None
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else if file_type.is_file() {
            // 确保父目录存在
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&from, &to)?;
        }
    }

    Ok(())
}
