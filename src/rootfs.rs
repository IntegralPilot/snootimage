use std::process::{Command, Stdio};
use std::path::PathBuf;

pub struct RootFS;

impl RootFS {
    pub fn create_image(rootfs_file_name: PathBuf) -> Result<(), String> {
        let dd_command = if cfg!(target_os = "windows") {
            format!("C:\\msys64\\usr\\bin\\{}", "dd")
        } else {
            "dd".to_string()
        };
        let mcopy_command = if cfg!(target_os = "windows") {
            format!("C:\\msys64\\bin\\{}", "mcopy")
        } else {
            "mcopy".to_string()
        };
        let mformat_command = if cfg!(target_os = "windows") {
            format!("C:\\msys64\\bin\\{}", "mformat")
        } else {
            "mformat".to_string()
        };
       // Initialize MSYS2 if not on Linux
        if !cfg!(target_os = "linux") {
            let _msys2_status = Command::new("cmd")
                    .args(&["/C", "'C:\\msys64\\usr\\bin\\pacman.exe -Syu --noconfirm mingw-w64-x86_64-mtools'"])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn();
        }

        // Create a 2 MB file
        let _dd_status = Command::new(dd_command)
            .arg("if=/dev/zero")
            .arg(format!("of={}", rootfs_file_name.to_str().unwrap()).as_str())
            .arg("bs=1K")
            .arg("count=10")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();


        // Put a FAT filesystem on it
        let _mformat_status = Command::new(mformat_command)
            .arg("-i")
            .arg(rootfs_file_name.to_str().unwrap())
            .arg("::")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();

        // Copy files from ./rootfs to the image
        let _mcopy_status = Command::new(mcopy_command)
            .arg("-i")
            .arg(rootfs_file_name.to_str().unwrap())
            .arg("-s")
            .arg("-v")
            .arg("-o")
            .arg("./rootfs")
            .arg("::")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();

        Ok(())
    }
}