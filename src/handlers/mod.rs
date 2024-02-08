use crate::{CopyCommand, DeleteCommand};
use crate::file_service::copy_file_and_folders::copy_files_with_progress;
use crate::file_service::delete_file_and_folders::delete_folder_with_progress;
use crate::logger::Logger;

pub fn handle_copy_command(command: CopyCommand) {
    Logger.debug(&format!("Copying All Files From {} To {}", command.source, command.destination));

    let result = copy_files_with_progress(&command.source, &command.destination);

    match result {
        Ok(_) => Logger.success("Copying Successful"),
        Err(e) => {
            Logger.error("Copying Failed with error");
            println!("{}", e);
        }
    }
}

pub fn handle_delete_command(command: DeleteCommand) {
    Logger.debug(&format!("Deleting Folder {}", command.folder));

    let result = delete_folder_with_progress(&command.folder);

    match result {
        Ok(_) => Logger.success("Deletion Successful"),
        Err(e) => {
            Logger.error("Deletion Failed with error");
            println!("{}", e);
        }
    }
}