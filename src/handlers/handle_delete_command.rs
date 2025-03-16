use std::io::Error;
use std::path::PathBuf;

use crate::file_service::delete_file_and_folders::{
    delete_folder_with_progress, delete_single_file_with_progress,
};
use crate::filefly_args::DeleteCommand;
use crate::logger::Logger;

// Function to handle the delete command
pub fn handle_delete_command(command: DeleteCommand) {
    // Variable to hold the result of the deletion operation
    let result: Result<(), Error>;

    let logger = Logger::new(Some(command.log_level.clone()), Some(command.no_log));

    // Check if the path specified is a directory
    if PathBuf::from(&command.folder).is_dir() {
        logger.debug(&format!("Given Path is a Directory"));
        logger.debug(&format!("Deleting Folder {}", command.folder));
        // Delete the folder with progress
        result = delete_folder_with_progress(&logger, &command.folder);
    } else {
        logger.debug(&format!("Given Path is a File"));
        logger.debug(&format!("Copying File From {}", command.folder));
        // Delete the single file with progress
        result = delete_single_file_with_progress(&logger, &command.folder);
    }

    // Handle the result of the deletion operation
    match result {
        Ok(_) => {
            // Start the logger
            let logger = Logger::default();
            logger.success("Deletion Successful")
        }
        Err(e) => {
            // Log an error message if deletion fails and print the error details
            logger.error("Deletion Failed with error");
            println!("{}", e);
        }
    }
}
