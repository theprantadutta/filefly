use crate::file_service::synchronize_folders::synchronize_folders;
use crate::filefly_args::SynchronizeCommand;
use crate::logger::Logger;

// Function to handle the synchronize command
pub fn handle_synchronize_command(command: SynchronizeCommand) {
    // Start the logger
    let logger = Logger::default();

    // Log debug information about the synchronization
    logger.debug(&format!(
        "Synchronizing Folder {} To {}",
        command.source, command.destination
    ));

    let operation_logger = Logger::new(Some(command.log_level.clone()), Some(command.no_log));
    // Call the synchronize_folders function and handle the result
    let result: Result<(), std::io::Error> = synchronize_folders(
        &operation_logger,
        &command.source,
        &command.destination,
        &command.no_delete,
        command.no_log,
    );

    match result {
        Ok(_) => logger.success("Synchronization Successful"),
        Err(e) => {
            // Log an error message if synchronization fails and print the error details
            logger.warning("Synchronization Failed with error");
            println!("{}", e);
        }
    }
}
