use std::io::Error;
use std::path::PathBuf;

use crate::file_service::replace_file_and_folders::{
    replace_files_with_progress, replace_single_file_with_progress,
};
use crate::filefly_args::ReplaceCommand;
use crate::logger::Logger;

pub fn handle_replace_command(command: ReplaceCommand) {
    let result: Result<(), Error>;

    if PathBuf::from(&command.source).is_dir() {
        Logger.debug(&format!("Given Path is a Directory"));
        Logger.debug(&format!(
            "Replacing Folder {} To {}",
            command.source, command.destination
        ));
        result = replace_files_with_progress(&command.source, &command.destination);
    } else {
        Logger.debug(&format!("Given Path is a File"));
        Logger.debug(&format!(
            "Replacing File From {} To {}",
            command.source, command.destination
        ));
        result = replace_single_file_with_progress(&command.source, &command.destination);
    }

    match result {
        Ok(_) => Logger.success("Replacing Successful"),
        Err(e) => {
            Logger.error("Replacing Failed with error");
            println!("{}", e);
        }
    }
}
