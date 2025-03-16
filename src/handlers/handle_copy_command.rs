use std::io::Error;
use std::path::PathBuf;

use crate::file_service::copy_file_and_folders::{
    copy_files_with_progress, copy_single_file_with_progress,
};
use crate::filefly_args::CopyCommand;
use crate::logger::Logger;

// Function to handle the copy command
pub fn handle_copy_command(command: CopyCommand) {
    // Variable to hold the result of the copy operation
    let result: Result<(), Error>;

    let logger = Logger::new(Some(command.log_level.clone()), Some(command.no_log));
    // Check if the source path is a directory
    if PathBuf::from(&command.source).is_dir() {
        logger.debug(&format!("Given Path is a Directory"));
        logger.debug(&format!(
            "Copying Folder {} To {}",
            command.source, command.destination
        ));
        // Copy files from the source directory to the destination with progress
        result = copy_files_with_progress(
            &logger,
            &command.source,
            &command.destination,
            command.no_log,
        );
    } else {
        logger.debug(&format!("Given Path is a File"));
        logger.debug(&format!(
            "Copying File From {} To {}",
            command.source, command.destination
        ));
        // Copy a single file from the source to the destination with progress
        result = copy_single_file_with_progress(
            &logger,
            &command.source,
            &command.destination,
            command.no_log,
        );
    }

    // Handle the result of the copy operation
    match result {
        Ok(_) => {
            // Start the logger
            let logger = Logger::default();
            logger.success("Copying Successful")
        }
        Err(e) => {
            // Log an error message if copying fails and print the error details
            logger.error("Copying Failed with error");
            println!("{}", e);
        }
    }
}
