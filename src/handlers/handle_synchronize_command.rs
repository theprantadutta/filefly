use crate::file_service::synchronize_folders::synchronize_folders;
use crate::filefly_args::SynchronizeCommand;
use crate::logger::Logger;

pub fn handle_synchronize_command(command: SynchronizeCommand) {
    Logger.debug(&format!(
        "Synchronizing Folder {} To {}",
        command.source, command.destination
    ));
    let result = synchronize_folders(&command.source, &command.destination);

    match result {
        Ok(_) => Logger.success("Synchronizing Successful"),
        Err(e) => {
            Logger.error("Synchronizing Failed with error");
            println!("{}", e);
        }
    }
}
