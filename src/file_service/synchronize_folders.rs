use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::logger::Logger;

use super::copy_file_and_folders::copy_single_file_with_progress;

// Function to synchronize folders between source and destination
pub fn synchronize_folders(
    logger: &Logger,
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    no_delete: &bool,
    no_log: bool,
) -> io::Result<()> {
    // Create destination directory if it doesn't exist
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

    // Iterate through entries in the source directory
    for entry in fs::read_dir(&src_path)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let path = entry.path();

        // Log the synchronization process for the current entry
        logger.debug(&format!(
            "Synchronizing {} with {}...",
            path.to_str().unwrap(),
            dst_path.to_str().unwrap()
        ));

        // Get the relative path of the current entry
        match relative_path_without_prefix(src.as_ref(), path.as_path()) {
            Some(relative_path) => {
                let dst_path = dst_path.join(relative_path);

                if ty.is_dir() {
                    // If the entry is a directory, recursively synchronize it
                    let result = synchronize_folders(
                        logger,
                        path.to_str().unwrap(),
                        dst_path.to_str().unwrap(),
                        no_delete,
                        no_log,
                    );

                    match result {
                        Ok(_) => logger.success(&format!(
                            "Synchronized {} with {} Successfully",
                            path.to_str().unwrap(),
                            dst_path.to_str().unwrap()
                        )),
                        Err(e) => {
                            // Log an error message if synchronization fails for the directory
                            logger.error(&format!(
                                "Synchronizing Failed from {} with {} with error",
                                path.to_str().unwrap(),
                                dst_path.to_str().unwrap()
                            ));
                            println!("{}", e);
                        }
                    }
                } else {
                    if fs::metadata(&dst_path).is_ok() {
                        // If the entry is a file and it already exists at the destination, log an info message
                        logger.info("File already exists at the destination.");
                    } else {
                        let dst_parent = dst_path.parent().ok_or_else(|| {
                            // Log an error if the destination parent directory is invalid
                            std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                "Invalid destination directory",
                            )
                        })?;

                        // Copy the single file to the destination
                        let result =
                            copy_single_file_with_progress(&logger, &path, &dst_parent, no_log);
                        match result {
                            Ok(_) => logger.success(&format!(
                                "Copied from {} to {} Successfully",
                                path.to_str().unwrap(),
                                dst_path.to_str().unwrap()
                            )),
                            Err(e) => {
                                // Log an error message if copying the file fails
                                logger.error("Synchronizing Failed with error");
                                println!("{}", e);
                            }
                        }
                    }
                }
            }
            None => {
                // Log a message if the paths are not related
                println!("Paths are not related");
            }
        }
    }

    // Check if no_delete is true
    if *no_delete {
        logger.info("Skipping File Deletion...");
        return Ok(());
    }

    // If no_delete is false, proceed with the deletion of files

    logger.info("Deleting Files...");
    for entry in fs::read_dir(&dst_path)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let path = entry.path();

        // Check if the corresponding file or folder exists in the source
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
                logger.success(&format!("Deleted directory: {}", path.display()));
            } else {
                // Delete the file
                fs::remove_file(&path).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to delete file {}: {}", path.display(), e),
                    )
                })?;
                logger.success(&format!("Deleted file: {}", path.display()));
            }
        }
    }

    Ok(())
}

// Function to get the relative path without a prefix
fn relative_path_without_prefix(base: &Path, full_path: &Path) -> Option<PathBuf> {
    match full_path.strip_prefix(base) {
        Ok(relative_path) => Some(relative_path.to_path_buf()),
        Err(_) => None,
    }
}
