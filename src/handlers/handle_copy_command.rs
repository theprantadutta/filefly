use crate::file_service::copy_file_and_folders::copy_files_with_progress;
use crate::filefly_args::CopyCommand;
use crate::logger::Logger;

pub fn handle_copy_command(command: CopyCommand) {
    Logger.debug(&format!("Copying All Files From {} To {}", command.source, command.destination));

    let result = copy_files_with_progress(&command.source, &command.destination);
    // let result = copy_file(&command.source, &command.destination);

    match result {
        Ok(_) => Logger.success("Copying Successful"),
        Err(e) => {
            Logger.error("Copying Failed with error");
            println!("{}", e);
        }
    }
}