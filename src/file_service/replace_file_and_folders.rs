use std::{io, path::Path};

use crate::logger::Logger;

use super::{
    copy_file_and_folders::{copy_files_with_progress, copy_single_file_with_progress},
    delete_file_and_folders::{delete_folder_with_progress, delete_single_file_with_progress},
};

pub fn replace_files_with_progress(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    let cp_result = copy_files_with_progress(&src, dst);
    let del_result = delete_folder_with_progress(&src);

    match cp_result {
        Ok(_) => {
            Logger.success("Copying Successful");
            match del_result {
                Ok(_) => {
                    Logger.success("Deleting Successful");
                    // RETURN OK RESULT IF EVERYTHING SUCCEEDS
                    Ok(())
                }
                Err(e) => {
                    Logger.error("Deleting Failed with error");
                    println!("{}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            Logger.error("Copying Failed with error");
            println!("{}", e);
            Err(e)
        }
    }
}

pub fn replace_single_file_with_progress(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
) -> io::Result<()> {
    let cp_result = copy_single_file_with_progress(&src, &dst);
    let del_result = delete_single_file_with_progress(&src);

    match cp_result {
        Ok(_) => {
            Logger.success("Copying Successful");
            match del_result {
                Ok(_) => {
                    Logger.success("Deleting Successful");
                    // RETURN OK RESULT IF EVERYTHING SUCCEEDS
                    Ok(())
                }
                Err(e) => {
                    Logger.error("Deleting Failed with error");
                    println!("{}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            Logger.error("Copying Failed with error");
            println!("{}", e);
            Err(e)
        }
    }
}
