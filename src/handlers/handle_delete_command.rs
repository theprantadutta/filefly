use std::io::Error;
use std::path::PathBuf;

use crate::file_service::delete_file_and_folders::{
    delete_folder_with_progress, delete_single_file_with_progress,
};
use crate::filefly_args::DeleteCommand;
use crate::logger::Logger;

pub fn handle_delete_command(command: DeleteCommand) {
    let result: Result<(), Error>;

    if PathBuf::from(&command.folder).is_dir() {
        Logger.debug(&format!("Given Path is a Directory"));
        Logger.debug(&format!("Deleting Folder {}", command.folder));
        result = delete_folder_with_progress(&command.folder);
    } else {
        Logger.debug(&format!("Given Path is a File"));
        Logger.debug(&format!("Copying File From {}", command.folder));
        result = delete_single_file_with_progress(&command.folder);
    }

    match result {
        Ok(_) => Logger.success("Deletion Successful"),
        Err(e) => {
            Logger.error("Deletion Failed with error");
            println!("{}", e);
        }
    }
}
