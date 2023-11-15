use std::fs::File;
use std::io::{self, Write, Read};
use std::path::PathBuf;

pub struct FinalImage;

impl FinalImage {
    pub fn create(output_path: PathBuf, boot_file_name: PathBuf, rootfs_file_name: PathBuf) -> io::Result<()> {
        // Open the input files
        let mut boot_bin = File::open(boot_file_name)?;
        let mut output_file = File::create(output_path)?;

        // Read and write boot.bin
        let mut buffer = Vec::new();
        boot_bin.read_to_end(&mut buffer)?;
        output_file.write_all(&buffer)?;

        // Create and write two zero sectors
        buffer.resize(512, 0); // assuming a sector size of 512 bytes
        output_file.write_all(&buffer)?;
        output_file.write_all(&buffer)?;

        // Create and write one FE sector (this should be 0xFE repeated 512 times)
        buffer.clear();
        buffer.resize(512, 0xFE); // fill the buffer with 0xFE
        output_file.write_all(&buffer)?;

        // Read and write rootfs.img (if you have this file)
        if let Ok(mut rootfs_img) = File::open(rootfs_file_name) {
            buffer.clear();
            rootfs_img.read_to_end(&mut buffer)?;
            output_file.write_all(&buffer)?;
        }

        Ok(())
    }
}