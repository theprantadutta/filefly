use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{fs, io};

use crate::logger::Logger;

use super::copy_file_and_folders::copy_single_file_with_progress;

pub fn synchronize_folders(
    logger: &Logger,
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    no_delete: bool,
    no_log: bool,
) -> io::Result<()> {
    fs::create_dir_all(&dst)?;

    let src_path = src.as_ref();
    let dst_path = dst.as_ref();

    if !src_path.is_dir() || !dst_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source and destination must be directories",
        ));
    }

    for entry in fs::read_dir(src_path)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let path = entry.path();

        logger.debug(&format!(
            "Synchronizing {} with {}...",
            path.display(),
            dst_path.display()
        ));

        let relative_path = match relative_path_without_prefix(src_path, path.as_path()) {
            Some(p) => p,
            None => {
                logger.warning(&format!(
                    "Skipping {}: not relative to source",
                    path.display()
                ));
                continue;
            }
        };
        let dst_entry_path = dst_path.join(&relative_path);

        if ty.is_dir() {
            let result = synchronize_folders(logger, &path, &dst_entry_path, no_delete, no_log);

            match result {
                Ok(_) => logger.success(&format!(
                    "Synchronized {} with {} Successfully",
                    path.display(),
                    dst_entry_path.display()
                )),
                Err(e) => {
                    logger.error(&format!(
                        "Synchronizing Failed from {} with {} with error",
                        path.display(),
                        dst_entry_path.display()
                    ));
                    println!("{}", e);
                }
            }
        } else if dst_entry_path.exists() {
            if needs_update(&path, &dst_entry_path)? {
                if !no_log {
                    logger.info(&format!("Updating {}", dst_entry_path.display()));
                }
                let dst_parent = dst_entry_path.parent().ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Invalid destination directory",
                    )
                })?;
                match copy_single_file_with_progress(logger, &path, dst_parent, no_log) {
                    Ok(_) => logger.success(&format!(
                        "Updated {} from {} Successfully",
                        dst_entry_path.display(),
                        path.display()
                    )),
                    Err(e) => {
                        logger.error("Synchronizing Failed with error");
                        println!("{}", e);
                    }
                }
            } else if !no_log {
                logger.info(&format!("Up-to-date: {}", dst_entry_path.display()));
            }
        } else {
            let dst_parent = dst_entry_path.parent().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid destination directory",
                )
            })?;

            match copy_single_file_with_progress(logger, &path, dst_parent, no_log) {
                Ok(_) => logger.success(&format!(
                    "Copied from {} to {} Successfully",
                    path.display(),
                    dst_entry_path.display()
                )),
                Err(e) => {
                    logger.error("Synchronizing Failed with error");
                    println!("{}", e);
                }
            }
        }
    }

    if no_delete {
        if !no_log {
            logger.info("Skipping File Deletion...");
        }
        return Ok(());
    }

    if !no_log {
        logger.info("Deleting Files...");
    }
    for entry in fs::read_dir(dst_path)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let path = entry.path();

        let relative_path = match relative_path_without_prefix(dst_path, path.as_path()) {
            Some(p) => p,
            None => {
                logger.warning(&format!(
                    "Skipping {}: not relative to destination",
                    path.display()
                ));
                continue;
            }
        };

        if !src_path.join(&relative_path).exists() {
            if ty.is_dir() {
                fs::remove_dir_all(&path).map_err(|e| {
                    io::Error::other(format!(
                        "Failed to delete directory {}: {}",
                        path.display(),
                        e
                    ))
                })?;
                if !no_log {
                    logger.success(&format!("Deleted directory: {}", path.display()));
                }
            } else {
                fs::remove_file(&path).map_err(|e| {
                    io::Error::other(format!(
                        "Failed to delete file {}: {}",
                        path.display(),
                        e
                    ))
                })?;
                if !no_log {
                    logger.success(&format!("Deleted file: {}", path.display()));
                }
            }
        }
    }

    Ok(())
}

fn needs_update(src: &Path, dst: &Path) -> io::Result<bool> {
    let src_meta = fs::metadata(src)?;
    let dst_meta = fs::metadata(dst)?;

    if src_meta.len() != dst_meta.len() {
        return Ok(true);
    }

    let src_mtime = src_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let dst_mtime = dst_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);

    Ok(src_mtime > dst_mtime)
}

fn relative_path_without_prefix(base: &Path, full_path: &Path) -> Option<PathBuf> {
    full_path.strip_prefix(base).ok().map(|p| p.to_path_buf())
}
