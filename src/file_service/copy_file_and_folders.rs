use indicatif::{ProgressBar, ProgressStyle};
use std::io::{Read, Write};
use std::path::Path;
use std::{fs, io};

use crate::logger::Logger;

const BUFFER_SIZE: usize = 8192;

// Function to copy files with progress from source to destination directory
pub fn copy_files_with_progress(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    // Create destination directory if it does not exist
    fs::create_dir_all(&dst)?;

    // Iterate through the entries in the source directory
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        // Check if entry is a directory
        if ty.is_dir() {
            // Recursively call the function for subdirectories
            copy_files_with_progress(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            // Log information about the file being copied
            Logger.info(&format!(
                "Copying Files From {}",
                entry.file_name().to_str().unwrap()
            ));

            // Get file length for progress bar
            let file_len = entry.metadata()?.len();

            // Initialize progress bar
            let pb = ProgressBar::new(file_len);

            pb.set_style(
                ProgressStyle::with_template(
                    "{spinner:.cyan} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})"
                )
                .unwrap()
                .progress_chars("#>-"),
            );

            // Open source file for reading
            let mut src_file = fs::File::open(&entry.path())?;

            // Create destination file for writing
            let mut dst_file = fs::File::create(dst.as_ref().join(entry.file_name()))?;

            // Initialize buffer for file copy
            let mut buffer = [0u8; BUFFER_SIZE];

            // Main loop for file copy
            loop {
                match src_file.read(&mut buffer) {
                    Ok(0) => break, // Reached the end of the file
                    Ok(bytes_read) => {
                        // Write buffer to destination file
                        dst_file.write_all(&buffer[..bytes_read])?;

                        // Increment progress bar
                        pb.inc(bytes_read as u64);
                    }
                    Err(err) => {
                        // Handle error during file read
                        eprintln!("Error reading file: {}", err);
                        pb.finish_with_message("error");
                        return Err(err);
                    }
                }
            }

            // Finish progress bar with "done" message
            pb.finish_with_message("done");
        }
    }

    // Return OK result if everything succeeds
    Ok(())
}

// Function to copy a single file with progress from source to destination
pub fn copy_single_file_with_progress(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
) -> io::Result<()> {
    // Ensure src is a file
    let src_path = src.as_ref();
    if !src_path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source is not a file",
        ));
    }

    // Create destination directory if it does not exist
    fs::create_dir_all(&dst)?;

    // Log information about the file being copied
    Logger.info(&format!(
        "Copying File: {}",
        src_path.file_name().unwrap().to_str().unwrap()
    ));

    // Get file length for progress bar
    let file_len = src_path.metadata()?.len();

    // Initialize progress bar
    let pb = ProgressBar::new(file_len);

    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.cyan} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})"
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    // Open source file for reading
    let mut src_file = fs::File::open(src_path)?;

    // Create destination file for writing
    let mut dst_file = fs::File::create(dst.as_ref().join(src_path.file_name().unwrap()))?;

    // Initialize buffer for file copy
    let mut buffer = [0u8; BUFFER_SIZE];

    // Main loop for file copy
    loop {
        match src_file.read(&mut buffer) {
            Ok(0) => break, // Reached the end of the file
            Ok(bytes_read) => {
                // Write buffer to destination file
                dst_file.write_all(&buffer[..bytes_read])?;

                // Increment progress bar
                pb.inc(bytes_read as u64);
            }
            Err(err) => {
                // Handle error during file read
                eprintln!("Error reading file: {}", err);
                pb.finish_with_message("error");
                return Err(err);
            }
        }
    }

    // Finish progress bar with "done" message
    pb.finish_with_message("done");

    Ok(())
}
