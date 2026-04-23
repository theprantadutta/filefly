use std::io::Error;
use std::path::PathBuf;

use crate::file_service::delete_file_and_folders::{
    delete_folder_with_progress, delete_single_file_with_progress,
};
use crate::filefly_args::DeleteCommand;
use crate::logger::Logger;

pub fn handle_delete_command(command: DeleteCommand) {
    let logger = Logger::new(Some(command.log_level), Some(command.no_log));

    let result: Result<(), Error> = if PathBuf::from(&command.folder).is_dir() {
        logger.debug("Given Path is a Directory");
        logger.debug(&format!("Deleting Folder {}", command.folder));
        delete_folder_with_progress(&logger, &command.folder, command.no_log)
    } else {
        logger.debug("Given Path is a File");
        logger.debug(&format!("Deleting File {}", command.folder));
        delete_single_file_with_progress(&logger, &command.folder, command.no_log)
    };

    match result {
        Ok(_) => logger.success("Deletion Successful"),
        Err(e) => {
            logger.error("Deletion Failed with error");
            println!("{}", e);
        }
    }
}
