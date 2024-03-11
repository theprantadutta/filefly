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

    // Check if the path specified is a directory
    if PathBuf::from(&command.folder).is_dir() {
        Logger.debug(&format!("Given Path is a Directory"));
        Logger.debug(&format!("Deleting Folder {}", command.folder));
        // Delete the folder with progress
        result = delete_folder_with_progress(&command.folder);
    } else {
        Logger.debug(&format!("Given Path is a File"));
        Logger.debug(&format!("Copying File From {}", command.folder));
        // Delete the single file with progress
        result = delete_single_file_with_progress(&command.folder);
    }

    // Handle the result of the deletion operation
    match result {
        Ok(_) => Logger.success("Deletion Successful"),
        Err(e) => {
            // Log an error message if deletion fails and print the error details
            Logger.error("Deletion Failed with error");
            println!("{}", e);
        }
    }
}
