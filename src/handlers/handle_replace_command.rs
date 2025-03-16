use std::io::Error;
use std::path::PathBuf;

use crate::file_service::replace_file_and_folders::{
    replace_files_with_progress, replace_single_file_with_progress,
};
use crate::filefly_args::ReplaceCommand;
use crate::logger::Logger;

// Function to handle the replace command
pub fn handle_replace_command(command: ReplaceCommand) {
    let logger = Logger::new(Some(command.log_level.clone()), Some(command.no_log));
    // Variable to hold the result of the replacement operation
    let result: Result<(), Error>;

    // Check if the source path is a directory
    if PathBuf::from(&command.source).is_dir() {
        // Log debug information about the source being a directory
        logger.debug(&format!("Given Path is a Directory"));
        logger.debug(&format!(
            "Replacing Folder {} To {}",
            command.source, command.destination
        ));
        // Replace files in the source directory with progress
        result = replace_files_with_progress(
            &logger,
            &command.source,
            &command.destination,
            command.no_log,
        );
    } else {
        // Log debug information about the source being a file
        logger.debug(&format!("Given Path is a File"));
        logger.debug(&format!(
            "Replacing File From {} To {}",
            command.source, command.destination
        ));
        // Replace a single file with progress
        result = replace_single_file_with_progress(
            &logger,
            &command.source,
            &command.destination,
            command.no_log,
        );
    }

    // Handle the result of the replacement operation
    match result {
        Ok(_) => {
            // Start the logger
            let logger = Logger::default();
            logger.success("Replacing Successful")
        }
        Err(e) => {
            // Log an error message if replacement fails and print the error details
            logger.error("Replacing Failed with error");
            println!("{}", e);
        }
    }
}
