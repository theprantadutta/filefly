use indicatif::{ProgressBar, ProgressStyle};
use std::io::{Read, Write};
use std::path::Path;
use std::{fs, io};

use crate::logger::Logger;

const BUFFER_SIZE: usize = 8192;

pub fn copy_files_with_progress(
    logger: &Logger,
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    no_log: bool,
) -> io::Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        if ty.is_dir() {
            copy_files_with_progress(
                logger,
                entry.path(),
                dst.as_ref().join(entry.file_name()),
                no_log,
            )?;
        } else {
            if !no_log {
                logger.info(&format!(
                    "Copying Files From {}",
                    entry.file_name().to_string_lossy()
                ));
            }

            let file_len = entry.metadata()?.len();

            let pb = if !no_log {
                let pb = ProgressBar::new(file_len);
                pb.set_style(
                    ProgressStyle::with_template(
                        "{spinner:.cyan} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes:>12}/{total_bytes:<12} ({eta}) {bytes_per_sec:>10} MB/s"
                    )
                    .unwrap()
                    .progress_chars("#>-"),
                );
                Some(pb)
            } else {
                None
            };

            let mut src_file = fs::File::open(entry.path())?;
            let mut dst_file = fs::File::create(dst.as_ref().join(entry.file_name()))?;
            let mut buffer = [0u8; BUFFER_SIZE];

            loop {
                match src_file.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(bytes_read) => {
                        dst_file.write_all(&buffer[..bytes_read])?;
                        if let Some(ref pb) = pb {
                            pb.inc(bytes_read as u64);
                        }
                    }
                    Err(err) => {
                        eprintln!("Error reading file: {}", err);
                        if let Some(pb) = pb {
                            pb.finish_with_message("error");
                        }
                        return Err(err);
                    }
                }
            }

            if let Some(pb) = pb {
                pb.finish_with_message("done");
            }
        }
    }

    Ok(())
}

pub fn copy_single_file_with_progress(
    logger: &Logger,
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    no_log: bool,
) -> io::Result<()> {
    let src_path = src.as_ref();
    if !src_path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source is not a file",
        ));
    }

    fs::create_dir_all(&dst)?;

    let file_name = src_path.file_name().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "Source has no file name")
    })?;

    if !no_log {
        logger.info(&format!("Copying File: {}", file_name.to_string_lossy()));
    }

    let file_len = src_path.metadata()?.len();

    let pb = if !no_log {
        let pb = ProgressBar::new(file_len);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.cyan} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes:>12}/{total_bytes:<12} ({eta}) {bytes_per_sec:>10} MB/s"
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        Some(pb)
    } else {
        None
    };

    let mut src_file = fs::File::open(src_path)?;
    let mut dst_file = fs::File::create(dst.as_ref().join(file_name))?;
    let mut buffer = [0u8; BUFFER_SIZE];

    loop {
        match src_file.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => {
                dst_file.write_all(&buffer[..bytes_read])?;
                if let Some(ref pb) = pb {
                    pb.inc(bytes_read as u64);
                }
            }
            Err(err) => {
                eprintln!("Error reading file: {}", err);
                if let Some(pb) = pb {
                    pb.finish_with_message("error");
                }
                return Err(err);
            }
        }
    }

    if let Some(pb) = pb {
        pb.finish_with_message("done");
    }

    Ok(())
}
