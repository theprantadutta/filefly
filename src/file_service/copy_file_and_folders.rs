use indicatif::{ProgressBar, ProgressDrawTarget};
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;
use std::{fs, io};

use crate::logger::Logger;
use crate::progress_style::copy_style;

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
            let file_len = entry.metadata()?.len();

            logger.debug(&format!(
                "copy {} ({} bytes)",
                entry.file_name().to_string_lossy(),
                file_len
            ));

            let pb = if !no_log {
                let pb = ProgressBar::with_draw_target(Some(file_len), ProgressDrawTarget::stderr());
                pb.set_style(copy_style());
                pb.set_prefix(format!("\u{21AA} {}", entry.file_name().to_string_lossy()));
                pb.enable_steady_tick(Duration::from_millis(90));
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

    let file_len = src_path.metadata()?.len();

    logger.debug(&format!(
        "copy {} ({} bytes)",
        file_name.to_string_lossy(),
        file_len
    ));

    let pb = if !no_log {
        let pb = ProgressBar::with_draw_target(Some(file_len), ProgressDrawTarget::stderr());
        pb.set_style(copy_style());
        pb.set_prefix(format!("\u{21AA} {}", file_name.to_string_lossy()));
        pb.enable_steady_tick(Duration::from_millis(90));
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
