use crate::file_service::delete_file_and_folders::delete_folder_with_progress;
use crate::filefly_args::DeleteCommand;
use crate::logger::Logger;

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