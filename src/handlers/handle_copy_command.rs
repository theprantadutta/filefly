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

    // Check if the source path is a directory
    if PathBuf::from(&command.source).is_dir() {
        Logger.debug(&format!("Given Path is a Directory"));
        Logger.debug(&format!(
            "Copying Folder {} To {}",
            command.source, command.destination
        ));
        // Copy files from the source directory to the destination with progress
        result = copy_files_with_progress(&command.source, &command.destination);
    } else {
        Logger.debug(&format!("Given Path is a File"));
        Logger.debug(&format!(
            "Copying File From {} To {}",
            command.source, command.destination
        ));
        // Copy a single file from the source to the destination with progress
        result = copy_single_file_with_progress(&command.source, &command.destination);
    }

    // Handle the result of the copy operation
    match result {
        Ok(_) => Logger.success("Copying Successful"),
        Err(e) => {
            // Log an error message if copying fails and print the error details
            Logger.error("Copying Failed with error");
            println!("{}", e);
        }
    }
}
