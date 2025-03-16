use std::{io, path::Path};

use crate::logger::Logger;

use super::{
    copy_file_and_folders::{copy_files_with_progress, copy_single_file_with_progress},
    delete_file_and_folders::{delete_folder_with_progress, delete_single_file_with_progress},
};

// Function to replace files with progress
pub fn replace_files_with_progress(
    logger: &Logger,
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    no_log: bool,
) -> io::Result<()> {
    // Copy files from source to destination
    match copy_files_with_progress(&logger, &src, &dst, no_log) {
        Ok(_) => {
            logger.success("Copying Successful");
            // Only delete the source folder if copying was successful
            match delete_folder_with_progress(&logger, &src) {
                Ok(_) => {
                    logger.success("Deleting Successful");
                    Ok(())
                }
                Err(e) => {
                    logger.error("Deleting Failed with error");
                    println!("{}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            logger.error("Copying Failed with error");
            println!("{}", e);
            Err(e)
        }
    }
}

// Function to replace a single file with progress
pub fn replace_single_file_with_progress(
    logger: &Logger,
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    no_log: bool,
) -> io::Result<()> {
    // Copy single file from source to destination
    let cp_result = copy_single_file_with_progress(&logger, &src, &dst, no_log);
    // Delete source file
    let del_result = delete_single_file_with_progress(&logger, &src);

    match cp_result {
        Ok(_) => {
            logger.success("Copying Successful");
            match del_result {
                Ok(_) => {
                    logger.success("Deleting Successful");
                    // Return OK result if everything succeeds
                    Ok(())
                }
                Err(e) => {
                    logger.error("Deleting Failed with error");
                    println!("{}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            logger.error("Copying Failed with error");
            println!("{}", e);
            Err(e)
        }
    }
}
