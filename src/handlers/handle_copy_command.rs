use std::io::Error;
use std::path::PathBuf;

use crate::file_service::copy_file_and_folders::{
    copy_files_with_progress, copy_single_file_with_progress,
};
use crate::filefly_args::CopyCommand;
use crate::logger::Logger;

pub fn handle_copy_command(command: CopyCommand) {
    let logger = Logger::new(Some(command.log_level), Some(command.no_log));

    let result: Result<(), Error> = if PathBuf::from(&command.source).is_dir() {
        logger.debug("Given Path is a Directory");
        logger.debug(&format!(
            "Copying Folder {} To {}",
            command.source, command.destination
        ));
        copy_files_with_progress(
            &logger,
            &command.source,
            &command.destination,
            command.no_log,
        )
    } else {
        logger.debug("Given Path is a File");
        logger.debug(&format!(
            "Copying File From {} To {}",
            command.source, command.destination
        ));
        copy_single_file_with_progress(
            &logger,
            &command.source,
            &command.destination,
            command.no_log,
        )
    };

    match result {
        Ok(_) => logger.success("Copying Successful"),
        Err(e) => {
            logger.error("Copying Failed with error");
            println!("{}", e);
        }
    }
}
