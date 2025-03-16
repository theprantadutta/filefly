use clap::Parser;
use filefly_args::FileFlyArgs;
use handlers::{
    handle_copy_command::handle_copy_command, handle_delete_command::handle_delete_command,
    handle_replace_command::handle_replace_command,
    handle_synchronize_command::handle_synchronize_command,
    handle_upgrade_command::handle_upgrade_command,
};
use logger::Logger;

mod file_service;
mod filefly_args;
mod handlers;
mod logger;

fn main() {
    // Parse command line arguments
    let args = FileFlyArgs::parse();

    let logger = Logger::default();

    // Record the start time
    let start_time = std::time::Instant::now();

    // Handle different commands based on the parsed arguments
    match args {
        FileFlyArgs::Copy(command) => handle_copy_command(command),
        FileFlyArgs::Delete(command) => handle_delete_command(command),
        FileFlyArgs::Replace(command) => handle_replace_command(command),
        FileFlyArgs::Synchronize(command) => handle_synchronize_command(command),
        FileFlyArgs::Upgrade(command) => {
            if let Err(e) = handle_upgrade_command(command) {
                logger.error(&format!("Upgrade failed: {}", e));
            }
        }
    }

    // Calculate and print the elapsed time
    let elapsed_time = start_time.elapsed();
    logger.info(&format!(
        "Time taken: {:.2} seconds ({:.2} milliseconds)",
        elapsed_time.as_secs_f64(),
        elapsed_time.as_millis() as f64
    ));
}
