use std::io::Error;
use std::path::PathBuf;

use crate::file_service::replace_file_and_folders::{
    replace_files_with_progress, replace_single_file_with_progress,
};
use crate::filefly_args::ReplaceCommand;
use crate::logger::Logger;

// Function to handle the replace command
pub fn handle_replace_command(command: ReplaceCommand) {
    // Variable to hold the result of the replacement operation
    let result: Result<(), Error>;

    // Check if the source path is a directory
    if PathBuf::from(&command.source).is_dir() {
        // Log debug information about the source being a directory
        Logger.debug(&format!("Given Path is a Directory"));
        Logger.debug(&format!(
            "Replacing Folder {} To {}",
            command.source, command.destination
        ));
        // Replace files in the source directory with progress
        result = replace_files_with_progress(&command.source, &command.destination);
    } else {
        // Log debug information about the source being a file
        Logger.debug(&format!("Given Path is a File"));
        Logger.debug(&format!(
            "Replacing File From {} To {}",
            command.source, command.destination
        ));
        // Replace a single file with progress
        result = replace_single_file_with_progress(&command.source, &command.destination);
    }

    // Handle the result of the replacement operation
    match result {
        Ok(_) => Logger.success("Replacing Successful"),
        Err(e) => {
            // Log an error message if replacement fails and print the error details
            Logger.error("Replacing Failed with error");
            println!("{}", e);
        }
    }
}
