use std::io::Error;
use std::path::PathBuf;

use crate::file_service::replace_file_and_folders::{
    replace_files_with_progress, replace_single_file_with_progress,
};
use crate::filefly_args::ReplaceCommand;
use crate::logger::Logger;

pub fn handle_replace_command(command: ReplaceCommand) {
    let logger = Logger::new(Some(command.log_level), Some(command.no_log));

    let result: Result<(), Error> = if PathBuf::from(&command.source).is_dir() {
        logger.debug("Given Path is a Directory");
        logger.debug(&format!(
            "Replacing Folder {} To {}",
            command.source, command.destination
        ));
        replace_files_with_progress(
            &logger,
            &command.source,
            &command.destination,
            command.no_log,
        )
    } else {
        logger.debug("Given Path is a File");
        logger.debug(&format!(
            "Replacing File From {} To {}",
            command.source, command.destination
        ));
        replace_single_file_with_progress(
            &logger,
            &command.source,
            &command.destination,
            command.no_log,
        )
    };

    match result {
        Ok(_) => logger.success("Replacing Successful"),
        Err(e) => {
            logger.error("Replacing Failed with error");
            println!("{}", e);
        }
    }
}
