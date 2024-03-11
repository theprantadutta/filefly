use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::logger::Logger;

use super::copy_file_and_folders::copy_single_file_with_progress;

pub fn synchronize_folders(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    // Create Destination Directory If it Doesn't Exist
    fs::create_dir_all(&dst)?;

    // Ensure source and destination are directories
    let src_path = src.as_ref();
    let dst_path = dst.as_ref();

    if !src_path.is_dir() || !dst_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source and destination must be directories",
        ));
    }

    for entry in fs::read_dir(&src_path)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let path = entry.path();
        Logger.debug(&format!(
            "Synchronizing {} with {}...",
            path.to_str().unwrap(),
            dst_path.to_str().unwrap()
        ));

        match relative_path_without_prefix(src.as_ref(), path.as_path()) {
            Some(relative_path) => {
                let dst_path = dst_path.join(relative_path);

                if ty.is_dir() {
                    let result =
                        synchronize_folders(path.to_str().unwrap(), dst_path.to_str().unwrap());

                    match result {
                        Ok(_) => Logger.success(&format!(
                            "Synchronized {} with {} Successfully",
                            path.to_str().unwrap(),
                            dst_path.to_str().unwrap()
                        )),
                        Err(e) => {
                            Logger.error(&format!(
                                "Synchronizing Failed from {} with {} with error",
                                path.to_str().unwrap(),
                                dst_path.to_str().unwrap()
                            ));
                            println!("{}", e)
                        }
                    }
                } else {
                    if fs::metadata(&dst_path).is_ok() {
                        Logger.info("File already exists at the destination.");
                    } else {
                        let dst_parent = dst_path.parent().ok_or_else(|| {
                            std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                "Invalid destination directory",
                            )
                        })?;
                        let result = copy_single_file_with_progress(&path, &dst_parent);
                        match result {
                            Ok(_) => Logger.success(&format!(
                                "Copyied from {} to {} Successfully",
                                path.to_str().unwrap(),
                                dst_path.to_str().unwrap()
                            )),
                            Err(e) => {
                                Logger.error("Synchronizing Failed with error");
                                println!("{}", e);
                            }
                        }
                    }
                }
            }
            None => {
                println!("Paths are not related")
            }
        }
    }

    // Delete files and folders in the destination that don't exist in the source
    Logger.info("Deleting Files...");
    for entry in fs::read_dir(&dst_path)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let path = entry.path();

        if !src_path
            .join(relative_path_without_prefix(dst.as_ref(), path.as_path()).unwrap())
            .exists()
        {
            if ty.is_dir() {
                // Recursively delete the directory
                fs::remove_dir_all(&path).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to delete directory {}: {}", path.display(), e),
                    )
                })?;
                Logger.success(&format!("Deleted directory: {}", path.display()));
            } else {
                // Delete the file
                fs::remove_file(&path).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to delete file {}: {}", path.display(), e),
                    )
                })?;
                Logger.success(&format!("Deleted file: {}", path.display()));
            }
        }
    }

    Ok(())
}

fn relative_path_without_prefix(base: &Path, full_path: &Path) -> Option<PathBuf> {
    match full_path.strip_prefix(base) {
        Ok(relative_path) => Some(relative_path.to_path_buf()),
        Err(_) => None,
    }
}
