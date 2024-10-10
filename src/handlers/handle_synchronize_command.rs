use crate::file_service::synchronize_folders::synchronize_folders;
use crate::filefly_args::SynchronizeCommand;
use crate::logger::Logger;

// Function to handle the synchronize command
pub fn handle_synchronize_command(command: SynchronizeCommand) {
    // Log debug information about the synchronization
    Logger.debug(&format!(
        "Synchronizing Folder {} To {}",
        command.source, command.destination
    ));

    // Call the synchronize_folders function and handle the result
    let result = synchronize_folders(&command.source, &command.destination, &command.no_delete);

    match result {
        Ok(_) => Logger.success("Synchronization Successful"),
        Err(e) => {
            // Log an error message if synchronization fails and print the error details
            Logger.error("Synchronization Failed with error");
            println!("{}", e);
        }
    }
}
