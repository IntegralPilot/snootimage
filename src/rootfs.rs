use std::process::{Command, Stdio};
use std::path::PathBuf;

pub struct RootFS;

impl RootFS {
    fn handle_command_result(result: std::io::Result<std::process::ExitStatus>, error_message: &str) -> Result<(), String> {
        match result {
            Ok(exit_status) => {
                if exit_status.code().unwrap_or(1) != 0 {
                    return Err(error_message.to_string());
                } else {
                    return Ok(());
                }
            },
            Err(_) => {
                return Err(error_message.to_string());
            }
        }
    }
    pub fn create_image(rootfs_file_name: PathBuf) -> Result<(), String> {
        #[cfg(not(target_os = "linux"))]
        return Err(String::from("RootFS::create_image() is only supported on Linux"));

        // Create a 2 MB file
        let dd_status = Command::new("dd")
            .arg("if=/dev/zero")
            .arg(format!("of={}", rootfs_file_name.to_str().unwrap()).as_str())
            .arg("bs=1K")
            .arg("count=10")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        RootFS::handle_command_result(dd_status, "Error running dd command")?;

        // Put a FAT filesystem on it
        let mformat_status = Command::new("mformat")
            .arg("-i")
            .arg(rootfs_file_name.to_str().unwrap())
            .arg("::")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        RootFS::handle_command_result(mformat_status, "Error running mformat command")?;

        // Copy files from ./rootfs to the image
        let mcopy_status = Command::new("mcopy")
            .arg("-i")
            .arg(rootfs_file_name.to_str().unwrap())
            .arg("-s")
            .arg("-v")
            .arg("-o")
            .arg("./rootfs")
            .arg("::")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        RootFS::handle_command_result(mcopy_status, "Error running mcopy command")?;

        Ok(())
    }
}