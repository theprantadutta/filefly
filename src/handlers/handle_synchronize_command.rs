use crate::file_service::synchronize_folders::synchronize_folders;
use crate::filefly_args::SynchronizeCommand;
use crate::logger::Logger;

pub fn handle_synchronize_command(command: SynchronizeCommand) {
    let logger = Logger::new(Some(command.log_level), Some(command.no_log));

    logger.debug(&format!(
        "Synchronizing Folder {} To {}",
        command.source, command.destination
    ));

    let result: Result<(), std::io::Error> = synchronize_folders(
        &logger,
        &command.source,
        &command.destination,
        command.no_delete,
        command.no_log,
    );

    match result {
        Ok(_) => logger.success("Synchronization Successful"),
        Err(e) => {
            logger.warning("Synchronization Failed with error");
            println!("{}", e);
        }
    }
}
