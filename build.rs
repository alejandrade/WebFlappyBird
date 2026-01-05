use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Create symlink to assets in target directory
    let target_dir = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let assets_src = Path::new("assets");
    let assets_dst = target_dir.join("assets");

    // Remove existing symlink/dir if it exists
    let _ = fs::remove_dir_all(&assets_dst);

    // Try to create symlink, fall back to copying if that fails
    #[cfg(unix)]
    {
        if std::os::unix::fs::symlink(assets_src, &assets_dst).is_err() {
            copy_dir_all(assets_src, &assets_dst).ok();
        }
    }

    #[cfg(not(unix))]
    {
        copy_dir_all(assets_src, &assets_dst).ok();
    }

    println!("cargo:rerun-if-changed=assets/");
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst_path)?;
        } else {
            fs::copy(entry.path(), dst_path)?;
        }
    }
    Ok(())
}
