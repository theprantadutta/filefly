use crate::logger::Logger;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::{fs, io};

pub fn delete_folder_with_progress(
    logger: &Logger,
    folder_path: impl AsRef<Path>,
    no_log: bool,
) -> io::Result<()> {
    if !folder_path.as_ref().exists() {
        return Err(Error::new(ErrorKind::NotFound, "Folder not found"));
    }

    let total_size = calculate_folder_size(&folder_path)?;

    let pb = if !no_log {
        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.cyan} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})"
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        Some(pb)
    } else {
        None
    };

    delete_folder_recursive_with_progress(logger, &folder_path, pb.as_ref(), no_log)?;

    if let Some(pb) = pb {
        pb.finish_with_message("done");
    }

    Ok(())
}

fn delete_folder_recursive_with_progress(
    logger: &Logger,
    folder_path: impl AsRef<Path>,
    pb: Option<&ProgressBar>,
    no_log: bool,
) -> io::Result<()> {
    for entry in fs::read_dir(&folder_path)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        if ty.is_dir() {
            delete_folder_recursive_with_progress(logger, entry.path(), pb, no_log)?;
        } else {
            if !no_log {
                logger.info(&format!(
                    "Deleting File {}",
                    entry.file_name().to_string_lossy()
                ));
            }

            let file_len = entry.metadata()?.len();

            fs::remove_file(entry.path())?;

            if let Some(pb) = pb {
                pb.inc(file_len);
            }
        }
    }

    fs::remove_dir(&folder_path)?;

    Ok(())
}

pub fn delete_single_file_with_progress(
    logger: &Logger,
    file_path: impl AsRef<Path>,
    no_log: bool,
) -> io::Result<()> {
    if !file_path.as_ref().exists() {
        return Err(Error::new(ErrorKind::NotFound, "File not found"));
    }

    let file_len = file_path.as_ref().metadata()?.len();

    let pb = if !no_log {
        let pb = ProgressBar::new(file_len);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.cyan} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})"
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        Some(pb)
    } else {
        None
    };

    if !no_log {
        if let Some(name) = file_path.as_ref().file_name() {
            logger.info(&format!("Deleting File: {}", name.to_string_lossy()));
        }
    }

    fs::remove_file(&file_path)?;

    if let Some(pb) = pb {
        pb.inc(file_len);
        pb.finish_with_message("done");
    }

    Ok(())
}

fn calculate_folder_size(folder_path: impl AsRef<Path>) -> io::Result<u64> {
    let mut total_size = 0;

    for entry in fs::read_dir(&folder_path)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        if ty.is_dir() {
            total_size += calculate_folder_size(entry.path())?;
        } else {
            total_size += entry.metadata()?.len();
        }
    }

    Ok(total_size)
}
