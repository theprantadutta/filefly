use std::{io, path::Path};

use crate::logger::Logger;

use super::{
    copy_file_and_folders::{copy_files_with_progress, copy_single_file_with_progress},
    delete_file_and_folders::{delete_folder_with_progress, delete_single_file_with_progress},
};

pub fn replace_files_with_progress(
    logger: &Logger,
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    no_log: bool,
) -> io::Result<()> {
    match copy_files_with_progress(logger, &src, &dst, no_log) {
        Ok(_) => {
            logger.success("Copying Successful");
            match delete_folder_with_progress(logger, &src, no_log) {
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

pub fn replace_single_file_with_progress(
    logger: &Logger,
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    no_log: bool,
) -> io::Result<()> {
    match copy_single_file_with_progress(logger, &src, &dst, no_log) {
        Ok(_) => {
            logger.success("Copying Successful");
            match delete_single_file_with_progress(logger, &src, no_log) {
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
