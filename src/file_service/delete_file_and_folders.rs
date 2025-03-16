use crate::logger::Logger;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::{fs, io};

// Function to delete a folder with progress
pub fn delete_folder_with_progress(
    logger: &Logger,
    folder_path: impl AsRef<Path>,
) -> io::Result<()> {
    // Check if the folder exists
    if !folder_path.as_ref().exists() {
        return Err(Error::new(ErrorKind::NotFound, "Folder not found"));
    }

    // Get total size of the folder for progress bar
    let total_size = calculate_folder_size(&folder_path)?;

    // Initialize progress bar
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.cyan} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})"
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    // Delete folder recursively
    delete_folder_recursive_with_progress(&logger, &folder_path, &pb)?;

    // Finish progress bar with "done" message
    pb.finish_with_message("done");

    Ok(())
}

// Recursive function to delete a folder with progress
fn delete_folder_recursive_with_progress(
    logger: &Logger,
    folder_path: impl AsRef<Path>,
    pb: &ProgressBar,
) -> io::Result<()> {
    // Iterate through the entries in the folder
    for entry in fs::read_dir(&folder_path)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        // Check if entry is a directory
        if ty.is_dir() {
            // Recursively call the function for subdirectories
            delete_folder_recursive_with_progress(&logger, entry.path(), pb)?;
        } else {
            // Log information about the file being deleted
            logger.info(&format!(
                "Deleting File {}",
                entry.file_name().to_str().unwrap()
            ));

            // Get file length for progress bar
            let file_len = entry.metadata()?.len();

            // Increment progress bar
            pb.inc(file_len);

            // Delete file
            fs::remove_file(entry.path())?;
        }
    }

    // Delete the empty folder
    fs::remove_dir(&folder_path)?;

    Ok(())
}

// Function to delete a single file with progress
pub fn delete_single_file_with_progress(
    logger: &Logger,
    file_path: impl AsRef<Path>,
) -> io::Result<()> {
    // Check if the file exists
    if !file_path.as_ref().exists() {
        return Err(Error::new(ErrorKind::NotFound, "File not found"));
    }

    // Get file length for progress bar
    let file_len = file_path.as_ref().metadata()?.len();

    // Initialize progress bar
    let pb = ProgressBar::new(file_len);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.cyan} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})"
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    // Log information about the file being deleted
    logger.info(&format!(
        "Deleting File: {}",
        file_path.as_ref().file_name().unwrap().to_str().unwrap()
    ));

    // Increment progress bar
    pb.inc(file_len);

    // Delete file
    fs::remove_file(file_path)?;

    // Finish progress bar with "done" message
    pb.finish_with_message("done");

    Ok(())
}

// Function to calculate the size of a folder
fn calculate_folder_size(folder_path: impl AsRef<Path>) -> io::Result<u64> {
    let mut total_size = 0;

    for entry in fs::read_dir(&folder_path)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        if ty.is_dir() {
            // Recursively calculate size of subdirectories
            total_size += calculate_folder_size(entry.path())?;
        } else {
            // Add file size to total size
            total_size += entry.metadata()?.len();
        }
    }

    Ok(total_size)
}
