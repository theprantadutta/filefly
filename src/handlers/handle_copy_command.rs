use std::io::Error;
use std::path::PathBuf;

use crate::file_service::copy_file_and_folders::{
    copy_files_with_progress, copy_single_file_with_progress,
};
use crate::filefly_args::CopyCommand;
use crate::logger::Logger;

pub fn handle_copy_command(command: CopyCommand) {
    let result: Result<(), Error>;

    if PathBuf::from(&command.source).is_dir() {
        Logger.debug(&format!("Given Path is a Directory"));
        Logger.debug(&format!(
            "Copying Folder {} To {}",
            command.source, command.destination
        ));
        result = copy_files_with_progress(&command.source, &command.destination);
    } else {
        Logger.debug(&format!("Given Path is a File"));
        Logger.debug(&format!(
            "Copying File From {} To {}",
            command.source, command.destination
        ));
        result = copy_single_file_with_progress(&command.source, &command.destination);
    }

    match result {
        Ok(_) => Logger.success("Copying Successful"),
        Err(e) => {
            Logger.error("Copying Failed with error");
            println!("{}", e);
        }
    }
}
