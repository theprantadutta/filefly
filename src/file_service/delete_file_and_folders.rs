use crate::logger::Logger;
use crate::progress_style::delete_style;
use indicatif::{ProgressBar, ProgressDrawTarget};
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::time::Duration;
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
        let pb = ProgressBar::with_draw_target(Some(total_size), ProgressDrawTarget::stderr());
        pb.set_style(delete_style());
        pb.set_prefix(format!(
            "\u{2716} {}",
            folder_path.as_ref().display()
        ));
        pb.enable_steady_tick(Duration::from_millis(90));
        Some(pb)
    } else {
        None
    };

    delete_folder_recursive_with_progress(logger, &folder_path, pb.as_ref())?;

    if let Some(pb) = pb {
        pb.finish_with_message("done");
    }

    Ok(())
}

fn delete_folder_recursive_with_progress(
    logger: &Logger,
    folder_path: impl AsRef<Path>,
    pb: Option<&ProgressBar>,
) -> io::Result<()> {
    for entry in fs::read_dir(&folder_path)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        if ty.is_dir() {
            delete_folder_recursive_with_progress(logger, entry.path(), pb)?;
        } else {
            let file_len = entry.metadata()?.len();

            logger.debug(&format!(
                "remove {} ({} bytes)",
                entry.file_name().to_string_lossy(),
                file_len
            ));

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

    logger.debug(&format!(
        "remove {} ({} bytes)",
        file_path.as_ref().display(),
        file_len
    ));

    let pb = if !no_log {
        let pb = ProgressBar::with_draw_target(Some(file_len), ProgressDrawTarget::stderr());
        pb.set_style(delete_style());
        let label = file_path
            .as_ref()
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| file_path.as_ref().display().to_string());
        pb.set_prefix(format!("\u{2716} {}", label));
        pb.enable_steady_tick(Duration::from_millis(90));
        Some(pb)
    } else {
        None
    };

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
